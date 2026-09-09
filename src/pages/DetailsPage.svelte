<script lang="ts">
  import { onMount } from "svelte";
  import { api } from "../lib/api";
  import { reportError } from "../lib/ipc";
  import type { AnimeCard, EpisodeEntry, ListStatus, ProgressEntry } from "../lib/types";
  import { router } from "../lib/router";
  import { formatDesc } from "../lib/format";

  interface CharacterCard {
    id: number;
    name: string;
    image: string | null;
    role: string;
    voiceActor: string | null;
  }

  let { id }: { id: number } = $props();

  let card: AnimeCard | null = $state(null);
  let episodes: EpisodeEntry[] = $state([]);
  let characters: CharacterCard[] = $state([]);
  let inList = $state(false);
  let listStatus = $state<ListStatus | "">("");
  let progress: ProgressEntry[] = $state([]);
  let loading = $state(true);
  let error = $state("");
  let tab = $state<"episodes" | "characters" | "info">("episodes");

  onMount(async () => {
    try {
      const c = await api.details(id);
      card = c;
      [inList, progress] = await Promise.all([api.inWatchlist(id), api.progress(id)]);
      const src = await api.episodes(id, c.title, c.titleEnglish);
      episodes = src.episodes;
      apiDetailsExtras(id);
    } catch (e) {
      error = String(e);
      reportError("DetailsPage.load", e);
    } finally {
      loading = false;
    }
  });

  async function apiDetailsExtras(animeId: number) {
    try {
      characters = await api.characters(animeId);
    } catch {
      // Optional characters polish
    }
  }

  const LIST_STATUSES: { value: ListStatus; label: string }[] = [
    { value: "watching", label: "Watching" },
    { value: "planning", label: "Want to Watch" },
    { value: "completed", label: "Completed" },
    { value: "paused", label: "Paused" },
    { value: "dropped", label: "Dropped" },
  ];

  async function addToList(status: ListStatus) {
    if (!card) return;
    await api.addToWatchlist(card, status);
    inList = true;
    listStatus = status;
  }

  async function removeFromList() {
    await api.removeFromWatchlist(id);
    inList = false;
    listStatus = "";
  }

  function epProgress(n: number): ProgressEntry | undefined {
    return progress.find((p) => p.episode === n);
  }

  function firstUnwatched(): number {
    const watched = new Set(progress.filter((p) => p.completed).map((p) => p.episode));
    let n = 1;
    while (watched.has(n)) n++;
    return n;
  }

  const lastCompleted = $derived(
    progress.filter((p) => p.completed).reduce((max, p) => Math.max(max, p.episode), 0),
  );

  const metaLine = $derived.by(() => {
    if (!card) return "";
    const parts: string[] = [];
    if (card.averageScore) parts.push(`★ ${(card.averageScore / 10).toFixed(1)}`);
    if (card.episodes) parts.push(`${card.episodes} ${card.episodes === 1 ? "episode" : "episodes"}`);
    if (card.duration) parts.push(`${card.duration}m`);
    if (card.startDate) parts.push(card.startDate.slice(0, 4));
    else if (card.seasonYear) parts.push(String(card.seasonYear));
    return parts.join(" · ");
  });

  const infoRows = $derived.by(() => {
    const c = card;
    if (!c) return [] as { label: string; value: string }[];
    const rows: { label: string; value: string }[] = [];
    if (c.format) rows.push({ label: "Type", value: c.format === "MOVIE" ? "Movie" : c.format });
    if (c.episodes) rows.push({ label: "Episodes", value: String(c.episodes) });
    if (c.duration) rows.push({ label: "Episode Duration", value: `${c.duration} min` });
    if (c.status) rows.push({ label: "Status", value: c.status === "RELEASING" ? "Currently Airing" : c.status });
    if (c.season || c.seasonYear) rows.push({ label: "Season", value: [c.season, c.seasonYear].filter(Boolean).join(" ") });
    if (c.studio) rows.push({ label: "Studio", value: c.studio });
    if (c.averageScore) rows.push({ label: "Score", value: `${c.averageScore}%` });
    return rows;
  });
</script>

<div class="details-page">
  {#if loading}
    <div class="spinner"></div>
  {:else if error || !card}
    <div class="page">
      <div class="error-box">
        <span>{error || "Anime details not found"}</span>
        <button class="btn secondary" onclick={() => history.back()}>Back</button>
      </div>
    </div>
  {:else}
    <div class="backdrop">
      {#if card.banner || card.cover}
        <div
          class="backdrop-img"
          style={`background-image: url('${card.banner || card.cover}')`}
        ></div>
      {/if}
      <div class="backdrop-shade"></div>
    </div>

    <div class="page head-wrap">
      <div class="head">
        <div class="poster">
          {#if card.cover}
            <img src={card.cover} alt={card.title} />
          {/if}
        </div>

        <div class="head-info">
          <h1>{card.title}</h1>
          {#if card.titleNative}
            <div class="native">{card.titleNative}</div>
          {/if}

          <div class="meta-line">{metaLine}</div>

          <div class="genre-row">
            {#each card.genres.slice(0, 6) as g}
              <a class="genre-pill" href={`#/browse?q=${encodeURIComponent(g)}`}>{g}</a>
            {/each}
          </div>

          <div class="actions">
            <button
              class="btn primary watch-btn"
              onclick={() => router.navigate({ name: "watch", id, episode: firstUnwatched() })}
            >
              <svg viewBox="0 0 24 24" fill="currentColor" width="16" height="16">
                <path d="M8 5v14l11-7z" />
              </svg>
              {lastCompleted > 0 ? `Resume EP ${firstUnwatched()}` : "Start Watching"}
            </button>

            {#if inList}
              <div class="in-list">
                <svg viewBox="0 0 24 24" fill="currentColor" width="16" height="16">
                  <path d="M9 16.2L4.8 12l-1.4 1.4L9 19 21 7l-1.4-1.4L9 16.2z" />
                </svg>
                <select
                  class="in-list-select"
                  onchange={async (e) => {
                    const v = (e.target as HTMLSelectElement).value;
                    if (v === "__remove") await removeFromList();
                    else await api.setListStatus(id, v);
                  }}
                >
                  {#each LIST_STATUSES as s}
                    <option value={s.value} selected={s.value === (listStatus || "planning")}>
                      {s.label}
                    </option>
                  {/each}
                  <option value="__remove">Remove from My List</option>
                </select>
              </div>
            {:else}
              <button class="btn secondary" onclick={() => addToList("planning")}>
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.6" width="15" height="15">
                  <line x1="12" y1="5" x2="12" y2="19" />
                  <line x1="5" y1="12" x2="19" y2="12" />
                </svg>
                My List
              </button>
            {/if}

            {#if card.trailerId && card.trailerSite === "youtube"}
              <a
                class="btn outline"
                href={`https://www.youtube.com/watch?v=${card.trailerId}`}
                target="_blank"
                rel="noreferrer"
              >
                Trailer
              </a>
            {/if}
          </div>
        </div>
      </div>

      <div class="tabs">
        <button class="tab" class:active={tab === "episodes"} onclick={() => (tab = "episodes")}>
          Episodes
        </button>
        <button class="tab" class:active={tab === "characters"} onclick={() => (tab = "characters")}>
          Characters
        </button>
        <button class="tab" class:active={tab === "info"} onclick={() => (tab = "info")}>
          Information
        </button>
      </div>

      {#if tab === "episodes"}
        {#if episodes.length === 0}
          <div class="empty-state">
            <div class="big">🎬</div>
            <p>No episodes found for this title.</p>
            <p class="hint">Add local media in Library if you have files stored locally.</p>
          </div>
        {:else}
          <div class="ep-list">
            {#each episodes as ep (ep.number)}
              {@const prog = epProgress(ep.number)}
              <button
                class="ep-row"
                class:watched={prog?.completed}
                onclick={() => router.navigate({ name: "watch", id, episode: ep.number })}
              >
                <span class="ep-num">{ep.number}</span>
                <span class="ep-main">
                  <span class="ep-title">{ep.title}</span>
                  {#if ep.filler}
                    <span class="ep-flag filler">Filler</span>
                  {/if}
                </span>

                {#if prog && !prog.completed}
                  <span class="ep-progress-track">
                    <span
                      class="ep-progress-fill"
                      style={`width: ${Math.round((prog.positionSec / Math.max(1, prog.durationSec)) * 100)}%`}
                    ></span>
                  </span>
                {:else if prog?.completed}
                  <span class="ep-flag done">Watched</span>
                {/if}
                <svg class="ep-play" viewBox="0 0 24 24" fill="currentColor">
                  <path d="M8 5v14l11-7z" />
                </svg>
              </button>
            {/each}
          </div>
        {/if}
      {:else if tab === "characters"}
        {#if characters.length === 0}
          <div class="empty-state">
            <div class="big">👤</div>
            <p>No character information available.</p>
          </div>
        {:else}
          <div class="char-grid">
            {#each characters as ch (ch.id)}
              <div class="char-card">
                <div class="char-img">
                  {#if ch.image}
                    <img src={ch.image} alt={ch.name} loading="lazy" />
                  {/if}
                </div>
                <div class="char-name" title={ch.name}>{ch.name}</div>
                <div class="char-role">{ch.role}{ch.voiceActor ? ` · ${ch.voiceActor}` : ""}</div>
              </div>
            {/each}
          </div>
        {/if}
      {:else}
        <div class="info-layout">
          <div class="info-main">
            <section>
              <h3 class="block-title">Synopsis</h3>
              <p class="synopsis">{formatDesc(card.description) || "No synopsis available."}</p>
            </section>
            {#if card.genres.length}
              <section>
                <h3 class="block-title">Genres</h3>
                <div class="genre-pill-row">
                  {#each card.genres as g}
                    <a class="genre-pill" href={`#/browse?q=${encodeURIComponent(g)}`}>{g}</a>
                  {/each}
                </div>
              </section>
            {/if}
          </div>
          <aside class="info-side">
            <h3 class="block-title">Information</h3>
            {#each infoRows as row}
              <div class="info-row">
                <span class="info-label">{row.label}</span>
                <span class="info-value">{row.value}</span>
              </div>
            {/each}
          </aside>
        </div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .details-page {
    padding-bottom: 70px;
  }

  .backdrop {
    position: relative;
    height: 58vh;
    min-height: 420px;
    max-height: 540px;
    background: var(--surface);
    overflow: hidden;
  }

  .backdrop-img {
    position: absolute;
    inset: -20px;
    background-size: cover;
    background-position: center 25%;
    filter: blur(24px);
    opacity: 0.55;
    transform: scale(1.08);
  }

  .backdrop-shade {
    position: absolute;
    inset: 0;
    background:
      linear-gradient(to top, #0d0d0d 6%, rgba(13, 13, 13, 0.5) 45%, rgba(13, 13, 13, 0.15) 85%),
      linear-gradient(to right, rgba(13, 13, 13, 0.85) 0%, rgba(13, 13, 13, 0.4) 50%, transparent 80%);
  }

  .head-wrap {
    margin-top: -160px;
    position: relative;
    z-index: 2;
  }

  .head {
    display: flex;
    gap: 36px;
    align-items: flex-end;
  }

  .poster {
    width: 230px;
    min-width: 230px;
    aspect-ratio: 2 / 3;
    border-radius: var(--radius);
    overflow: hidden;
    background: var(--surface-2);
    box-shadow: 0 16px 36px rgba(0, 0, 0, 0.7);
    flex-shrink: 0;
  }

  .poster img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .head-info {
    flex: 1;
    min-width: 0;
    padding-bottom: 4px;
  }

  h1 {
    font-size: 36px;
    line-height: 1.08;
    letter-spacing: -0.025em;
    margin-bottom: 6px;
    text-shadow: 0 2px 14px rgba(0, 0, 0, 0.8);
    color: var(--text);
  }

  .native {
    color: var(--text-dim);
    font-size: 14px;
    margin-bottom: 12px;
  }

  .meta-line {
    font-size: 14px;
    font-weight: 700;
    color: #e2e2e5;
    margin-bottom: 16px;
    text-shadow: 0 1px 8px rgba(0, 0, 0, 0.7);
  }

  .genre-row {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
    margin-bottom: 22px;
  }

  .genre-pill {
    padding: 6px 16px;
    border-radius: 999px;
    background: var(--surface-2);
    font-size: 12px;
    font-weight: 700;
    color: var(--text-dim);
    transition: background 0.13s ease, color 0.13s ease;
  }

  .genre-pill:hover {
    background: var(--surface-3);
    color: var(--text);
  }

  .actions {
    display: flex;
    gap: 12px;
    align-items: center;
    flex-wrap: wrap;
  }

  .watch-btn {
    padding: 12px 28px;
    font-size: 14px;
  }

  .in-list {
    display: flex;
    align-items: center;
    gap: 8px;
    background: var(--surface-2);
    border-radius: 999px;
    padding: 6px 12px 6px 16px;
  }

  .in-list svg {
    color: var(--accent);
    flex-shrink: 0;
  }

  .in-list-select {
    background: transparent;
    border: none;
    font-weight: 700;
    padding: 4px 2px;
    font-size: 13px;
    color: var(--text);
  }

  /* tabs */
  .tabs {
    display: flex;
    gap: 32px;
    border-bottom: 1px solid var(--border);
    margin: 36px 0 24px;
  }

  .tab {
    padding: 12px 4px;
    font-size: 15px;
    font-weight: 700;
    color: var(--text-dim);
    border-bottom: 3px solid transparent;
    margin-bottom: -1px;
    transition: color 0.13s ease, border-color 0.13s ease;
  }

  .tab:hover {
    color: var(--text);
  }

  .tab.active {
    color: var(--text);
    border-bottom-color: var(--accent);
  }

  /* episode rows */
  .ep-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
    max-width: 1040px;
  }

  .ep-row {
    display: flex;
    align-items: center;
    gap: 18px;
    padding: 14px 20px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    text-align: left;
    transition: background 0.12s ease, border-color 0.12s ease;
    cursor: pointer;
  }

  .ep-row:hover {
    background: var(--surface-2);
    border-color: var(--surface-3);
  }

  .ep-row.watched {
    opacity: 0.55;
  }

  .ep-num {
    font-family: var(--mono);
    font-size: 14px;
    font-weight: 800;
    color: var(--accent);
    min-width: 32px;
  }

  .ep-main {
    flex: 1;
    min-width: 0;
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .ep-title {
    font-size: 14px;
    font-weight: 600;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
  }

  .ep-row:hover .ep-title {
    color: var(--accent-hover);
  }

  .ep-flag {
    font-size: 10px;
    font-weight: 800;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    padding: 2px 7px;
    border-radius: 3px;
  }

  .ep-flag.filler {
    background: #382d13;
    color: #ffd166;
  }

  .ep-flag.done {
    background: #142e1d;
    color: var(--green);
  }

  .ep-progress-track {
    width: 100px;
    height: 4px;
    border-radius: 2px;
    background: var(--surface-3);
    overflow: hidden;
    flex-shrink: 0;
  }

  .ep-progress-fill {
    display: block;
    height: 100%;
    background: var(--accent);
  }

  .ep-play {
    width: 16px;
    height: 16px;
    color: var(--text-faint);
    flex-shrink: 0;
  }

  .ep-row:hover .ep-play {
    color: var(--accent);
  }

  /* characters */
  .char-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
    gap: 20px;
  }

  .char-card {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    padding: 10px;
  }

  .char-img {
    aspect-ratio: 3 / 4;
    border-radius: var(--radius-sm);
    overflow: hidden;
    background: var(--surface-2);
    margin-bottom: 10px;
  }

  .char-img img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .char-name {
    font-size: 13px;
    font-weight: 700;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
  }

  .char-role {
    font-family: var(--mono);
    font-size: 11px;
    color: var(--text-faint);
    margin-top: 2px;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
  }

  /* info tab */
  .info-layout {
    display: grid;
    grid-template-columns: 1fr 340px;
    gap: 48px;
  }

  .block-title {
    font-size: 17px;
    letter-spacing: -0.01em;
    margin-bottom: 14px;
  }

  .info-main section + section {
    margin-top: 32px;
  }

  .synopsis {
    color: var(--text-dim);
    line-height: 1.7;
    margin: 0;
    white-space: pre-line;
    font-size: 14px;
  }

  .genre-pill-row {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
  }

  .info-side {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    padding: 24px;
    height: fit-content;
  }

  .info-row {
    display: flex;
    justify-content: space-between;
    gap: 16px;
    padding: 11px 0;
    border-bottom: 1px solid var(--border);
    font-size: 13px;
  }

  .info-row:last-child {
    border-bottom: none;
  }

  .info-label {
    color: var(--text-faint);
  }

  .info-value {
    font-weight: 600;
    text-align: right;
  }

  @media (max-width: 900px) {
    .info-layout {
      grid-template-columns: 1fr;
    }
  }
</style>
