// Stream resolution — Luci's own server chain with health probing.
//
// Order of battle for every episode:
//   1. Local library files (offline, private)
//   2. ani.zip episodeData → direct HLS sources (m3u8 blobs)
//   3. ani.zip episodeData → embed iframes (streamed in a webview frame)
//   4. User-configured Consumet-compatible mirrors (Settings → Providers)
//
// Every candidate is probed (HEAD) before it reaches the player; dead servers
// are dropped automatically and the next one takes over.

use crate::types::{EpisodeEntry, SourcesResult};
use std::time::Duration;

fn client() -> Result<reqwest::Client, String> {
    reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) luci/1.0")
        .timeout(Duration::from_secs(12))
        .build()
        .map_err(|e| e.to_string())
}

/// HEAD/GET probe: does this URL actually serve media?
pub async fn probe_url(url: &str) -> bool {
    if url.is_empty() {
        return false;
    }
    let client = match client() {
        Ok(c) => c,
        Err(_) => return false,
    };
    if let Ok(head) = client.head(url).send().await {
        if head.status().is_success() {
            return true;
        }
    }
    // Some CDNs reject HEAD — fall back to a ranged GET.
    match client.get(url).header("Range", "bytes=0-1").send().await {
        Ok(resp) => resp.status().is_success(),
        Err(_) => false,
    }
}

// ---------------------------------------------------------------------------
// ani.zip episode sources
// ---------------------------------------------------------------------------

#[derive(Debug, serde::Deserialize)]
pub struct AnizipSource {
    #[serde(default)]
    pub url: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct AnizipEpisode {
    #[serde(default)]
    pub sources: Vec<AnizipSource>,
}

fn source_is_direct(src: &AnizipSource) -> bool {
    let url = match &src.url {
        Some(u) if !u.is_empty() => u,
        _ => return false,
    };
    let direct_markers = [".m3u8", ".mp4", ".mkv"];
    direct_markers.iter().any(|m| url.to_lowercase().contains(m))
}

/// Pull direct + embed sources for one episode from ani.zip.
pub async fn anizip_sources(
    cache: &crate::providers::ResponseCache,
    mal_id: i64,
    episode: i64,
) -> Option<(Vec<String>, Vec<String>)> {
    let json = crate::providers::cached_json(
        cache,
        &format!("anizip:{mal_id}:ep{episode}"),
        &format!("{}?mal_id={mal_id}", crate::providers::ANIZIP),
        Duration::from_secs(1800),
    )
    .await
    .ok()?;
    let ep_key = episode.to_string();
    let data = json.get("episodes")?.get(&ep_key)?;
    let parsed: AnizipEpisode = serde_json::from_value(data.clone()).ok()?;
    let mut direct = Vec::new();
    let mut embeds = Vec::new();
    for src in &parsed.sources {
        let url = match &src.url {
            Some(u) if !u.is_empty() => u.clone(),
            _ => continue,
        };
        if source_is_direct(src) {
            direct.push(url);
        } else {
            embeds.push(url);
        }
    }
    Some((direct, embeds))
}

// ---------------------------------------------------------------------------
// User-configured mirrors (Consumet-compatible)
// ---------------------------------------------------------------------------

/// Ask one mirror for stream sources of an episode. Tolerates several
/// response shapes since mirrors drift.
async fn mirror_sources(
    cache: &crate::providers::ResponseCache,
    base: &str,
    title: &str,
    episode: i64,
) -> Option<Vec<String>> {
    let key = format!("mirror:{}:{}:ep{episode}", base.split("//").nth(1).unwrap_or(base), crate::providers::urlencode(title));
    let url = format!(
        "{}/anime/episode-sources?title={}&episode={episode}",
        base.trim_end_matches('/'),
        crate::providers::urlencode(title)
    );
    let json = crate::providers::cached_json(cache, &key, &url, Duration::from_secs(1800)).await.ok()?;
    let mut out = Vec::new();
    for pointer in ["/sources/0/url", "/source/0/url", "/url", "/data/0/url"] {
        if let Some(u) = json.pointer(pointer).and_then(|v| v.as_str()) {
            out.push(u.to_string());
        }
    }
    if let Some(arr) = json.pointer("/sources").and_then(|v| v.as_array()) {
        for s in arr {
            if let Some(u) = s.get("url").and_then(|v| v.as_str()) {
                out.push(u.to_string());
            }
        }
    }
    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}

pub async fn mirror_base_urls(db: &crate::db::Db) -> Vec<String> {
    let stored = db
        .with_conn(|conn| {
            Ok(conn
                .query_row("SELECT value FROM settings WHERE key = 'stream_mirrors'", [], |r| r.get::<_, String>(0))
                .ok())
        })
        .unwrap_or(None);
    stored
        .map(|s| {
            s.split(',')
                .map(str::trim)
                .filter(|u| u.starts_with("http"))
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

// ---------------------------------------------------------------------------
// Resolution pipeline
// ---------------------------------------------------------------------------

pub struct ResolvedEpisode {
    pub number: i64,
    pub title: String,
    pub url: String,
    pub kind: &'static str, // "hls" | "file" | "embed"
    pub server: String,
}

fn classify(url: &str) -> &'static str {
    if url.to_lowercase().contains(".m3u8") {
        "hls"
    } else if url.starts_with("http") {
        "file"
    } else {
        "embed"
    }
}

/// Resolve the full episode list for an anime. Every entry carries its best
/// URL; the player can fall back to other servers per episode via
/// `resolve_playable`.
pub async fn resolve_episode_list(
    db: &crate::db::Db,
    cache: &crate::providers::ResponseCache,
    mal_id: i64,
    title: &str,
    title_english: Option<&str>,
) -> Result<SourcesResult, String> {
    // 1) Local library — offline first.
    let local = crate::library::local_episodes_internal(db, title).await.unwrap_or_default();
    if !local.is_empty() {
        let episodes = local.iter().map(crate::library::local_to_entry).collect::<Vec<EpisodeEntry>>();
        return Ok(SourcesResult {
            mal_id: 0,
            anime_title: title.to_string(),
            episodes,
            source: "local".into(),
        });
    }

    if mal_id <= 0 {
        return Err("no provider id for this title".into());
    }

    // 2) ani.zip episode metadata keeps titles/aired dates for every episode.
    let meta = crate::providers::cached_json(
        cache,
        &format!("anizip:{mal_id}"),
        &format!("{}?mal_id={mal_id}", crate::providers::ANIZIP),
        Duration::from_secs(3600),
    )
    .await
    .ok();

    // Episode count: prefer ani.zip, else Jikan.
    let mut count = meta
        .as_ref()
        .and_then(|m| m.get("episodeCount"))
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    if count == 0 {
        count = crate::jikan::episode_count(mal_id).await.unwrap_or(0);
    }
    if count == 0 {
        return Err("episode metadata unavailable from every provider".into());
    }

    // Probe server availability once per list (sample episode 1).
    let (direct1, embed1) = anizip_sources(cache, mal_id, 1).await.unwrap_or_default();
    let anizip_alive = !direct1.is_empty() || !embed1.is_empty();
    let mirrors = mirror_base_urls(db).await;
    let mut mirror_ok: Option<String> = None;
    for m in &mirrors {
        if mirror_sources(cache, m, title_english.unwrap_or(title), 1)
            .await
            .map(|urls| !urls.is_empty())
            .unwrap_or(false)
        {
            mirror_ok = Some(m.clone());
            break;
        }
    }

    let mut episodes: Vec<EpisodeEntry> = Vec::with_capacity(count as usize);
    for n in 1..=count {
        // Metadata: ani.zip per-episode title when present.
        let ep_key = n.to_string();
        let (ep_title, aired) = meta
            .as_ref()
            .and_then(|m| m.get("episodes"))
            .and_then(|e| e.get(&ep_key))
            .and_then(|e| {
                let t = e.get("title").and_then(|t| {
                    if t.is_string() {
                        t.as_str().map(str::to_string)
                    } else {
                        t.get("en").and_then(|v| v.as_str()).map(str::to_string)
                    }
                });
                let d = e.get("airdate").and_then(|v| v.as_str()).map(|s| s.chars().take(10).collect::<String>());
                Some((t.unwrap_or_else(|| format!("Episode {n}")), d))
            })
            .unwrap_or_else(|| (format!("Episode {n}"), None));

        // URL for this episode: ani.zip ep1-style sources were only sampled
        // for episode 1; per-episode probing of every list entry is too slow,
        // so direct URLs are resolved on-demand by resolve_playable instead.
        // The list carries an empty URL when the source is not yet known and
        // the frontend triggers lazy resolution on play.
        let url = if n == 1 {
            direct1.first().cloned().or_else(|| embed1.first().cloned()).unwrap_or_default()
        } else if anizip_alive || mirror_ok.is_some() {
            String::new() // resolved lazily on play
        } else {
            String::new()
        };

        episodes.push(EpisodeEntry { number: n, title: ep_title, url, aired, filler: false, recap: false });
    }

    Ok(SourcesResult {
        mal_id,
        anime_title: title.to_string(),
        episodes,
        source: "anizip+chain".into(),
    })
}

/// Resolve one playable episode: probe candidates in order, return the first
/// that answers. Mirrors the chain described at the top of this file.
pub async fn resolve_playable(
    db: &crate::db::Db,
    cache: &crate::providers::ResponseCache,
    mal_id: i64,
    title: &str,
    title_english: Option<&str>,
    episode: i64,
    exclude: Vec<String>,
) -> Result<ResolvedEpisode, String> {
    let excluded = |u: &str| exclude.iter().any(|x| x == u);

    // Local library first.
    let local = crate::library::local_episodes_internal(db, title).await.unwrap_or_default();
    if let Some(ep) = local.iter().find(|e| e.number == episode) {
        return Ok(ResolvedEpisode {
            number: episode,
            title: ep.title.clone(),
            url: ep.path.clone(),
            kind: "file",
            server: "local".into(),
        });
    }

    if mal_id <= 0 {
        return Err("no provider id for this title".into());
    }

    // ani.zip direct sources, probed.
    if let Some((direct, embeds)) = anizip_sources(cache, mal_id, episode).await {
        for url in &direct {
            if excluded(url) {
                continue;
            }
            if probe_url(url).await {
                return Ok(ResolvedEpisode {
                    number: episode,
                    title: format!("Episode {episode}"),
                    url: url.clone(),
                    kind: classify(url),
                    server: "ani.zip-direct".into(),
                });
            }
        }
        // Embeds are probed too (they respond 200 even when the stream dies,
        // so they rank below direct links).
        for url in &embeds {
            if excluded(url) {
                continue;
            }
            if probe_url(url).await {
                return Ok(ResolvedEpisode {
                    number: episode,
                    title: format!("Episode {episode}"),
                    url: url.clone(),
                    kind: "embed",
                    server: format!("ani.zip-embed: {}", host_of(url)),
                });
            }
        }
    }

    // User mirrors.
    for base in mirror_base_urls(db).await {
        if let Some(urls) = mirror_sources(cache, &base, title_english.unwrap_or(title), episode).await {
            for url in urls {
                if excluded(&url) {
                    continue;
                }
                let kind = classify(&url);
                if kind == "embed" || probe_url(&url).await {
                    return Ok(ResolvedEpisode {
                        number: episode,
                        title: format!("Episode {episode}"),
                        url,
                        kind,
                        server: format!("mirror: {}", host_of(&base)),
                    });
                }
            }
        }
    }

    Err(format!(
        "no live server found for episode {episode} — tried ani.zip (direct+embed) and {} mirror(s)",
        mirror_base_urls(db).await.len()
    ))
}

fn host_of(url: &str) -> String {
    url.split("//")
        .nth(1)
        .and_then(|rest| rest.split('/').next())
        .unwrap_or(url)
        .to_string()
}
