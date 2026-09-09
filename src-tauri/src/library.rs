use crate::db::Db;
use crate::types::EpisodeEntry;
use rusqlite::params;
use serde::Serialize;
use std::path::{Path, PathBuf};
use tauri::State;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalItem {
    pub title: String,
    pub episode_count: usize,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalEpisode {
    pub number: i64,
    pub title: String,
    pub path: String,
}

const VIDEO_EXTS: &[&str] = &["mp4", "mkv", "avi", "webm", "mov", "m4v"];

fn is_video(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| VIDEO_EXTS.contains(&e.to_lowercase().as_str()))
        .unwrap_or(false)
}

/// Extract a plausible episode number from a filename (S01E05, E05, EP 05,
/// Episode 5, or a trailing number). No regex dependency needed.
fn episode_number(name: &str) -> Option<i64> {
    let lower = name.to_lowercase();

    // S01E05 style
    if let Some(pos) = lower.find('e') {
        let bytes = lower.as_bytes();
        if pos > 0 && bytes[pos - 1].is_ascii_digit() {
            let after = &lower[pos + 1..];
            let digits: String = after.chars().take_while(|c| c.is_ascii_digit()).collect();
            if !digits.is_empty() {
                if let Ok(n) = digits.parse::<i64>() {
                    return Some(n);
                }
            }
        }
    }

    // "episode 5" / "ep 5" / "e 5"
    for prefix in ["episode", "ep", "e"] {
        if let Some(rest) = lower.strip_prefix(prefix) {
            let digits: String = rest.chars().skip_while(|c| !c.is_ascii_digit()).take_while(|c| c.is_ascii_digit()).collect();
            if !digits.is_empty() {
                if let Ok(n) = digits.parse::<i64>() {
                    return Some(n);
                }
            }
        }
    }

    // trailing number anywhere
    let digits: String = lower.chars().filter(|c| c.is_ascii_digit()).collect();
    if !digits.is_empty() {
        if let Ok(n) = digits.parse::<i64>() {
            return Some(n);
        }
    }
    None
}

/// Guess a clean show title from a folder name.
fn clean_title(folder: &str) -> String {
    let s = folder.replace(['_', '.'], " ");
    // strip a trailing "(2021)" or "[1080p]" style bracket group
    let mut out = s.clone();
    if let Some(idx) = s.find(" (") {
        if s.ends_with(')') && idx > 0 {
            out = s[..idx].to_string();
        }
    }
    if let Some(idx) = s.find(" [") {
        if s.ends_with(']') && idx > 0 {
            out = s[..idx].to_string();
        }
    }
    out.trim().to_string()
}

/// Scan one folder: each subfolder (or loose video file) becomes a show.
fn scan_folder(dir: &Path) -> Vec<(String, Vec<LocalEpisode>)> {
    let mut result: Vec<(String, Vec<LocalEpisode>)> = Vec::new();
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return result,
    };
    let mut subdirs: Vec<PathBuf> = Vec::new();
    let mut loose: Vec<PathBuf> = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            subdirs.push(path);
        } else if is_video(&path) {
            loose.push(path);
        }
    }
    subdirs.sort();
    loose.sort();

    for sub in subdirs {
        let mut vids: Vec<LocalEpisode> = Vec::new();
        if let Ok(files) = std::fs::read_dir(&sub) {
            let mut files: Vec<PathBuf> = files.flatten().map(|e| e.path()).filter(|p| p.is_file() && is_video(p)).collect();
            files.sort();
            for fp in files {
                let name = fp.file_stem().unwrap_or_default().to_string_lossy().to_string();
                vids.push(LocalEpisode {
                    number: episode_number(&name).unwrap_or(vids.len() as i64 + 1),
                    title: name,
                    path: fp.to_string_lossy().to_string(),
                });
            }
        }
        if !vids.is_empty() {
            vids.sort_by_key(|e| e.number);
            result.push((
                clean_title(&sub.file_name().unwrap_or_default().to_string_lossy()),
                vids,
            ));
        }
    }

    for path in loose {
        let stem = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
        result.push((
            clean_title(&stem),
            vec![LocalEpisode {
                number: episode_number(&stem).unwrap_or(1),
                title: stem.clone(),
                path: path.to_string_lossy().to_string(),
            }],
        ));
    }
    result
}

#[tauri::command]
pub async fn add_library_folder(db: State<'_, Db>, path: String) -> Result<(), String> {
    if !Path::new(&path).is_dir() {
        return Err("folder not found".into());
    }
    add_folder(&db, path).await
}

async fn add_folder(db: &Db, path: String) -> Result<(), String> {
    db.with_conn(move |conn| {
        conn.execute(
            "INSERT OR IGNORE INTO library(path, added_at) VALUES (?1, ?2)",
            params![path, chrono::Utc::now().timestamp()],
        )?;
        Ok(())
    })
}

#[tauri::command]
pub async fn remove_library_folder(db: State<'_, Db>, path: String) -> Result<(), String> {
    remove_folder(&db, path).await
}

async fn remove_folder(db: &Db, path: String) -> Result<(), String> {
    db.with_conn(move |conn| {
        conn.execute("DELETE FROM library WHERE path = ?1", params![path])?;
        Ok(())
    })
}

#[tauri::command]
pub async fn list_library_folders(db: State<'_, Db>) -> Result<Vec<String>, String> {
    folders(&db).await
}

async fn folders(db: &Db) -> Result<Vec<String>, String> {
    db.with_conn(|conn| {
        let mut stmt = conn.prepare("SELECT path FROM library ORDER BY added_at DESC")?;
        let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
        let mut out = Vec::new();
        for r in rows {
            out.push(r?);
        }
        Ok(out)
    })
}

fn scan_all_folders(paths: Vec<String>) -> Vec<(String, Vec<LocalEpisode>)> {
    let mut all = Vec::new();
    for folder in paths {
        for found in scan_folder(Path::new(&folder)) {
            all.push(found);
        }
    }
    all
}

#[tauri::command]
pub async fn scan_library(db: State<'_, Db>) -> Result<Vec<LocalItem>, String> {
    let folders = folders(&db).await?;
    let found = tokio::task::spawn_blocking(move || scan_all_folders(folders))
        .await
        .map_err(|e| e.to_string())?;
    Ok(found
        .into_iter()
        .map(|(title, eps)| LocalItem { title, episode_count: eps.len() })
        .collect())
}

/// List local episodes for one show title across all library folders.
#[tauri::command]
pub async fn local_episodes(db: State<'_, Db>, title: String) -> Result<Vec<LocalEpisode>, String> {
    local_episodes_internal(&db, &title).await
}

/// Internal helper (non-command) so other modules can fetch local episodes.
pub async fn local_episodes_internal(db: &Db, title: &str) -> Result<Vec<LocalEpisode>, String> {
    let folders = folders(db).await?;
    let owned = title.to_string();
    let found = tokio::task::spawn_blocking(move || scan_all_folders(folders))
        .await
        .map_err(|e| e.to_string())?;
    for (t, eps) in found {
        if t == owned {
            return Ok(eps);
        }
    }
    Ok(Vec::new())
}

/// Convert a LocalEpisode-like entry into the shared EpisodeEntry for the player.
pub fn local_to_entry(ep: &LocalEpisode) -> EpisodeEntry {
    EpisodeEntry {
        number: ep.number,
        title: ep.title.clone(),
        url: ep.path.clone(),
        aired: None,
        filler: false,
        recap: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn episode_number_extracts_common_patterns() {
        assert_eq!(episode_number("Show - S01E05 [1080p]"), Some(5));
        assert_eq!(episode_number("[Subs] Show - 07 (1080p)"), Some(7));
        assert_eq!(episode_number("Episode 12"), Some(12));
        assert_eq!(episode_number("ep_3"), Some(3));
        assert_eq!(episode_number("no numbers"), None);
    }

    #[test]
    fn clean_title_strips_brackets_and_years() {
        assert_eq!(clean_title("Frieren (2023) [1080p]"), "Frieren");
        assert_eq!(clean_title("Cowboy_Bebop"), "Cowboy Bebop");
    }

    #[test]
    fn is_video_checks_extensions() {
        assert!(is_video(Path::new("a.mkv")));
        assert!(is_video(Path::new("b.MP4")));
        assert!(!is_video(Path::new("c.txt")));
        assert!(!is_video(Path::new("d")));
    }
}
