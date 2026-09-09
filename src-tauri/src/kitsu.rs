use crate::types::AnimeCard;
use std::time::Duration;

const KITSU: &str = "https://kitsu.app/api/edge";

/// Kitsu-sourced ids are offset into their own space so they never collide
/// with MAL ids used by the Jikan provider.
pub const KITSU_ID_OFFSET: i64 = 9_000_000;

fn client() -> Result<reqwest::Client, String> {
    reqwest::Client::builder()
        .user_agent("luci-app/1.0")
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|e| e.to_string())
}

// ---------------------------------------------------------------------------
// Kitsu wire types (JSON:API — only what we consume)
// ---------------------------------------------------------------------------

#[derive(Debug, serde::Deserialize)]
pub struct KitsuResponse {
    pub data: Vec<KitsuResource>,
}

#[derive(Debug, serde::Deserialize)]
pub struct KitsuResource {
    pub id: String,
    pub attributes: KitsuAttributes,
}

#[derive(Debug, serde::Deserialize)]
pub struct KitsuAttributes {
    #[serde(default)]
    pub titles: std::collections::HashMap<String, String>,
    pub canonical_title: Option<String>,
    pub poster_image: Option<KitsuImage>,
    pub cover_image: Option<KitsuCover>,
    pub subtype: Option<String>,
    pub episode_count: Option<i64>,
    pub episode_length: Option<i64>,
    pub status: Option<String>,
    pub average_rating: Option<String>,
    pub popularity_rank: Option<i64>,
    pub synopsis: Option<String>,
    pub start_date: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct KitsuImage {
    pub tiny: Option<String>,
    pub small: Option<String>,
    pub medium: Option<String>,
    pub large: Option<String>,
    pub original: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct KitsuCover {
    pub small: Option<String>,
    pub large: Option<String>,
    pub original: Option<String>,
}

fn pick_title(attrs: &KitsuAttributes) -> String {
    if let Some(t) = attrs.titles.get("en") {
        return t.clone();
    }
    if let Some(t) = attrs.titles.get("en_jp") {
        return t.clone();
    }
    if let Some(t) = attrs.titles.get("en_us") {
        return t.clone();
    }
    if let Some(t) = &attrs.canonical_title {
        return t.clone();
    }
    if let Some((_, t)) = attrs.titles.iter().next() {
        return t.clone();
    }
    "Unknown".to_string()
}

fn subtype_label(subtype: &Option<String>) -> Option<String> {
    subtype.as_ref().map(|s| match s.as_str() {
        "TV" => "TV".to_string(),
        "movie" => "MOVIE".to_string(),
        "OVA" => "OVA".to_string(),
        "ONA" => "ONA".to_string(),
        "special" => "SPECIAL".to_string(),
        other => other.to_uppercase(),
    })
}

fn status_label(status: &Option<String>) -> Option<String> {
    status.as_ref().map(|s| match s.as_str() {
        "current" => "RELEASING".to_string(),
        "finished" => "FINISHED".to_string(),
        "upcoming" => "NOT_YET_RELEASED".to_string(),
        "tba" => "NOT_YET_RELEASED".to_string(),
        "unreleased" => "NOT_YET_RELEASED".to_string(),
        other => other.to_uppercase(),
    })
}

pub fn into_card(r: KitsuResource) -> AnimeCard {
    let id: i64 = r.id.parse().unwrap_or(0);
    let attrs = r.attributes;
    AnimeCard {
        id: id + KITSU_ID_OFFSET,
        mal_id: None,
        title: pick_title(&attrs),
        title_english: attrs.titles.get("en").cloned().or_else(|| attrs.titles.get("en_us").cloned()),
        title_native: attrs.titles.get("ja_jp").cloned(),
        cover: attrs
            .poster_image
            .as_ref()
            .and_then(|p| p.large.clone().or_else(|| p.medium.clone()).or_else(|| p.small.clone()).or_else(|| p.original.clone()).or_else(|| p.tiny.clone())),
        banner: attrs
            .cover_image
            .as_ref()
            .and_then(|c| c.original.clone().or_else(|| c.large.clone()).or_else(|| c.small.clone())),
        color: None,
        format: subtype_label(&attrs.subtype),
        episodes: attrs.episode_count,
        duration: attrs.episode_length,
        status: status_label(&attrs.status),
        season: None,
        season_year: attrs.start_date.as_deref().and_then(|s| s.get(0..4)).and_then(|y| y.parse().ok()),
        average_score: attrs.average_rating.as_deref().and_then(|s| s.parse::<f64>().ok()).map(|v| (v * 10.0) as i64),
        popularity: attrs.popularity_rank,
        genres: Vec::new(), // categories require a second request; not worth it for failover rows
        description: attrs.synopsis,
        start_date: attrs.start_date,
        studio: None,
        trailer_site: None,
        trailer_id: None,
        next_airing_episode: None,
        next_airing_at: None,
        rank: None,
        members: None,
        favorites: None,
        aired_string: None,
        broadcast: None,
        rating: None,
        themes: Vec::new(),
    }
}

async fn kitsu_get(path: &str) -> Result<serde_json::Value, String> {
    let url = format!("{KITSU}{path}");
    let resp = client()?.get(&url).send().await.map_err(|e| format!("kitsu network: {e}"))?;
    if !resp.status().is_success() {
        return Err(format!("kitsu returned HTTP {}", resp.status()));
    }
    resp.json().await.map_err(|e| e.to_string())
}

/// Trending anime (used as home hero + trending row fallback).
pub async fn trending() -> Result<Vec<AnimeCard>, String> {
    let json = kitsu_get("/trending/anime?page%5Blimit%5D=20").await?;
    let parsed: KitsuResponse = serde_json::from_value(json).map_err(|e| e.to_string())?;
    Ok(parsed.data.into_iter().map(into_card).collect())
}

/// Popular anime by popularity rank.
pub async fn popular() -> Result<Vec<AnimeCard>, String> {
    let json = kitsu_get("/anime?page%5Blimit%5D=20&sort=popularityRank").await?;
    let parsed: KitsuResponse = serde_json::from_value(json).map_err(|e| e.to_string())?;
    Ok(parsed.data.into_iter().map(into_card).collect())
}

/// Seasonal anime via Kitsu's season/seasonYear filters.
pub async fn season(year: i64, season: &str) -> Result<Vec<AnimeCard>, String> {
    let s = season.to_lowercase();
    let json = kitsu_get(&format!(
        "/anime?page%5Blimit%5D=20&sort=-userCount&filter%5Bseason%5D={s}&filter%5BseasonYear%5D={year}"
    ))
    .await?;
    let parsed: KitsuResponse = serde_json::from_value(json).map_err(|e| e.to_string())?;
    Ok(parsed.data.into_iter().map(into_card).collect())
}

/// Text search.
pub async fn search(query: &str) -> Result<Vec<AnimeCard>, String> {
    let encoded: String = url_encode(query);
    let json = kitsu_get(&format!("/anime?page%5Blimit%5D=20&filter%5Btext%5D={encoded}")).await?;
    let parsed: KitsuResponse = serde_json::from_value(json).map_err(|e| e.to_string())?;
    Ok(parsed.data.into_iter().map(into_card).collect())
}

/// Single anime by Kitsu id (without the offset).
pub async fn details(kitsu_id: i64) -> Result<AnimeCard, String> {
    let json = kitsu_get(&format!("/anime/{kitsu_id}")).await?;
    let parsed: KitsuResponse = serde_json::from_value(json).map_err(|e| e.to_string())?;
    parsed
        .data
        .into_iter()
        .next()
        .map(into_card)
        .ok_or_else(|| "not found".into())
}

/// Resolve a Kitsu card from a MAL id via Kitsu's external-id filter.
pub async fn lookup_by_mal(mal_id: i64) -> Result<Option<KitsuResource>, String> {
    let url = format!("{KITSU}/anime?filter%5BexternalId%5D={mal_id}");
    let resp = client()?.get(&url).send().await.map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Ok(None);
    }
    let parsed: KitsuResponse = resp.json().await.map_err(|e| e.to_string())?;
    Ok(parsed.data.into_iter().next())
}

fn url_encode(input: &str) -> String {
    let mut out = String::with_capacity(input.len() * 2);
    for b in input.bytes() {
        if b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b == b'.' {
            out.push(b as char);
        } else if b == b' ' {
            out.push('%');
            out.push('2');
            out.push('0');
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}
