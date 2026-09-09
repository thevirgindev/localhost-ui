// Type definitions shared across the frontend.

export interface AnimeCard {
  id: number;
  malId: number | null;
  title: string;
  titleEnglish: string | null;
  titleNative: string | null;
  cover: string | null;
  banner: string | null;
  color: string | null;
  format: string | null;
  episodes: number | null;
  duration: number | null;
  status: string | null;
  season: string | null;
  seasonYear: number | null;
  averageScore: number | null;
  popularity: number | null;
  genres: string[];
  description: string | null;
  startDate: string | null;
  studio: string | null;
  trailerSite: string | null;
  trailerId: string | null;
  nextAiringEpisode: number | null;
  nextAiringAt: number | null;
}

export type ListStatus = "watching" | "planning" | "completed" | "paused" | "dropped";

export interface BrowsePage {
  cards: AnimeCard[];
  hasNextPage: boolean;
  total: number | null;
}

export interface EpisodeEntry {
  number: number;
  title: string;
  url: string;
  aired: string | null;
  filler: boolean;
  recap: boolean;
}

export interface SourcesResult {
  malId: number;
  animeTitle: string;
  episodes: EpisodeEntry[];
  source: string;
}

export interface PlayableEpisode {
  malId: number;
  number: number;
  title: string;
  url: string;
  /** "hls" | "file" | "embed" */
  kind: string;
  /** Human label of the server that answered. */
  server: string;
}

export interface ProviderHealth {
  name: string;
  kind: string;
  ok: boolean;
  latencyMs: number;
  detail: string;
}

export interface WatchlistItem {
  animeId: number;
  malId: number | null;
  title: string;
  cover: string | null;
  banner: string | null;
  color: string | null;
  episodesTotal: number | null;
  status: string | null;
  listStatus: string;
  addedAt: number;
  progress: number;
}

export interface ProgressEntry {
  animeId: number;
  episode: number;
  positionSec: number;
  durationSec: number;
  completed: boolean;
  updatedAt: number;
}

export interface SeasonInfo {
  season: string;
  year: number;
  label: string;
}

export interface LocalItem {
  title: string;
  episodeCount: number;
}

export interface LocalEpisode {
  number: number;
  title: string;
  path: string;
}
