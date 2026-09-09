// LuciAPI — Luci's own unified anime API layer.
//
// Every metadata request flows through here. Providers are tried in health
// order until one answers; results are cached in-memory so repeat navigation
// is instant and third parties are hammered less.
//
//   1. Jikan (MyAnimeList)      — primary catalog
//   2. Kitsu                    — secondary catalog (own id space, offset)
//   3. Shikimori                — tertiary catalog (MAL-compatible ids)
//   4. ani.zip                  — episode metadata / mappings enrichment
//   5. AniList (GraphQL)        — optional; API currently disabled upstream,
//                                 kept wired so it heals automatically

use std::collections::HashMap;
use std::time::{Duration, Instant};

pub const JIKAN: &str = "https://api.jikan.moe/v4";
pub const KITSU: &str = "https://kitsu.app/api/edge";
pub const SHIKI: &str = "https://shikimori.one/api";
pub const ANIZIP: &str = "https://api.ani.zip/mappings";
pub const ANILIST: &str = "https://graphql.anilist.co";

pub const KITSU_ID_OFFSET: i64 = 9_000_000;

fn client() -> Result<reqwest::Client, String> {
    reqwest::Client::builder()
        .user_agent("luci-app/1.0")
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|e| e.to_string())
}

// ---------------------------------------------------------------------------
// Response cache (shared by every provider)
// ---------------------------------------------------------------------------

struct Entry {
    json: serde_json::Value,
    expires: Instant,
}

pub struct ResponseCache {
    inner: tokio::sync::Mutex<HashMap<String, Entry>>,
}

impl ResponseCache {
    pub fn new() -> Self {
        Self { inner: tokio::sync::Mutex::new(HashMap::new()) }
    }

    pub async fn get(&self, key: &str) -> Option<serde_json::Value> {
        let mut map = self.inner.lock().await;
        if let Some(e) = map.get(key) {
            if e.expires > Instant::now() {
                return Some(e.json.clone());
            }
            map.remove(key);
        }
        None
    }

    /// Drop every cached response (Settings → Providers → Clear API Cache).
    pub async fn clear(&self) {
        self.inner.lock().await.clear();
    }

    pub async fn put(&self, key: String, json: serde_json::Value, ttl: Duration) {
        let mut map = self.inner.lock().await;
        if map.len() > 512 {
            map.retain(|_, e| e.expires > Instant::now());
            if map.len() > 512 {
                let mut keys: Vec<_> = map.keys().cloned().collect();
                keys.sort();
                for k in keys.into_iter().take(256) {
                    map.remove(&k);
                }
            }
        }
        map.insert(key, Entry { json, expires: Instant::now() + ttl });
    }
}

// ---------------------------------------------------------------------------
// Raw fetch with retry (handles 429 + transient 5xx)
// ---------------------------------------------------------------------------

pub async fn http_json(url: &str, method: &str, body: Option<String>) -> Result<serde_json::Value, String> {
    let client = client()?;
    let mut backoff = Duration::from_millis(400);
    for attempt in 0..=2 {
        let req = match method {
            "POST" => client
                .post(url)
                .header("Content-Type", "application/json")
                .body(body.clone().unwrap_or_default()),
            _ => client.get(url),
        };
        // Hard per-attempt deadline so a slow provider never stalls the
        // fallback chain — a hung API must fail over, quickly.
        let resp = match tokio::time::timeout(Duration::from_secs(8), req.send()).await {
            Ok(r) => r.map_err(|e| format!("network: {e}"))?,
            Err(_) => {
                if attempt < 2 {
                    continue;
                }
                return Err("timeout".into());
            }
        };
        let status = resp.status();
        if (status.as_u16() == 429 || status.as_u16() >= 500) && attempt < 2 {
            tokio::time::sleep(backoff).await;
            backoff *= 2;
            continue;
        }
        if status.as_u16() == 404 {
            return Err("not found".into());
        }
        if !status.is_success() {
            return Err(format!("HTTP {status}"));
        }
        return resp.json().await.map_err(|e| e.to_string());
    }
    Err("unreachable".into())
}

pub async fn cached_json(
    cache: &ResponseCache,
    key: &str,
    url: &str,
    ttl: Duration,
) -> Result<serde_json::Value, String> {
    if let Some(hit) = cache.get(key).await {
        return Ok(hit);
    }
    let json = http_json(url, "GET", None).await?;
    cache.put(key.to_string(), json.clone(), ttl).await;
    Ok(json)
}

pub fn urlencode(input: &str) -> String {
    let mut out = String::with_capacity(input.len() * 2);
    for b in input.bytes() {
        if b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b == b'.' {
            out.push(b as char);
        } else if b == b' ' {
            out.push_str("%20");
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

/// Provider health probe for the Settings panel.
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderHealth {
    pub name: String,
    pub kind: String,
    pub ok: bool,
    pub latency_ms: u128,
    pub detail: String,
}

pub async fn probe_all() -> Vec<ProviderHealth> {
    let probes: Vec<(&str, &str, String, &str, Option<String>)> = vec![
        ("Jikan", "metadata", format!("{JIKAN}/anime/1"), "GET", None),
        ("Kitsu", "metadata", format!("{KITSU}/anime?page%5Blimit%5D=1"), "GET", None),
        ("Shikimori", "metadata", format!("{SHIKI}/animes?limit=1"), "GET", None),
        ("ani.zip", "episodes", format!("{ANIZIP}?mal_id=1"), "GET", None),
        ("AniList", "metadata", ANILIST.to_string(), "POST", Some(r#"{"query":"{ Media(id:1){ id } }"}"#.to_string())),
    ];
    let mut out = Vec::new();
    for (name, kind, url, method, body) in probes {
        let start = Instant::now();
        let res = http_json(&url, method, body).await;
        let ms = start.elapsed().as_millis();
        out.push(match res {
            Ok(_) => ProviderHealth { name: name.into(), kind: kind.into(), ok: true, latency_ms: ms, detail: "ok".into() },
            Err(e) => ProviderHealth { name: name.into(), kind: kind.into(), ok: false, latency_ms: ms, detail: e },
        });
    }
    out
}
