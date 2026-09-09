use crate::types::{EpisodeEntry, SourcesResult};
use std::collections::HashMap;
use std::time::Duration;

const JIKAN_API: &str = "https://api.jikan.moe/v4";

fn client() -> Result<reqwest::Client, String> {
    reqwest::Client::builder()
        .user_agent("luci-app/1.0")
        .timeout(Duration::from_secs(20))
        .build()
        .map_err(|e| e.to_string())
}

/// Fetch Jikan episodes for a MAL id (title, aired date, filler flags).
pub async fn jikan_episodes(
    mal_id: i64,
    page: i64,
) -> Result<(Vec<EpisodeEntry>, bool), String> {
    let url = format!("{JIKAN_API}/anime/{mal_id}/episodes?page={page}");
    let resp = client()?.get(&url).send().await.map_err(|e| e.to_string())?;
    if resp.status() == 404 {
        return Ok((Vec::new(), false));
    }
    if !resp.status().is_success() {
        return Err(format!("jikan returned HTTP {}", resp.status()));
    }
    let json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
    let data = json
        .get("data")
        .and_then(|d| d.as_array())
        .cloned()
        .unwrap_or_default();
    let has_next = json
        .pointer("/pagination/has_next_page")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let mut out = Vec::with_capacity(data.len());
    for ep in &data {
        out.push(EpisodeEntry {
            number: ep.get("mal_id").and_then(|v| v.as_i64()).unwrap_or(0),
            title: ep
                .get("title")
                .and_then(|v| v.as_str())
                .unwrap_or("Episode")
                .to_string(),
            url: String::new(),
            aired: ep
                .get("aired")
                .and_then(|v| v.as_str())
                .map(|s| s.chars().take(10).collect()),
            filler: ep
                .get("filler")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
            recap: ep
                .get("recap")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
        });
    }
    Ok((out, has_next))
}

/// Fetch all episode pages concurrently-ish (sequential but bounded).
pub async fn jikan_all_episodes(mal_id: i64, max_pages: i64) -> Result<Vec<EpisodeEntry>, String> {
    let mut all = Vec::new();
    for page in 1..=max_pages.min(10) {
        let (mut eps, has_next) = jikan_episodes(mal_id, page).await?;
        all.append(&mut eps);
        if !has_next {
            break;
        }
        // Jikan rate limit: ~3 req/s, be polite.
        tokio::time::sleep(Duration::from_millis(400)).await;
    }
    Ok(all)
}

/// Search MAL via Jikan to resolve an AniList title to a MAL id.
pub async fn jikan_search_id(title: &str) -> Result<Option<i64>, String> {
    let encoded: String = urlencoding_lite(title);
    let url = format!("{JIKAN_API}/anime?q={encoded}&limit=1&sfw=true");
    let resp = client()?.get(&url).send().await.map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("jikan returned HTTP {}", resp.status()));
    }
    let json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
    Ok(json
        .pointer("/data/0/mal_id")
        .and_then(|v| v.as_i64()))
}

/// Very small query encoder (alnum + space -> %20) to avoid a dependency.
fn urlencoding_lite(input: &str) -> String {
    let mut out = String::with_capacity(input.len() * 2);
    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
            out.push(ch);
        } else if ch == ' ' {
            out.push('%');
            out.push('2');
            out.push('0');
        } else {
            let mut buf = [0u8; 4];
            for b in ch.encode_utf8(&mut buf).as_bytes() {
                out.push_str(&format!("%{b:02X}"));
            }
        }
    }
    out
}

/// Resolve episode sources for an anime: MAL episode metadata + AniList
/// streaming links when reachable. `anime_id` is the MAL id under Jikan mode.
pub async fn resolve_sources(
    _anime_id: i64,
    title: &str,
    _title_english: Option<&str>,
    known_mal_id: Option<i64>,
) -> Result<SourcesResult, String> {
    // MAL id is the id under Jikan mode; fall back to lookup when absent.
    let mal_id = match known_mal_id {
        Some(id) if id > 0 => Some(id),
        _ => jikan_search_id(title).await?,
    };
    let mal_id = match mal_id {
        Some(id) => id,
        None => {
            return Ok(SourcesResult {
                mal_id: 0,
                anime_title: title.to_string(),
                episodes: Vec::new(),
                source: "jikan".into(),
            })
        }
    };

    // Episode metadata from Jikan.
    let mut meta: HashMap<i64, EpisodeEntry> = HashMap::new();
    let mut order: Vec<i64> = Vec::new();
    let eps = jikan_all_episodes(mal_id, 5).await.unwrap_or_default();
    for ep in eps {
        order.push(ep.number);
        meta.insert(ep.number, ep);
    }

    let max_ep = order.iter().copied().max().unwrap_or(0);
    let mut episodes: Vec<EpisodeEntry> = Vec::with_capacity(max_ep as usize);
    for n in 1..=max_ep {
        let entry = meta.remove(&n).unwrap_or(EpisodeEntry {
            number: n,
            title: format!("Episode {n}"),
            url: String::new(),
            aired: None,
            filler: false,
            recap: false,
        });
        episodes.push(entry);
    }

    Ok(SourcesResult {
        mal_id,
        anime_title: title.to_string(),
        episodes,
        source: "jikan".into(),
    })
}
