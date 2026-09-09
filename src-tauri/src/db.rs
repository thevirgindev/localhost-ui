use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::Mutex;

pub struct Db(Mutex<Connection>);

pub fn data_dir(_app: &tauri::AppHandle) -> PathBuf {
    data_dir_standalone()
}

/// Same directory, usable from anywhere (no AppHandle needed).
pub fn data_dir_standalone() -> PathBuf {
    let dir = dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("luci");
    let _ = std::fs::create_dir_all(&dir);
    dir
}

impl Db {
    pub fn open(path: PathBuf) -> Result<Self, String> {
        let conn = Connection::open(path).map_err(|e| e.to_string())?;
        conn.execute_batch(
            r#"
            PRAGMA journal_mode = WAL;
            PRAGMA synchronous = NORMAL;

            CREATE TABLE IF NOT EXISTS watchlist (
                anime_id INTEGER PRIMARY KEY,
                mal_id INTEGER,
                title TEXT NOT NULL,
                cover TEXT,
                banner TEXT,
                color TEXT,
                episodes_total INTEGER,
                status TEXT,
                added_at INTEGER NOT NULL,
                list_status TEXT NOT NULL DEFAULT 'planning'
            );
            CREATE INDEX IF NOT EXISTS idx_watchlist_status ON watchlist(list_status);

            CREATE TABLE IF NOT EXISTS progress (
                anime_id INTEGER NOT NULL,
                episode INTEGER NOT NULL,
                position_sec REAL NOT NULL DEFAULT 0,
                duration_sec REAL NOT NULL DEFAULT 0,
                completed INTEGER NOT NULL DEFAULT 0,
                updated_at INTEGER NOT NULL,
                PRIMARY KEY (anime_id, episode)
            );

            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS library (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                path TEXT NOT NULL UNIQUE,
                added_at INTEGER NOT NULL
            );
            "#,
        )
        .map_err(|e| e.to_string())?;
        Ok(Self(Mutex::new(conn)))
    }

    pub fn with_conn<T>(
        &self,
        f: impl FnOnce(&Connection) -> rusqlite::Result<T>,
    ) -> Result<T, String> {
        let conn = self.0.lock().map_err(|e| e.to_string())?;
        f(&conn).map_err(|e| e.to_string())
    }
}
