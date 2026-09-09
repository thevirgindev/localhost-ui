use crate::types::{AnimeCard, BrowsePage};
use serde::Deserialize;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tauri::State;
use tokio::sync::Mutex;

const JIKAN: &str = "https://api.jikan.moe/v4";

/// Tolerant list deserializer: Jikan occasionally returns `{}` or `null` for
/// list fields (rate-limit hiccups / partial data), which would otherwise
/// abort the whole `AnimeCard` parse with "invalid type: map, expected a sequence".
fn seq_or_empty<'de, D, T>(de: D) -> Result<Vec<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::de::DeserializeOwned,
{
    let v = serde_json::Value::deserialize(de)?;
    match v {
        serde_json::Value::Array(items) => Ok(items
            .into_iter()
            .filter_map(|x| serde_json::from_value::<T>(x).ok())
            .collect()),
        // Any non-sequence shape degrades to an empty list instead of failing.
        _ => Ok(Vec::new()),
    }
}

// ---------------------------------------------------------------------------
// Cache (same design as the old AniList cache)
// ---------------------------------------------------------------------------

struct CacheEntry {
    json: serde_json::Value,
    expires: Instant,
}

pub struct JikanCache {
    inner: Mutex<HashMap<String, CacheEntry>>,
}

impl JikanCache {
    pub fn new() -> Self {
        Self { inner: Mutex::new(HashMap::new()) }
    }

    async fn get(&self, key: &str) -> Option<serde_json::Value> {
        let mut map = self.inner.lock().await;
        if let Some(e) = map.get(key) {
            if e.expires > Instant::now() {
                return Some(e.json.clone());
            }
            map.remove(key);
        }
        None
    }

    async fn put(&self, key: String, json: serde_json::Value, ttl: Duration) {
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
        map.insert(key, CacheEntry { json, expires: Instant::now() + ttl });
    }
}

// ---------------------------------------------------------------------------
// Jikan wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct JikanPagination {
    pub last_visible_page: i64,
    pub has_next_page: bool,
    #[serde(default)]
    pub items: Option<JikanItems>,
}

#[derive(Debug, Deserialize)]
pub struct JikanItems {
    pub count: Option<i64>,
    pub total: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct JikanImageSet {
    pub jpg: Option<JikanImage>,
    pub webp: Option<JikanImage>,
}

#[derive(Debug, Deserialize)]
pub struct JikanImage {
    pub image_url: Option<String>,
    pub large_image_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct JikanTitleEntry {
    pub r#type: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct JikanAired {
    pub from: Option<String>,
    pub prop: Option<JikanAiredProp>,
    #[serde(rename = "string")]
    pub string: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct JikanAiredProp {
    pub from: Option<JikanDatePart>,
}

#[derive(Debug, Deserialize)]
pub struct JikanDatePart {
    pub year: Option<i64>,
    pub month: Option<i64>,
    pub day: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct JikanStudio {
    pub mal_id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct JikanTrailer {
    pub youtube_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct JikanAiring {
    pub prop: Option<JikanAiringCountdown>,
}

#[derive(Debug, Deserialize)]
pub struct JikanAiringCountdown {
    pub episode: Option<i64>,
    pub seconds_until: Option<i64>,
    pub time: Option<JikanDatePart>,
}

#[derive(Debug, Deserialize)]
pub struct JikanAnime {
    pub mal_id: i64,
    pub url: Option<String>,
    pub images: Option<JikanImageSet>,
    pub trailer: Option<JikanTrailer>,
    pub title: Option<String>,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    #[serde(default, deserialize_with = "seq_or_empty")]
    pub titles: Vec<JikanTitleEntry>,
    #[serde(default, deserialize_with = "seq_or_empty")]
    pub genres: Vec<JikanStudio>,
    #[serde(default, deserialize_with = "seq_or_empty")]
    pub studios: Vec<JikanStudio>,
    #[serde(default, deserialize_with = "seq_or_empty")]
    pub themes: Vec<JikanStudio>,
    pub r#type: Option<String>,
    pub source: Option<String>,
    pub episodes: Option<i64>,
    pub status: Option<String>,
    pub airing: Option<bool>,
    pub aired: Option<JikanAired>,
    pub duration: Option<String>,
    pub rating: Option<String>,
    pub score: Option<f64>,
    pub rank: Option<i64>,
    pub popularity: Option<i64>,
    pub members: Option<i64>,
    pub favorites: Option<i64>,
    pub synopsis: Option<String>,
    pub season: Option<String>,
    pub year: Option<i64>,
    pub broadcast: Option<JikanBroadcast>,
}

#[derive(Debug, Deserialize)]
pub struct JikanBroadcast {
    pub string: Option<String>,
}

impl JikanAnime {
    pub fn into_card(self) -> AnimeCard {
        let title = self
            .title_english
            .clone()
            .or_else(|| self.title.clone())
            .unwrap_or_else(|| "Unknown".into());
        let start = self
            .aired
            .as_ref()
            .and_then(|a| a.prop.as_ref())
            .and_then(|p| p.from.as_ref())
            .and_then(|f| {
                match (f.year, f.month, f.day) {
                    (Some(y), Some(m), Some(d)) => Some(format!("{y:04}-{m:02}-{d:02}")),
                    (Some(y), Some(m), None) => Some(format!("{y:04}-{m:02}")),
                    (Some(y), None, None) => Some(format!("{y:04}")),
                    _ => None,
                }
            })
            .or_else(|| self.aired.as_ref().and_then(|a| a.from.clone()).map(|s| s.chars().take(10).collect()));
        let duration_min = self.duration.as_deref().and_then(|d| {
            if d.contains("min") {
                d.split_whitespace()
                    .next()
                    .and_then(|n| n.parse::<i64>().ok())
            } else if d.contains("hr") {
                d.split_whitespace()
                    .next()
                    .and_then(|n| n.parse::<i64>().ok())
                    .map(|h| h * 60)
            } else {
                None
            }
        });
        AnimeCard {
            id: self.mal_id,
            mal_id: Some(self.mal_id),
            title,
            title_english: self.title_english.clone(),
            title_native: self.title_japanese.clone(),
            cover: self
                .images
                .as_ref()
                .and_then(|i| i.webp.as_ref().or(i.jpg.as_ref()))
                .and_then(|img| img.large_image_url.clone().or(img.image_url.clone())),
            banner: None, // Jikan has no banner art; details page uses the poster
            color: None,
            format: self.r#type,
            episodes: self.episodes,
            duration: duration_min,
            status: self.status,
            season: self.season,
            season_year: self.year,
            average_score: self.score.map(|s| (s * 10.0) as i64),
            popularity: self.popularity,
            genres: self.genres.iter().map(|g| g.name.clone()).collect(),
            description: self.synopsis,
            start_date: start,
            studio: self.studios.first().map(|s| s.name.clone()),
            trailer_site: self.trailer.as_ref().and_then(|_| Some("youtube".to_string())),
            trailer_id: self.trailer.as_ref().and_then(|t| t.youtube_id.clone()),
            next_airing_episode: None,
            next_airing_at: None,
            rank: self.rank,
            members: self.members,
            favorites: self.favorites,
            aired_string: self.aired.as_ref().and_then(|a| a.string.clone()),
            broadcast: self.broadcast.and_then(|b| b.string),
            rating: self.rating,
            themes: self.themes.iter().map(|t| t.name.clone()).collect(),
        }
    }
}

// ---------------------------------------------------------------------------
// Fetch helper with rate limiting + retry (Jikan: ~3 req/s, 429 on burst)
// ---------------------------------------------------------------------------

fn http_client() -> Result<reqwest::Client, String> {
    reqwest::Client::builder()
        .user_agent("luci-app/1.0")
        .timeout(Duration::from_secs(20))
        .build()
        .map_err(|e| e.to_string())
}

async fn jikan_get(cache: &JikanCache, key: &str, path: &str, ttl: Duration) -> Result<serde_json::Value, String> {
    if let Some(hit) = cache.get(key).await {
        return Ok(hit);
    }
    let client = http_client()?;
    let url = format!("{JIKAN}{path}");
    let mut backoff = Duration::from_millis(500);
    let mut attempt = 0;
    loop {
        attempt += 1;
        tokio::time::sleep(Duration::from_millis(350)).await; // baseline politeness delay
        let resp = client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("network: {e}"))?;
        let status = resp.status();
        if status.as_u16() == 429 && attempt <= 3 {
            tokio::time::sleep(backoff).await;
            backoff *= 2;
            continue;
        }
        if status.as_u16() == 404 {
            return Err("not found".into());
        }
        if !status.is_success() {
            return Err(format!("jikan returned HTTP {status}"));
        }
        let json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
        cache.put(key.to_string(), json.clone(), ttl).await;
        return Ok(json);
    }
}

fn parse_anime_list(value: &serde_json::Value) -> Vec<AnimeCard> {
    value
        .get("data")
        .and_then(|d| d.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|m| serde_json::from_value::<JikanAnime>(m.clone()).ok())
                .map(JikanAnime::into_card)
                .collect()
        })
        .unwrap_or_default()
}

// ---------------------------------------------------------------------------
// Commands — same surface as before, Jikan underneath
// ---------------------------------------------------------------------------

#[tauri::command]
pub async fn trending(cache: State<'_, JikanCache>, page: Option<i64>) -> Result<Vec<AnimeCard>, String> {
    let page = page.unwrap_or(1);
    match jikan_get(&cache, &format!("trending:{page}"), &format!("/top/anime?page={page}&limit=24&filter=airing"), Duration::from_secs(600)).await {
        Ok(json) => Ok(parse_anime_list(&json)),
        Err(_) => crate::kitsu::trending().await,
    }
}

#[tauri::command]
pub async fn popular(cache: State<'_, JikanCache>, page: Option<i64>) -> Result<Vec<AnimeCard>, String> {
    let page = page.unwrap_or(1);
    let jikan = jikan_get(&cache, &format!("popular:{page}"), &format!("/top/anime?page={page}&limit=24"), Duration::from_secs(3600)).await;
    match jikan {
        Ok(v) => Ok(parse_anime_list(&v)),
        Err(_) => {
            if page == 1 {
                crate::kitsu::popular().await
            } else {
                Err("catalog provider unavailable".into())
            }
        }
    }
}

#[tauri::command]
pub async fn season(
    cache: State<'_, JikanCache>,
    year: i64,
    season: String,
    page: Option<i64>,
) -> Result<Vec<AnimeCard>, String> {
    let page = page.unwrap_or(1);
    let s = season.to_lowercase();
    let jikan = jikan_get(
        &cache,
        &format!("season:{year}:{s}:{page}"),
        &format!("/seasons/{year}/{s}?page={page}&limit=24"),
        Duration::from_secs(3600),
    )
    .await;
    match jikan {
        Ok(json) => Ok(parse_anime_list(&json)),
        Err(_) if page == 1 => crate::kitsu::season(year, &s).await,
        Err(e) => Err(e),
    }
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseFilters {
    pub search: Option<String>,
    pub genres: Option<Vec<String>>,
    pub year: Option<i64>,
    pub season: Option<String>,
    pub format: Option<String>,
    pub status: Option<String>,
    pub sort: Option<String>,
    pub page: Option<i64>,
}

#[tauri::command]
pub async fn browse(cache: State<'_, JikanCache>, filters: BrowseFilters) -> Result<BrowsePage, String> {
    let page = filters.page.unwrap_or(1);
    let mut params: Vec<String> = vec![format!("page={page}"), "limit=24".into(), "sfw=true".into()];

    if let Some(q) = filters.search.as_deref().map(str::trim).filter(|q| !q.is_empty()) {
        let encoded: String = q
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '_' { c.to_string() } else { format!("%{:02X}", c as u32) })
            .collect();
        params.push(format!("q={encoded}"));
    }
    if let Some(year) = filters.year {
        params.push(format!("year={year}"));
    }
    if let Some(season) = filters.season.as_deref().filter(|s| !s.is_empty()) {
        params.push(format!("season={}", season.to_lowercase()));
    }
    if let Some(format) = filters.format.as_deref().filter(|f| !f.is_empty()) {
        // Jikan type names: TV, Movie, OVA, ONA, Special, TV
        let jf = match format.to_uppercase().as_str() {
            "MOVIE" => "movie".into(),
            "TV_SHORT" => "tv".into(),
            other => other.to_lowercase(),
        };
        params.push(format!("type={jf}"));
    }
    if let Some(status) = filters.status.as_deref().filter(|s| !s.is_empty()) {
        let js = match status.to_uppercase().as_str() {
            "RELEASING" => "airing".to_string(),
            "FINISHED" => "complete".to_string(),
            "NOT_YET_RELEASED" => "upcoming".to_string(),
            other => other.to_lowercase(),
        };
        params.push(format!("status={js}"));
    }
    if let Some(genres) = &filters.genres {
        if !genres.is_empty() {
            // Jikan accepts genre names in `genres=` (e.g. "Action").
            let names: Vec<String> = genres.iter().map(|g| g.replace(' ', "%20")).collect();
            params.push(format!("genres={}", names.join(",")));
        }
    }
    if let Some(sort) = filters.sort.as_deref() {
        let js = match sort {
            "SCORE_DESC" => "score",
            "POPULARITY_DESC" => "popularity",
            "FAVOURITES_DESC" => "favorite",
            "START_DATE_DESC" => "start_date",
            _ => "popularity",
        };
        params.push(format!("order_by={js}&sort=desc"));
    }

    let key = format!("browse:{}:{page}", params.join("&"));
    let path = format!("/anime?{}", params.join("&"));
    let jikan = jikan_get(&cache, &key, &path, Duration::from_secs(900)).await;
    match jikan {
        Ok(json) => {
            let cards = parse_anime_list(&json);
            let has_next = json
                .pointer("/pagination/has_next_page")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let total = json
                .pointer("/pagination/items/total")
                .and_then(|v| v.as_i64());
            Ok(BrowsePage { cards, has_next_page: has_next, total })
        }
        Err(_) => {
            // Kitsu failover: search or popularity sort (page 1 only).
            if page > 1 {
                return Err("catalog provider unavailable".into());
            }
            let cards = if let Some(q) = filters.search.as_deref().map(str::trim).filter(|q| !q.is_empty()) {
                crate::kitsu::search(q).await?
            } else {
                crate::kitsu::popular().await?
            };
            Ok(BrowsePage { cards, has_next_page: false, total: None })
        }
    }
}

#[tauri::command]
pub async fn anime_details(cache: State<'_, JikanCache>, id: i64) -> Result<AnimeCard, String> {
    // Kitsu ids live in their own offset space.
    if id >= crate::kitsu::KITSU_ID_OFFSET {
        return crate::kitsu::details(id - crate::kitsu::KITSU_ID_OFFSET).await;
    }

    let jikan = jikan_get(&cache, &format!("details:{id}"), &format!("/anime/{id}/full"), Duration::from_secs(3600)).await;
    match jikan {
        Ok(json) => match serde_json::from_value::<JikanAnime>(
            json.get("data").cloned().unwrap_or(serde_json::Value::Null),
        ) {
            Ok(anime) => Ok(anime.into_card()),
            Err(e) => {
                eprintln!("[luci] details parse failed for {id}: {e}; trying Kitsu fallback");
                let mapped = kitsu_by_mal(id).await?;
                mapped.ok_or_else(|| format!("anime details parse failed: {e}"))
            }
        },
        Err(_) => {
            // Kitsu detail fallback requires resolving the MAL id → Kitsu id.
            let mapped = kitsu_by_mal(id).await?;
            mapped.ok_or_else(|| "anime not found".to_string())
        }
    }
}

async fn kitsu_by_mal(mal_id: i64) -> Result<Option<AnimeCard>, String> {
    let json = crate::kitsu::lookup_by_mal(mal_id).await?;
    Ok(json.map(crate::kitsu::into_card))
}

/// Current season info (season calendar header).
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SeasonInfo {
    pub season: String,
    pub year: i64,
    pub label: String,
}

#[tauri::command]
pub fn current_season_info() -> SeasonInfo {
    use chrono::Datelike;
    let now = chrono::Utc::now();
    let (month, year) = (now.month(), now.year());
    let season = match month {
        1..=3 => "winter",
        4..=6 => "spring",
        7..=9 => "summer",
        _ => "fall",
    };
    SeasonInfo {
        season: season.to_uppercase(),
        year: year as i64,
        label: format!("{} {}", capitalize(season), year),
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

/// Jikan airing schedule for a season page badge (optional enrichment).
#[tauri::command]
pub async fn season_now(cache: State<'_, JikanCache>) -> Result<Vec<AnimeCard>, String> {
    let json = jikan_get(&cache, "season-now", "/seasons/now?limit=24", Duration::from_secs(900)).await?;
    Ok(parse_anime_list(&json))
}

// ---------------------------------------------------------------------------
// Characters (details page tab)
// ---------------------------------------------------------------------------

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterCard {
    pub id: i64,
    pub name: String,
    pub image: Option<String>,
    pub role: String,
    pub voice_actor: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct JikanCharacterEntry {
    pub character: JikanCharacterRef,
    pub role: Option<String>,
    #[serde(default)]
    pub voice_actors: Vec<JikanVoiceActorEntry>,
}

#[derive(Debug, Deserialize)]
pub struct JikanCharacterRef {
    pub mal_id: i64,
    pub name: Option<String>,
    pub images: Option<JikanImageSet>,
}

#[derive(Debug, Deserialize)]
pub struct JikanVoiceActorEntry {
    pub person: Option<JikanPersonRef>,
    pub language: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct JikanPersonRef {
    pub name: Option<String>,
}

#[tauri::command]
pub async fn anime_characters(cache: State<'_, JikanCache>, id: i64) -> Result<Vec<CharacterCard>, String> {
    let json = jikan_get(
        &cache,
        &format!("characters:{id}"),
        &format!("/anime/{id}/characters"),
        Duration::from_secs(3600),
    )
    .await;
    // If Jikan is down the details page still works; characters just stay empty.
    let json = match json {
        Ok(v) => v,
        Err(_) => return Ok(Vec::new()),
    };
    let entries: Vec<JikanCharacterEntry> = json
        .get("data")
        .and_then(|d| serde_json::from_value::<Vec<JikanCharacterEntry>>(d.clone()).ok())
        .unwrap_or_default();
    Ok(entries
        .into_iter()
        .take(12)
        .map(|e| {
            let va = e
                .voice_actors
                .iter()
                .find(|v| v.language.as_deref() == Some("Japanese"))
                .or_else(|| e.voice_actors.first());
            CharacterCard {
                id: e.character.mal_id,
                name: e.character.name.unwrap_or_else(|| "Unknown".into()),
                image: e
                    .character
                    .images
                    .as_ref()
                    .and_then(|i| i.webp.as_ref().or(i.jpg.as_ref()))
                    .and_then(|img| img.image_url.clone().or(img.large_image_url.clone())),
                role: e.role.unwrap_or_else(|| "Main".into()),
                voice_actor: va.and_then(|v| v.person.as_ref().and_then(|p| p.name.clone())),
            }
        })
        .collect())
}
