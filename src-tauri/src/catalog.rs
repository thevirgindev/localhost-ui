use crate::providers::{self, ResponseCache};
use crate::types::{AnimeCard, BrowsePage};
use serde::Deserialize;
use std::time::Duration;
use tauri::State;

/// Managed metadata cache (LuciAPI core).
pub type LuciCache = ResponseCache;

/// Tolerant list deserializer: providers occasionally return `{}` or `null`
/// for list fields (rate-limit hiccups / partial data), which would otherwise
/// abort the whole `AnimeCard` parse.
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
        _ => Ok(Vec::new()),
    }
}

// ---------------------------------------------------------------------------
// Jikan wire types (only fields the app consumes)
// ---------------------------------------------------------------------------

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
pub struct JikanNamed {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct JikanTrailer {
    pub youtube_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct JikanAnime {
    pub mal_id: i64,
    pub images: Option<JikanImageSet>,
    pub trailer: Option<JikanTrailer>,
    pub title: Option<String>,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    #[serde(default, deserialize_with = "seq_or_empty")]
    pub genres: Vec<JikanNamed>,
    #[serde(default, deserialize_with = "seq_or_empty")]
    pub studios: Vec<JikanNamed>,
    #[serde(default, deserialize_with = "seq_or_empty")]
    pub themes: Vec<JikanNamed>,
    pub r#type: Option<String>,
    pub episodes: Option<i64>,
    pub status: Option<String>,
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
                d.split_whitespace().next().and_then(|n| n.parse::<i64>().ok())
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
            banner: None,
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
            trailer_site: Some("youtube".to_string()).filter(|_| self.trailer.is_some()),
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
// Shikimori fallback (MAL-compatible ids) — third catalog provider
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct ShikiAnime {
    id: i64,
    name: Option<String>,
    russian: Option<String>,
    #[serde(default)]
    image: Option<ShikiImage>,
    episodes: Option<i64>,
    status: Option<String>,
    description: Option<String>,
    description_source: Option<String>,
    released_on: Option<String>,
    aired_on: Option<String>,
    score: Option<String>,
    #[serde(default)]
    genres: Vec<ShikiGenre>,
    #[serde(default)]
    studios: Vec<ShikiNamed>,
    kind: Option<String>,
    season: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ShikiImage {
    original: Option<String>,
    preview: Option<String>,
    #[serde(default)]
    banner: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ShikiGenre {
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ShikiNamed {
    name: Option<String>,
}

impl ShikiAnime {
    fn into_card(self) -> AnimeCard {
        let kind = self.kind.as_deref().map(|k| match k {
            "tv" => "TV".to_string(),
            "movie" => "MOVIE".to_string(),
            "ova" => "OVA".to_string(),
            "ona" => "ONA".to_string(),
            "special" => "SPECIAL".to_string(),
            other => other.to_uppercase(),
        });
        AnimeCard {
            id: self.id,
            mal_id: Some(self.id), // Shikimori ids mirror MAL ids
            title: self.name.unwrap_or_else(|| "Unknown".into()),
            title_english: None,
            title_native: self.russian,
            cover: self.image.as_ref().and_then(|i| {
                i.original
                    .as_ref()
                    .or(i.preview.as_ref())
                    .map(|p| format!("https://shikimori.one{p}"))
            }),
            banner: self
                .image
                .as_ref()
                .and_then(|i| i.banner.as_ref())
                .map(|b| format!("https://shikimori.one{b}")),
            color: None,
            format: kind,
            episodes: self.episodes,
            duration: None,
            status: self.status.as_deref().map(|s| match s {
                "ongoing" => "RELEASING".to_string(),
                "released" => "FINISHED".to_string(),
                "anons" => "NOT_YET_RELEASED".to_string(),
                other => other.to_uppercase(),
            }),
            season: self.season.as_deref().map(|s| {
                s.split('_').next().unwrap_or(s).to_uppercase()
            }),
            season_year: self
                .aired_on
                .as_deref()
                .and_then(|s| s.get(0..4))
                .and_then(|y| y.parse().ok()),
            average_score: self.score.as_deref().and_then(|s| s.parse::<f64>().ok()).map(|v| (v * 10.0) as i64),
            popularity: None,
            genres: self.genres.iter().filter_map(|g| g.name.clone()).collect(),
            description: self.description.or(self.description_source),
            start_date: self.aired_on.as_deref().map(|s| s.chars().take(10).collect()),
            studio: self.studios.first().and_then(|s| s.name.clone()),
            trailer_site: None,
            trailer_id: None,
            next_airing_episode: None,
            next_airing_at: None,
            rank: None,
            members: None,
            favorites: None,
            aired_string: self.released_on,
            broadcast: None,
            rating: None,
            themes: Vec::new(),
        }
    }
}

async fn shiki_get(cache: &ResponseCache, key: &str, path: &str) -> Result<serde_json::Value, String> {
    providers::cached_json(cache, key, &format!("{}{path}", providers::SHIKI), Duration::from_secs(1800)).await
}

fn parse_shiki_list(value: &serde_json::Value) -> Vec<AnimeCard> {
    value
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|m| serde_json::from_value::<ShikiAnime>(m.clone()).ok())
                .map(ShikiAnime::into_card)
                .collect()
        })
        .unwrap_or_default()
}

// ---------------------------------------------------------------------------
// Fetch helpers
// ---------------------------------------------------------------------------

/// Jikan GET with politeness delay + shared cache.
async fn jikan_get(cache: &ResponseCache, key: &str, path: &str, ttl: Duration) -> Result<serde_json::Value, String> {
    tokio::time::sleep(Duration::from_millis(350)).await; // Jikan: ~3 req/s
    providers::cached_json(cache, key, &format!("{}{path}", providers::JIKAN), ttl).await
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
// Catalog commands — fallback chain: Jikan → Kitsu → Shikimori
// ---------------------------------------------------------------------------

#[tauri::command]
pub async fn trending(cache: State<'_, LuciCache>, page: Option<i64>) -> Result<Vec<AnimeCard>, String> {
    let page = page.unwrap_or(1);
    let jikan = jikan_get(
        &cache,
        &format!("jikan:trending:{page}"),
        &format!("/top/anime?page={page}&limit=24&filter=airing"),
        Duration::from_secs(600),
    )
    .await;
    match jikan {
        Ok(json) => Ok(parse_anime_list(&json)),
        Err(_) => match crate::kitsu::trending().await {
            Ok(cards) => Ok(cards),
            Err(_) => shiki_get(&cache, "shiki:trending", "/animes?limit=24&order=ranked")
                .await
                .map(|v| parse_shiki_list(&v)),
        },
    }
}

#[tauri::command]
pub async fn popular(cache: State<'_, LuciCache>, page: Option<i64>) -> Result<Vec<AnimeCard>, String> {
    let page = page.unwrap_or(1);
    let jikan = jikan_get(
        &cache,
        &format!("jikan:popular:{page}"),
        &format!("/top/anime?page={page}&limit=24"),
        Duration::from_secs(3600),
    )
    .await;
    match jikan {
        Ok(json) => Ok(parse_anime_list(&json)),
        Err(_) => match crate::kitsu::popular().await {
            Ok(cards) if page == 1 => Ok(cards),
            _ => shiki_get(
                &cache,
                &format!("shiki:popular:{page}"),
                &format!("/animes?limit=24&page={page}&order=popularity"),
            )
            .await
            .map(|v| parse_shiki_list(&v)),
        },
    }
}

#[tauri::command]
pub async fn season(
    cache: State<'_, LuciCache>,
    year: i64,
    season: String,
    page: Option<i64>,
) -> Result<Vec<AnimeCard>, String> {
    let page = page.unwrap_or(1);
    let s = season.to_lowercase();
    let jikan = jikan_get(
        &cache,
        &format!("jikan:season:{year}:{s}:{page}"),
        &format!("/seasons/{year}/{s}?page={page}&limit=24"),
        Duration::from_secs(3600),
    )
    .await;
    match jikan {
        Ok(json) => Ok(parse_anime_list(&json)),
        Err(_) => match crate::kitsu::season(year, &s).await {
            Ok(cards) if page == 1 => Ok(cards),
            _ => shiki_get(
                &cache,
                &format!("shiki:season:{year}:{s}:{page}"),
                &format!("/animes?limit=24&page={page}&season={s}_{year}"),
            )
            .await
            .map(|v| parse_shiki_list(&v)),
        },
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
pub async fn browse(cache: State<'_, LuciCache>, filters: BrowseFilters) -> Result<BrowsePage, String> {
    let page = filters.page.unwrap_or(1);
    let mut params: Vec<String> = vec![format!("page={page}"), "limit=24".into(), "sfw=true".into()];

    if let Some(q) = filters.search.as_deref().map(str::trim).filter(|q| !q.is_empty()) {
        params.push(format!("q={}", providers::urlencode(q)));
    }
    if let Some(year) = filters.year {
        params.push(format!("year={year}"));
    }
    if let Some(season) = filters.season.as_deref().filter(|s| !s.is_empty()) {
        params.push(format!("season={}", season.to_lowercase()));
    }
    if let Some(format) = filters.format.as_deref().filter(|f| !f.is_empty()) {
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

    let key = format!("jikan:browse:{}:{page}", params.join("&"));
    let path = format!("/anime?{}", params.join("&"));
    let jikan = jikan_get(&cache, &key, &path, Duration::from_secs(900)).await;
    match jikan {
        Ok(json) => {
            let cards = parse_anime_list(&json);
            let has_next = json.pointer("/pagination/has_next_page").and_then(|v| v.as_bool()).unwrap_or(false);
            let total = json.pointer("/pagination/items/total").and_then(|v| v.as_i64());
            Ok(BrowsePage { cards, has_next_page: has_next, total })
        }
        Err(_) => {
            // Kitsu failover: search or popularity (page 1 only).
            if page > 1 {
                return Err("all catalog providers unreachable".into());
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
pub async fn anime_details(cache: State<'_, LuciCache>, id: i64) -> Result<AnimeCard, String> {
    // Kitsu ids live in their own offset space.
    if id >= providers::KITSU_ID_OFFSET {
        return crate::kitsu::details(id - providers::KITSU_ID_OFFSET).await;
    }

    let jikan = jikan_get(&cache, &format!("jikan:details:{id}"), &format!("/anime/{id}/full"), Duration::from_secs(3600)).await;
    match jikan {
        Ok(json) => match serde_json::from_value::<JikanAnime>(json.get("data").cloned().unwrap_or(serde_json::Value::Null)) {
            Ok(anime) => Ok(anime.into_card()),
            Err(e) => {
                eprintln!("[luci] details parse failed for {id}: {e}; trying Kitsu, then Shikimori");
                let mapped = kitsu_by_mal(&cache, id).await?;
                mapped.ok_or_else(|| format!("anime details parse failed: {e}"))
            }
        },
        Err(_) => {
            let mapped = kitsu_by_mal(&cache, id).await?;
            mapped.ok_or_else(|| "anime not found".to_string())
        }
    }
}

async fn kitsu_by_mal(cache: &ResponseCache, mal_id: i64) -> Result<Option<AnimeCard>, String> {
    if let Ok(json) = crate::kitsu::lookup_by_mal(mal_id).await {
        if let Some(res) = json {
            return Ok(Some(crate::kitsu::into_card(res)));
        }
    }
    // Shikimori ids mirror MAL ids — direct detail fetch.
    if let Ok(json) = shiki_get(cache, &format!("shiki:details:{mal_id}"), &format!("/animes/{mal_id}")).await {
        if let Ok(shiki) = serde_json::from_value::<ShikiAnime>(json) {
            return Ok(Some(shiki.into_card()));
        }
    }
    Ok(None)
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

#[tauri::command]
pub async fn season_now(cache: State<'_, LuciCache>) -> Result<Vec<AnimeCard>, String> {
    let jikan = jikan_get(&cache, "jikan:season-now", "/seasons/now?limit=24", Duration::from_secs(900)).await;
    match jikan {
        Ok(json) => Ok(parse_anime_list(&json)),
        Err(_) => crate::kitsu::trending().await,
    }
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
pub async fn anime_characters(cache: State<'_, LuciCache>, id: i64) -> Result<Vec<CharacterCard>, String> {
    let json = jikan_get(
        &cache,
        &format!("jikan:characters:{id}"),
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
