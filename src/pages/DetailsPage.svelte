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
  let source = $state("");
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
      source = src.source;
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
      // characters are optional polish; failures stay silent
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
    if (card.episodes) parts.push(`${card.episodes} episodes`);
    if (card.duration) parts.push(`${card.duration}m`);
    if (card.start_date) parts.push(card.start_date.slice(0, 4));
    return parts.join(" · ");
  });

  const infoRows = $derived.by(() => {
    const c = card;
    if (!c) return [] as { label: string; value: string }[];
    const rows: { label: string; value: string }[] = [];
    if (c.format) rows.push({ label: "Type", value: c.format === "MOVIE" ? "Movie" : c.format });
    if (c.episodes) rows.push({ label: "Episodes", value: String(c.episodes) });
    if (c.status) rows.push({ label: "Status", value: c.status });
    if (c.aired_string) rows.push({ label: "Aired", value: c.aired_string });
    if (c.broadcast) rows.push({ label: "Broadcast", value: c.broadcast });
    if (c.studio) rows.push({ label: "Studio", value: c.studio });
    if (c.rating) rows.push({ label: "Rating", value: c.rating });
    return rows;
  });
</script>

<div class="details-page">
  {#if loading}
    <div class="spinner"></div>
  {:else if error || !card}
    <div class="page">
      <div class="error-box"><span>{error || "Not found"}</span><button class="btn secondary" onclick={() => history.back()}>Back</button></div>
    </div>
  {:else}
    <div class="backdrop">
      {#if card.cover}
        <div class="backdrop-img" style={`background-image: url('${card.cover}')`}></div>
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
          {#if card.title_native}
            <div class="native">{card.title_native}</div>
          {/if}
          <div class="meta-line">{metaLine}</div>
          <div class="genre-row">
            {#each card.genres.slice(0, 5) as g}
              <a class="genre-pill" href={`#/browse?q=${encodeURIComponent(g)}`}>{g}</a>
            {/each}
          </div>
          <div class="actions">
            <button class="btn primary watch-btn" onclick={() => router.navigate({ name: "watch", id, episode: firstUnwatched() })}>
              <svg viewBox="0 0 24 24" fill="currentColor" width="16" height="16"><path d="M8 5v14l11-7z" /></svg>
              {lastCompleted > 0 ? `Resume EP ${firstUnwatched()}` : "Start Watching"}
            </button>

            {#if inList}
              <div class="in-list">
                <svg viewBox="0 0 24 24" fill="currentColor" width="15" height="15"><path d="M9 16.2L4.8 12l-1.4 1.4L9 19 21 7l-1.4-1.4L9 16.2z" /></svg>
                <select
                  onchange={async (e) => {
                    const v = (e.target as HTMLSelectElement).value;
                    if (v === "__remove") await removeFromList();
                    else await api.setListStatus(id, v);
                  }}
                >
                  {#each LIST_STATUSES as s}
                    <option value={s.value} selected={s.value === (listStatus || "planning")}>{s.label}</option>
                  {/each}
                  <option value="__remove">Remove from My List</option>
                </select>
              </div>
            {:else}
              <button class="btn secondary" onclick={() => addToList("planning")}>
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.6" width="15" height="15"><line x1="12" y1="5" x2="12" y2="19" /><line x1="5" y1="12" x2="19" y2="12" /></svg>
                My List
              </button>
            {/if}

            {#if card.trailer_id && card.trailer_site === "youtube"}
              <button class="btn outline" onclick={() => window.open(`https://www.youtube.com/watch?v=${card!.trailer_id}`, "_blank")}>
                Trailer
              </button>
            {/if}
          </div>
        </div>
      </div>

      <div class="tabs">
        <button class="tab" class:active={tab === "episodes"} onclick={() => (tab = "episodes")}>Episodes</button>
        <button class="tab" class:active={tab === "characters"} onclick={() => (tab = "characters")}>Characters</button>
        <button class="tab" class:active={tab === "info"} onclick={() => (tab = "info")}>Information</button>
      </div>

      {#if tab === "episodes"}
        {#if episodes.length === 0}
          <div class="empty-state">
            <p>No episode metadata found.</p>
            <p class="hint">Add the show's folder in Library to play local files.</p>
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
                    <span class="ep-progress-fill" style={`width: ${Math.round((prog.position_sec / Math.max(1, prog.duration_sec)) * 100)}%`}></span>
                  </span>
                {:else if prog?.completed}
                  <span class="ep-flag done">Watched</span>
                {/if}
                <svg class="ep-play" viewBox="0 0 24 24" fill="currentColor"><path d="M8 5v14l11-7z" /></svg>
              </button>
            {/each}
          </div>
        {/if}
      {:else if tab === "characters"}
        {#if characters.length === 0}
          <div class="empty-state"><p>No character data.</p></div>
        {:else}
          <div class="char-grid">
            {#each characters as ch (ch.id)}
              <div class="char-card">
                <div class="char-img">
                  {#if ch.image}
                    <img src={ch.image} alt={ch.name} loading="lazy" />
                  {/if}
                </div>
                <div class="char-name">{ch.name}</div>
                <div class="char-role">{ch.role}{ch.voice_actor ? ` · ${ch.voice_actor}` : ""}</div>
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
            {#if card.genres.length || card.themes.length}
              <section>
                <h3 class="block-title">Genres & Themes</h3>
                <div class="chip-row">
                  {#each card.genres as g}
                    <a class="mono-chip" href={`#/browse?q=${encodeURIComponent(g)}`}>{g}</a>
                  {/each}
                  {#each card.themes as t}
                    <a class="mono-chip" href={`#/browse?q=${encodeURIComponent(t)}`}>{t}</a>
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
    height: 56vh;
    min-height: 400px;
    background: var(--surface);
  }

  .backdrop-img {
    position: absolute;
    inset: 0;
    background-size: cover;
    background-position: center 24%;
  }

  .backdrop-shade {
    position: absolute;
    inset: 0;
    background:
      linear-gradient(to top, #0d0d0d 4%, rgba(13, 13, 13, 0.45) 55%, rgba(13, 13, 13, 0.1)),
      linear-gradient(to right, rgba(13, 13, 13, 0.75), transparent 55%);
  }

  .head-wrap {
    margin-top: -140px;
    position: relative;
    z-index: 2;
  }

  .head {
    display: flex;
    gap: 34px;
    align-items: flex-end;
  }

  .poster {
    width: 225px;
    min-width: 225px;
    aspect-ratio: 2 / 3;
    border-radius: var(--radius-lg, 10px);
    overflow: hidden;
    background: var(--surface);
    box-shadow: var(--shadow);
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
    font-size: 34px;
    line-height: 1.08;
    letter-spacing: -0.025em;
    margin-bottom: 4px;
    text-shadow: 0 2px 14px rgba(0, 0, 0, 0.75);
  }

  .native {
    color: var(--text-dim);
    font-size: 13.5px;
    margin-bottom: 10px;
  }

  .meta-line {
    font-size: 13.5px;
    font-weight: 700;
    color: #ececee;
    margin-bottom: 14px;
    text-shadow: 0 1px 8px rgba(0, 0, 0, 0.65);
  }

  .genre-row {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
    margin-bottom: 20px;
  }

  .genre-pill {
    padding: 6px 15px;
    border-radius: 999px;
    background: rgba(28, 28, 30, 0.9);
    font-size: 11.5px;
    font-weight: 700;
    color: var(--text-dim);
  }

  .genre-pill:hover {
    background: var(--surface-3);
    color: var(--text);
  }

  .actions {
    display: flex;
    gap: 10px;
    align-items: center;
    flex-wrap: wrap;
  }

  .watch-btn {
    padding: 12px 28px;
  }

  .in-list {
    display: flex;
    align-items: center;
    gap: 8px;
    background: var(--surface-2);
    border-radius: 999px;
    padding: 6px 8px 6px 16px;
  }

  .in-list svg {
    color: var(--accent);
  }

  .in-list select {
    background: transparent;
    border: none;
    font-weight: 700;
    padding: 4px 2px;
  }

  /* tabs */
  .tabs {
    display: flex;
    gap: 26px;
    border-bottom: 1px solid var(--border);
    margin: 34px 0 20px;
  }

  .tab {
    padding: 12px 2px;
    font-size: 14px;
    font-weight: 700;
    color: var(--text-faint);
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
    max-width: 980px;
  }

  .ep-row {
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 13px 16px;
    background: var(--surface);
    border-radius: var(--radius-sm);
    text-align: left;
    transition: background 0.12s ease;
  }

  .ep-row:hover {
    background: var(--surface-2);
  }

  .ep-row.watched {
    opacity: 0.5;
  }

  .ep-num {
    font-family: var(--mono);
    font-size: 13px;
    font-weight: 700;
    color: var(--accent);
    min-width: 30px;
  }

  .ep-main {
    flex: 1;
    min-width: 0;
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .ep-title {
    font-size: 13.5px;
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
    letter-spacing: 0.5px;
  }

  .ep-flag.filler {
    color: #e8c268;
  }

  .ep-flag.done {
    color: var(--green);
  }

  .ep-progress-track {
    width: 90px;
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
    width: 15px;
    height: 15px;
    color: var(--text-faint);
    flex-shrink: 0;
  }

  .ep-row:hover .ep-play {
    color: var(--accent);
  }

  /* characters */
  .char-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(130px, 1fr));
    gap: 18px;
  }

  .char-img {
    aspect-ratio: 3 / 4;
    border-radius: var(--radius);
    overflow: hidden;
    background: var(--surface);
    margin-bottom: 8px;
  }

  .char-img img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .char-name {
    font-size: 12.5px;
    font-weight: 700;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
  }

  .char-role {
    font-family: var(--mono);
    font-size: 10.5px;
    color: var(--text-faint);
    margin-top: 2px;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
  }

  /* info tab */
  .info-layout {
    display: grid;
    grid-template-columns: 1fr 320px;
    gap: 48px;
  }

  .block-title {
    font-size: 16px;
    margin-bottom: 12px;
  }

  .info-main section + section {
    margin-top: 28px;
  }

  .synopsis {
    color: var(--text-dim);
    line-height: 1.68;
    margin: 0;
    white-space: pre-line;
  }

  .chip-row {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
  }

  .info-row {
    display: flex;
    justify-content: space-between;
    gap: 16px;
    padding: 10px 0;
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
