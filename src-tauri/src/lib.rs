mod catalog;
mod commands;
mod db;
mod jikan;
mod kitsu;
mod library;
#[cfg(test)]
mod tests;
mod types;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle().clone();
            let dir = db::data_dir(&handle);
            let conn = db::Db::open(dir.join("luci.db"))
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
            app.manage(conn);
            app.manage(catalog::JikanCache::new());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            catalog::trending,
            catalog::popular,
            catalog::season,
            catalog::browse,
            catalog::anime_details,
            catalog::current_season_info,
            catalog::season_now,
            catalog::anime_characters,
            commands::add_to_watchlist,
            commands::remove_from_watchlist,
            commands::set_list_status,
            commands::get_watchlist,
            commands::in_watchlist,
            commands::save_progress,
            commands::get_progress,
            commands::continue_watching,
            commands::resume_point,
            commands::anime_episodes,
            commands::resolve_playable,
            commands::get_setting,
            commands::set_setting,
            commands::log_frontend_error,
            library::add_library_folder,
            library::remove_library_folder,
            library::list_library_folders,
            library::scan_library,
            library::local_episodes,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Luci");
}
