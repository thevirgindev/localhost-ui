// Thin wrapper over Tauri invoke, with seamless web preview fallback.

import { MOCK_ANIME, MOCK_SEASON_INFO, generateEpisodes } from "./mockData";
import type { AnimeCard, BrowsePage, ProgressEntry, WatchlistItem } from "./types";

const isTauri = typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

export async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  if (isTauri) {
    const { invoke: tauriInvoke } = await import("@tauri-apps/api/core");
    return tauriInvoke<T>(cmd, args);
  }

  // Browser / Web Preview fallback
  return (await handleBrowserMock(cmd, args)) as T;
}

export const tauriAvailable = isTauri;

// Fire-and-forget local error capture — written to a local log file only.
export function reportError(where: string, err: unknown) {
  if (!isTauri) {
    console.debug(`[localhost error: ${where}]`, err);
    return;
  }
  const message = `${where}: ${err instanceof Error ? (err.stack ?? err.message) : String(err)}`;
  import("@tauri-apps/api/core")
    .then(({ invoke }) => invoke("log_frontend_error", { message }).catch(() => {}))
    .catch(() => {});
}

function getStored<T>(key: string, fallback: T): T {
  try {
    const val = localStorage.getItem(`localhost_${key}`);
    return val ? JSON.parse(val) : fallback;
  } catch {
    return fallback;
  }
}

function setStored<T>(key: string, val: T): void {
  try {
    localStorage.setItem(`localhost_${key}`, JSON.stringify(val));
  } catch {
    // Ignore storage quota
  }
}

async function handleBrowserMock(cmd: string, args?: Record<string, unknown>): Promise<unknown> {
  // Simulate small async network tick
  await new Promise((r) => setTimeout(r, 40));

  switch (cmd) {
    case "trending":
    case "popular": {
      return [...MOCK_ANIME].sort((a, b) => (b.popularity ?? 0) - (a.popularity ?? 0));
    }

    case "current_season_info": {
      return MOCK_SEASON_INFO;
    }

    case "season": {
      const year = Number(args?.year ?? 2026);
      const season = String(args?.season ?? "WINTER").toUpperCase();
      const filtered = MOCK_ANIME.filter(
        (a) => (a.seasonYear === year && a.season?.toUpperCase() === season) || !a.seasonYear,
      );
      return filtered.length > 0 ? filtered : MOCK_ANIME.slice(0, 10);
    }

    case "browse": {
      const filters = (args?.filters ?? {}) as Record<string, unknown>;
      const q = String(filters.search ?? filters.query ?? "").toLowerCase().trim();
      const rawGenres = filters.genres;
      const genre = String(
        (Array.isArray(rawGenres) ? rawGenres[0] : rawGenres) ?? filters.genre ?? "",
      );
      const format = String(filters.format ?? "");
      const status = String(filters.status ?? "");
      const year = Number(filters.year ?? filters.seasonYear ?? 0);
      const sort = String(filters.sort ?? "POPULARITY_DESC");

      let list = [...MOCK_ANIME];

      if (q) {
        list = list.filter(
          (a) =>
            a.title.toLowerCase().includes(q) ||
            (a.titleEnglish && a.titleEnglish.toLowerCase().includes(q)) ||
            a.genres.some((g) => g.toLowerCase().includes(q)),
        );
      }
      if (genre) {
        list = list.filter((a) => a.genres.includes(genre));
      }
      if (format) {
        list = list.filter((a) => a.format === format);
      }
      if (status) {
        list = list.filter((a) => a.status === status);
      }
      if (year) {
        list = list.filter((a) => a.seasonYear === year);
      }

      if (sort === "SCORE_DESC") {
        list.sort((a, b) => (b.averageScore ?? 0) - (a.averageScore ?? 0));
      } else if (sort === "START_DATE_DESC") {
        list.sort((a, b) => (b.seasonYear ?? 0) - (a.seasonYear ?? 0));
      } else if (sort === "FAVORITES_DESC") {
        list.sort((a, b) => (b.popularity ?? 0) - (a.popularity ?? 0));
      } else {
        list.sort((a, b) => (b.popularity ?? 0) - (a.popularity ?? 0));
      }

      const res: BrowsePage = {
        cards: list,
        hasNextPage: false,
        total: list.length,
      };
      return res;
    }

    case "anime_details": {
      const id = Number(args?.id);
      const found = MOCK_ANIME.find((a) => a.id === id) ?? MOCK_ANIME[0];
      return found;
    }

    case "anime_characters": {
      return [
        {
          id: 1,
          name: "Main Protagonist",
          image: "https://images.unsplash.com/photo-1578632767115-351597cf2477?w=300&auto=format&fit=crop&q=80",
          role: "MAIN",
          voiceActor: "Aoi Yuuki",
        },
        {
          id: 2,
          name: "Deuteragonist",
          image: "https://images.unsplash.com/photo-1563089145-599997674d42?w=300&auto=format&fit=crop&q=80",
          role: "MAIN",
          voiceActor: "Takahiro Sakurai",
        },
        {
          id: 3,
          name: "Key Ally",
          image: "https://images.unsplash.com/photo-1607604276583-eef5d076aa5f?w=300&auto=format&fit=crop&q=80",
          role: "SUPPORTING",
          voiceActor: "Kenjiro Tsuda",
        },
      ];
    }

    case "anime_episodes": {
      const animeId = Number(args?.animeId);
      const anime = MOCK_ANIME.find((a) => a.id === animeId) ?? MOCK_ANIME[0];
      return {
        malId: anime.malId ?? anime.id,
        animeTitle: anime.title,
        episodes: generateEpisodes(anime),
        source: "crunchyroll-hls",
      };
    }

    case "add_to_watchlist": {
      const card = args?.card as AnimeCard;
      const listStatus = String(args?.listStatus ?? "watching");
      const list = getStored<WatchlistItem[]>("watchlist", []);
      const existing = list.findIndex((x) => x.animeId === card.id);
      const item: WatchlistItem = {
        animeId: card.id,
        malId: card.malId,
        title: card.title,
        cover: card.cover,
        banner: card.banner,
        color: card.color,
        episodesTotal: card.episodes,
        status: card.status,
        listStatus,
        addedAt: Date.now(),
        progress: 0,
      };
      if (existing >= 0) list[existing] = item;
      else list.unshift(item);
      setStored("watchlist", list);
      return;
    }

    case "remove_from_watchlist": {
      const id = Number(args?.animeId);
      const list = getStored<WatchlistItem[]>("watchlist", []).filter((x) => x.animeId !== id);
      setStored("watchlist", list);
      return;
    }

    case "set_list_status": {
      const id = Number(args?.animeId);
      const status = String(args?.listStatus ?? "watching");
      const list = getStored<WatchlistItem[]>("watchlist", []);
      const item = list.find((x) => x.animeId === id);
      if (item) item.listStatus = status;
      setStored("watchlist", list);
      return;
    }

    case "get_watchlist": {
      const filter = args?.listStatus ? String(args.listStatus) : null;
      let list = getStored<WatchlistItem[]>("watchlist", []);
      if (list.length === 0) {
        // Provide sample items for preview
        list = MOCK_ANIME.slice(0, 3).map((a, i) => ({
          animeId: a.id,
          malId: a.malId,
          title: a.title,
          cover: a.cover,
          banner: a.banner,
          color: a.color,
          episodesTotal: a.episodes,
          status: a.status,
          listStatus: i === 0 ? "watching" : i === 1 ? "planning" : "completed",
          addedAt: Date.now(),
          progress: i === 0 ? 5 : i === 1 ? 0 : Number(a.episodes ?? 12),
        }));
        setStored("watchlist", list);
      }
      if (filter) list = list.filter((x) => x.listStatus === filter);
      return list;
    }

    case "in_watchlist": {
      const id = Number(args?.animeId);
      const list = getStored<WatchlistItem[]>("watchlist", []);
      return list.some((x) => x.animeId === id);
    }

    case "save_progress": {
      const id = Number(args?.animeId);
      const ep = Number(args?.episode);
      const pos = Number(args?.positionSec);
      const dur = Number(args?.durationSec);
      const progressList = getStored<ProgressEntry[]>(`progress_${id}`, []);
      const idx = progressList.findIndex((p) => p.episode === ep);
      const entry: ProgressEntry = {
        animeId: id,
        episode: ep,
        positionSec: pos,
        durationSec: dur,
        completed: dur > 0 && pos / dur > 0.88,
        updatedAt: Date.now(),
      };
      if (idx >= 0) progressList[idx] = entry;
      else progressList.push(entry);
      setStored(`progress_${id}`, progressList);
      return;
    }

    case "get_progress": {
      const id = Number(args?.animeId);
      return getStored<ProgressEntry[]>(`progress_${id}`, []);
    }

    case "continue_watching": {
      const list = getStored<WatchlistItem[]>("watchlist", []);
      return list.filter((x) => x.progress > 0 && x.listStatus === "watching");
    }

    case "resume_point": {
      return 0;
    }

    case "list_library_folders": {
      return getStored<string[]>("library_folders", ["/Users/local/Anime"]);
    }

    case "add_library_folder": {
      const path = String(args?.path ?? "");
      const folders = getStored<string[]>("library_folders", []);
      if (path && !folders.includes(path)) folders.push(path);
      setStored("library_folders", folders);
      return;
    }

    case "remove_library_folder": {
      const path = String(args?.path ?? "");
      const folders = getStored<string[]>("library_folders", []).filter((f) => f !== path);
      setStored("library_folders", folders);
      return;
    }

    case "scan_library": {
      return [
        { title: "Frieren: Beyond Journey's End", episodeCount: 28 },
        { title: "Solo Leveling", episodeCount: 12 },
        { title: "Chainsaw Man", episodeCount: 12 },
      ];
    }

    case "local_episodes": {
      const title = String(args?.title ?? "Anime");
      return [
        { number: 1, title: `${title} - Episode 01.mkv`, path: `/Anime/${title}/01.mkv` },
        { number: 2, title: `${title} - Episode 02.mkv`, path: `/Anime/${title}/02.mkv` },
      ];
    }

    case "get_setting": {
      const key = String(args?.key ?? "");
      return getStored<string | null>(`setting_${key}`, null);
    }

    case "set_setting": {
      const key = String(args?.key ?? "");
      const value = String(args?.value ?? "");
      setStored(`setting_${key}`, value);
      return;
    }

    default:
      return null;
  }
}
