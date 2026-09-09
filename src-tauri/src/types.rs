use serde::{Deserialize, Serialize};

/// The anime card used across the app (source-normalized).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnimeCard {
    pub id: i64,
    pub mal_id: Option<i64>,
    pub title: String,
    pub title_english: Option<String>,
    pub title_native: Option<String>,
    pub cover: Option<String>,
    pub banner: Option<String>,
    pub color: Option<String>,
    pub format: Option<String>,
    pub episodes: Option<i64>,
    pub duration: Option<i64>,
    pub status: Option<String>,
    pub season: Option<String>,
    pub season_year: Option<i64>,
    pub average_score: Option<i64>,
    pub popularity: Option<i64>,
    pub genres: Vec<String>,
    pub description: Option<String>,
    pub start_date: Option<String>,
    pub studio: Option<String>,
    pub trailer_site: Option<String>,
    pub trailer_id: Option<String>,
    pub next_airing_episode: Option<i64>,
    pub next_airing_at: Option<i64>,
    /// Extra detail-page fields (Information tab).
    pub rank: Option<i64>,
    pub members: Option<i64>,
    pub favorites: Option<i64>,
    pub aired_string: Option<String>,
    pub broadcast: Option<String>,
    pub rating: Option<String>,
    pub themes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowsePage {
    pub cards: Vec<AnimeCard>,
    pub has_next_page: bool,
    pub total: Option<i64>,
}

/// Anime metadata + episode list, source-agnostic.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeEntry {
    pub number: i64,
    pub title: String,
    pub url: String,
    pub aired: Option<String>,
    pub filler: bool,
    pub recap: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SourcesResult {
    pub mal_id: i64,
    pub anime_title: String,
    pub episodes: Vec<EpisodeEntry>,
    pub source: String,
}

/// Mapping result used by the player.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayableEpisode {
    pub mal_id: i64,
    pub number: i64,
    pub title: String,
    pub url: String,
}
