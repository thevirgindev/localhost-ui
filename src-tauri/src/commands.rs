use crate::catalog::JikanCache;
use crate::db::Db;
use crate::types::{AnimeCard, EpisodeEntry, PlayableEpisode, SourcesResult};
use rusqlite::params;
use tauri::State;

// ---------------------------------------------------------------------------
// Watchlist
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchlistItem {
    pub anime_id: i64,
    pub mal_id: Option<i64>,
    pub title: String,
    pub cover: Option<String>,
    pub banner: Option<String>,
    pub color: Option<String>,
    pub episodes_total: Option<i64>,
    pub status: Option<String>,
    pub list_status: String,
    pub added_at: i64,
    pub progress: i64,
}

fn row_to_item(row: &rusqlite::Row) -> rusqlite::Result<WatchlistItem> {
    Ok(WatchlistItem {
        anime_id: row.get(0)?,
        mal_id: row.get(1)?,
        title: row.get(2)?,
        cover: row.get(3)?,
        banner: row.get(4)?,
        color: row.get(5)?,
        episodes_total: row.get(6)?,
        status: row.get(7)?,
        list_status: row.get(8)?,
        added_at: row.get(9)?,
        progress: row.get(10)?,
    })
}

const WATCHLIST_SELECT: &str = "SELECT w.anime_id, w.mal_id, w.title, w.cover, w.banner, w.color, w.episodes_total, w.status, w.list_status, w.added_at,
    COALESCE((SELECT MAX(p.episode) FROM progress p WHERE p.anime_id = w.anime_id AND p.completed = 1), 0)
    FROM watchlist w";

#[tauri::command]
pub async fn add_to_watchlist(
    db: State<'_, Db>,
    card: AnimeCard,
    list_status: Option<String>,
) -> Result<(), String> {
    db.with_conn(move |conn| {
        conn.execute(
            "INSERT INTO watchlist (anime_id, mal_id, title, cover, banner, color, episodes_total, status, added_at, list_status)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
             ON CONFLICT(anime_id) DO UPDATE SET list_status = excluded.list_status, title = excluded.title",
            params![
                card.id,
                card.mal_id,
                card.title,
                card.cover,
                card.banner,
                card.color,
                card.episodes,
                card.status,
                chrono::Utc::now().timestamp(),
                list_status.unwrap_or_else(|| "planning".into()),
            ],
        )?;
        Ok(())
    })
}

#[tauri::command]
pub async fn remove_from_watchlist(db: State<'_, Db>, anime_id: i64) -> Result<(), String> {
    db.with_conn(move |conn| {
        conn.execute("DELETE FROM watchlist WHERE anime_id = ?1", params![anime_id])?;
        Ok(())
    })
}

#[tauri::command]
pub async fn set_list_status(
    db: State<'_, Db>,
    anime_id: i64,
    list_status: String,
) -> Result<(), String> {
    db.with_conn(move |conn| {
        conn.execute(
            "UPDATE watchlist SET list_status = ?2 WHERE anime_id = ?1",
            params![anime_id, list_status],
        )?;
        Ok(())
    })
}

#[tauri::command]
pub async fn get_watchlist(db: State<'_, Db>, list_status: Option<String>) -> Result<Vec<WatchlistItem>, String> {
    db.with_conn(move |conn| {
        let sql = match list_status.as_deref() {
            Some(s) if s != "all" => format!("{WATCHLIST_SELECT} WHERE w.list_status = ?1 ORDER BY w.added_at DESC"),
            _ => format!("{WATCHLIST_SELECT} ORDER BY w.added_at DESC"),
        };
        let mut stmt = conn.prepare(&sql)?;
        let mut rows = if matches!(list_status.as_deref(), Some(s) if s != "all") {
            stmt.query_map(params![list_status.unwrap()], row_to_item)?
                .collect::<Result<Vec<_>, _>>()?
        } else {
            stmt.query_map([], row_to_item)?.collect::<Result<Vec<_>, _>>()?
        };
        rows.sort_by(|a, b| b.added_at.cmp(&a.added_at));
        Ok(rows)
    })
}

#[tauri::command]
pub async fn in_watchlist(db: State<'_, Db>, anime_id: i64) -> Result<bool, String> {
    let id = anime_id;
    db.with_conn(move |conn| {
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM watchlist WHERE anime_id = ?1",
            params![id],
            |r| r.get(0),
        )?;
        Ok(count > 0)
    })
}

// ---------------------------------------------------------------------------
// Progress
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressEntry {
    pub anime_id: i64,
    pub episode: i64,
    pub position_sec: f64,
    pub duration_sec: f64,
    pub completed: bool,
    pub updated_at: i64,
}

#[tauri::command]
pub async fn save_progress(
    db: State<'_, Db>,
    anime_id: i64,
    episode: i64,
    position_sec: f64,
    duration_sec: f64,
) -> Result<(), String> {
    let completed = if duration_sec > 0.0 && position_sec >= duration_sec - 90.0 { 1 } else { 0 };
    db.with_conn(move |conn| {
        conn.execute(
            "INSERT INTO progress (anime_id, episode, position_sec, duration_sec, completed, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(anime_id, episode) DO UPDATE SET
                position_sec = excluded.position_sec,
                duration_sec = excluded.duration_sec,
                completed = MAX(progress.completed, excluded.completed),
                updated_at = excluded.updated_at",
            params![anime_id, episode, position_sec, duration_sec, completed, chrono::Utc::now().timestamp()],
        )?;
        Ok(())
    })
}

#[tauri::command]
pub async fn get_progress(db: State<'_, Db>, anime_id: i64) -> Result<Vec<ProgressEntry>, String> {
    db.with_conn(move |conn| {
        let mut stmt = conn.prepare(
            "SELECT anime_id, episode, position_sec, duration_sec, completed, updated_at
             FROM progress WHERE anime_id = ?1 ORDER BY episode",
        )?;
        let rows = stmt
            .query_map(params![anime_id], |r| {
                Ok(ProgressEntry {
                    anime_id: r.get(0)?,
                    episode: r.get(1)?,
                    position_sec: r.get(2)?,
                    duration_sec: r.get(3)?,
                    completed: r.get::<_, i64>(4)? != 0,
                    updated_at: r.get(5)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    })
}

/// Continue-watching: shows with episode activity, most recent first.
#[tauri::command]
pub async fn continue_watching(db: State<'_, Db>) -> Result<Vec<WatchlistItem>, String> {
    db.with_conn(|conn| {
        let mut stmt = conn.prepare(&format!(
            "{WATCHLIST_SELECT} WHERE EXISTS (SELECT 1 FROM progress p WHERE p.anime_id = w.anime_id)
             ORDER BY (SELECT MAX(p.updated_at) FROM progress p WHERE p.anime_id = w.anime_id) DESC LIMIT 20"
        ))?;
        let rows = stmt.query_map([], row_to_item)?.collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    })
}

/// The exact resume point for an episode (0 if none/finished).
#[tauri::command]
pub async fn resume_point(db: State<'_, Db>, anime_id: i64, episode: i64) -> Result<f64, String> {
    db.with_conn(move |conn| {
        let row: Option<(f64, i64)> = conn
            .query_row(
                "SELECT position_sec, completed FROM progress WHERE anime_id = ?1 AND episode = ?2",
                params![anime_id, episode],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .ok();
        match row {
            Some((pos, completed)) if completed == 0 => Ok(pos),
            _ => Ok(0.0),
        }
    })
}

// ---------------------------------------------------------------------------
// Episodes / playback
// ---------------------------------------------------------------------------

#[tauri::command]
pub async fn anime_episodes(
    db: State<'_, Db>,
    anime_id: i64,
    title: String,
    title_english: Option<String>,
) -> Result<SourcesResult, String> {
    // Local library first — offline, private.
    let local = crate::library::local_episodes_internal(&db, &title).await?;
    if !local.is_empty() {
        let episodes = local
            .iter()
            .map(crate::library::local_to_entry)
            .collect::<Vec<EpisodeEntry>>();
        return Ok(SourcesResult {
            mal_id: 0,
            anime_title: title,
            episodes,
            source: "local".into(),
        });
    }

    crate::jikan::resolve_sources(anime_id, &title, title_english.as_deref(), Some(anime_id)).await
}

/// Resolve one playable episode URL for the player.
#[tauri::command]
pub async fn resolve_playable(
    db: State<'_, Db>,
    anime_id: i64,
    title: String,
    title_english: Option<String>,
    episode: i64,
) -> Result<PlayableEpisode, String> {
    let sources = anime_episodes(db, anime_id, title, title_english).await?;
    let ep = sources
        .episodes
        .iter()
        .find(|e| e.number == episode)
        .ok_or_else(|| format!("episode {episode} not found"))?;
    Ok(PlayableEpisode {
        mal_id: sources.mal_id,
        number: ep.number,
        title: ep.title.clone(),
        url: ep.url.clone(),
    })
}

// ---------------------------------------------------------------------------
// Diagnostics (frontend error capture for the local log only)
// ---------------------------------------------------------------------------

#[tauri::command]
pub async fn log_frontend_error(message: String) -> Result<(), String> {
    let path = crate::db::data_dir_standalone().join("frontend-errors.log");
    let stamp = chrono::Utc::now().to_rfc3339();
    std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .and_then(|mut f| std::io::Write::write_all(&mut f, format!("[{stamp}] {message}\n").as_bytes()))
        .map_err(|e| e.to_string())
}

// ---------------------------------------------------------------------------
// Settings
// ---------------------------------------------------------------------------

#[tauri::command]
pub async fn get_setting(db: State<'_, Db>, key: String) -> Result<Option<String>, String> {
    db.with_conn(move |conn| {
        let v = conn
            .query_row("SELECT value FROM settings WHERE key = ?1", params![key], |r| r.get(0))
            .ok();
        Ok(v)
    })
}

#[tauri::command]
pub async fn set_setting(db: State<'_, Db>, key: String, value: String) -> Result<(), String> {
    db.with_conn(move |conn| {
        conn.execute(
            "INSERT INTO settings(key, value) VALUES (?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            params![key, value],
        )?;
        Ok(())
    })
}

// JikanCache is managed state referenced by catalog commands; keep the import
// meaningful for tests that touch catalog types.
#[allow(unused_imports)]
use JikanCache as _JikanCacheImport;
