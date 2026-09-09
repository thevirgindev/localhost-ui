use crate::db::Db;
use rusqlite::params;

fn test_db() -> Db {
    Db::open(std::env::temp_dir().join(format!("luci-test-{}.db", std::process::id())))
        .expect("failed to open test db")
}

#[test]
fn watchlist_roundtrip() {
    let db = test_db();
    db.with_conn(|conn| {
        conn.execute(
            "INSERT INTO watchlist (anime_id, mal_id, title, cover, banner, color, episodes_total, status, added_at, list_status)
             VALUES (1, 11, 'Test Anime', NULL, NULL, NULL, 12, 'FINISHED', 1000, 'watching')",
            [],
        )?;
        Ok(())
    })
    .expect("insert failed");

    let count: i64 = db
        .with_conn(|conn| {
            Ok(conn.query_row("SELECT COUNT(*) FROM watchlist", [], |r| r.get(0))?)
        })
        .unwrap();
    assert_eq!(count, 1);
}

#[test]
fn progress_upsert_and_completion() {
    let db = test_db();
    db.with_conn(|conn| {
        // first save at 30s of a 300s episode -> not complete
        conn.execute(
            "INSERT INTO progress (anime_id, episode, position_sec, duration_sec, completed, updated_at)
             VALUES (1, 1, 30.0, 300.0, 0, 1000)",
            [],
        )?;
        // second save at 295s -> complete
        conn.execute(
            "INSERT INTO progress (anime_id, episode, position_sec, duration_sec, completed, updated_at)
             VALUES (1, 1, 295.0, 300.0, 1, 2000)
             ON CONFLICT(anime_id, episode) DO UPDATE SET
                position_sec = excluded.position_sec,
                duration_sec = excluded.duration_sec,
                completed = MAX(progress.completed, excluded.completed),
                updated_at = excluded.updated_at",
            [],
        )?;
        let (pos, completed): (f64, i64) = conn.query_row(
            "SELECT position_sec, completed FROM progress WHERE anime_id = 1 AND episode = 1",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )?;
        assert_eq!(pos, 295.0);
        assert_eq!(completed, 1);

        // completion must never regress
        conn.execute(
            "INSERT INTO progress (anime_id, episode, position_sec, duration_sec, completed, updated_at)
             VALUES (1, 1, 10.0, 300.0, 0, 3000)
             ON CONFLICT(anime_id, episode) DO UPDATE SET
                position_sec = excluded.position_sec,
                duration_sec = excluded.duration_sec,
                completed = MAX(progress.completed, excluded.completed),
                updated_at = excluded.updated_at",
            [],
        )?;
        let completed_after: i64 = conn.query_row(
            "SELECT completed FROM progress WHERE anime_id = 1 AND episode = 1",
            [],
            |r| r.get(0),
        )?;
        assert_eq!(completed_after, 1);
        Ok(())
    })
    .expect("progress test failed");
}

#[test]
fn settings_upsert() {
    let db = test_db();
    db.with_conn(|conn| {
        conn.execute(
            "INSERT INTO settings(key, value) VALUES ('theme', 'dark')
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            params![],
        )?;
        conn.execute(
            "INSERT INTO settings(key, value) VALUES ('theme', 'darker')
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            params![],
        )?;
        let v: String = conn.query_row("SELECT value FROM settings WHERE key = 'theme'", [], |r| r.get(0))?;
        assert_eq!(v, "darker");
        Ok(())
    })
    .expect("settings test failed");
}

#[test]
fn library_folder_unique() {
    let db = test_db();
    db.with_conn(|conn| {
        conn.execute(
            "INSERT OR IGNORE INTO library(path, added_at) VALUES ('C:/anime', 1)",
            [],
        )?;
        // duplicate insert is ignored
        conn.execute(
            "INSERT OR IGNORE INTO library(path, added_at) VALUES ('C:/anime', 2)",
            [],
        )?;
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM library", [], |r| r.get(0))?;
        assert_eq!(count, 1);
        Ok(())
    })
    .expect("library test failed");
}
