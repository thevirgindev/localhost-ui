// API surface: typed wrappers around every Tauri command.

import { invoke } from "./ipc";
import type {
  AnimeCard,
  BrowsePage,
  LocalEpisode,
  LocalItem,
  ProgressEntry,
  SeasonInfo,
  SourcesResult,
  WatchlistItem,
} from "./types";

export const api = {
  // catalog (Jikan/MAL underneath)
  trending: (page = 1) => invoke<AnimeCard[]>("trending", { page }),
  popular: (page = 1) => invoke<AnimeCard[]>("popular", { page }),
  season: (year: number, season: string, page = 1) =>
    invoke<AnimeCard[]>("season", { year, season, page }),
  browse: (filters: Record<string, unknown>) => invoke<BrowsePage>("browse", { filters }),
  details: (id: number) => invoke<AnimeCard>("anime_details", { id }),
  characters: (id: number) =>
    invoke<{ id: number; name: string; image: string | null; role: string; voiceActor: string | null }[]>(
      "anime_characters",
      { id },
    ),
  seasonInfo: () => invoke<SeasonInfo>("current_season_info"),

  // episodes
  episodes: (animeId: number, title: string, titleEnglish: string | null) =>
    invoke<SourcesResult>("anime_episodes", {
      animeId,
      title,
      titleEnglish,
    }),

  // watchlist
  addToWatchlist: (card: AnimeCard, listStatus?: string) =>
    invoke<void>("add_to_watchlist", { card, listStatus: listStatus ?? null }),
  removeFromWatchlist: (animeId: number) =>
    invoke<void>("remove_from_watchlist", { animeId }),
  setListStatus: (animeId: number, listStatus: string) =>
    invoke<void>("set_list_status", { animeId, listStatus }),
  watchlist: (listStatus?: string) =>
    invoke<WatchlistItem[]>("get_watchlist", { listStatus: listStatus ?? null }),
  inWatchlist: (animeId: number) => invoke<boolean>("in_watchlist", { animeId }),

  // progress
  saveProgress: (animeId: number, episode: number, positionSec: number, durationSec: number) =>
    invoke<void>("save_progress", { animeId, episode, positionSec, durationSec }),
  progress: (animeId: number) => invoke<ProgressEntry[]>("get_progress", { animeId }),
  continueWatching: () => invoke<WatchlistItem[]>("continue_watching"),
  resumePoint: (animeId: number, episode: number) =>
    invoke<number>("resume_point", { animeId, episode }),

  // library
  addLibraryFolder: (path: string) => invoke<void>("add_library_folder", { path }),
  removeLibraryFolder: (path: string) => invoke<void>("remove_library_folder", { path }),
  listLibraryFolders: () => invoke<string[]>("list_library_folders"),
  scanLibrary: () => invoke<LocalItem[]>("scan_library"),
  localEpisodes: (title: string) => invoke<LocalEpisode[]>("local_episodes", { title }),

  // settings
  getSetting: (key: string) => invoke<string | null>("get_setting", { key }),
  setSetting: (key: string, value: string) => invoke<void>("set_setting", { key, value }),
};
