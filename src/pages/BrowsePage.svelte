<script lang="ts">
  import { onMount } from "svelte";
  import { api } from "../lib/api";
  import { reportError } from "../lib/ipc";
  import type { AnimeCard } from "../lib/types";
  import AnimeCardItem from "../components/AnimeCard.svelte";

  const GENRES = [
    "Action", "Adventure", "Comedy", "Drama", "Fantasy", "Horror", "Mecha",
    "Music", "Mystery", "Psychological", "Romance", "Sci-Fi", "Slice of Life",
    "Sports", "Supernatural", "Thriller",
  ];
  const FORMATS = [
    { value: "TV", label: "Series" },
    { value: "MOVIE", label: "Movie" },
    { value: "ONA", label: "ONA" },
    { value: "OVA", label: "OVA" },
    { value: "SPECIAL", label: "Special" },
  ];
  const STATUSES = [
    { value: "RELEASING", label: "Airing" },
    { value: "FINISHED", label: "Complete" },
    { value: "NOT_YET_RELEASED", label: "Upcoming" },
  ];
  const SORTS = [
    { value: "POPULARITY_DESC", label: "Popularity" },
    { value: "SCORE_DESC", label: "Score" },
    { value: "START_DATE_DESC", label: "Newest" },
    { value: "FAVOURITES_DESC", label: "Favorites" },
  ];

  let query = $state("");
  let genres: string[] = $state([]);
  let format = $state("");
  let status = $state("");
  let sort = $state("POPULARITY_DESC");
  let year = $state<number | "">("");

  let cards: AnimeCard[] = $state([]);
  let page = $state(1);
  let hasNext = $state(false);
  let loading = $state(false);
  let firstLoad = $state(true);
  let error = $state("");
  let sentinel: HTMLDivElement | undefined = $state();
  let debounceTimer: ReturnType<typeof setTimeout> | undefined;
  let lastQuery = $state("");

  onMount(() => {
    readHashQuery();
    window.addEventListener("hashchange", readHashQuery);
    load(true);
    return () => window.removeEventListener("hashchange", readHashQuery);
  });

  function readHashQuery() {
    const q = window.location.hash.split("?")[1];
    if (q) {
      const params = new URLSearchParams(q);
      const queryParam = params.get("q");
      if (queryParam && queryParam !== query) {
        query = queryParam;
        load(true);
      }
    }
  }

  async function load(reset: boolean) {
    if (loading) return;
    loading = true;
    error = "";
    try {
      const p = reset ? 1 : page + 1;
      const result = await api.browse({
        search: query.trim() || null,
        genres: genres.length ? genres : null,
        format: format || null,
        status: status || null,
        sort,
        year: year === "" ? null : Number(year),
        page: p,
      });
      cards = reset ? result.cards : [...cards, ...result.cards];
      page = p;
      hasNext = result.hasNextPage;
      firstLoad = false;
      lastQuery = query;
    } catch (e) {
      error = String(e);
      reportError("BrowsePage.load", e);
    } finally {
      loading = false;
    }
  }

  function onSearchInput() {
    clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => load(true), 500);
  }

  function toggleGenre(g: string) {
    genres = genres.includes(g) ? genres.filter((x) => x !== g) : [...genres, g];
    load(true);
  }

  function toggleFormat(f: string) {
    format = format === f ? "" : f;
    load(true);
  }

  function toggleStatus(s: string) {
    status = status === s ? "" : s;
    load(true);
  }

  function setYear(e: Event) {
    const v = (e.target as HTMLSelectElement).value;
    year = v === "" ? "" : Number(v);
    load(true);
  }

  function setSort(e: Event) {
    sort = (e.target as HTMLSelectElement).value;
    load(true);
  }

  $effect(() => {
    if (sentinel && hasNext && !loading) {
      const obs = new IntersectionObserver(
        (entries) => {
          if (entries[0].isIntersecting) load(false);
        },
        { rootMargin: "500px" },
      );
      obs.observe(sentinel);
      return () => obs.disconnect();
    }
  });
</script>

<div class="page">
  <div class="browse-header">
    <h1 class="page-title">{query.trim() ? `Results for “${lastQuery || query.trim()}”` : "Browse All"}</h1>
    <p class="page-sub">{firstLoad ? "Loading catalog…" : `${cards.length}${hasNext ? "+" : ""} series`}</p>
  </div>

  <div class="filter-bar">
    <div class="search-wrap">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" class="search-icon">
        <circle cx="11" cy="11" r="8" />
        <line x1="21" y1="21" x2="16.65" y2="16.65" />
      </svg>
      <input
        class="search-input"
        bind:value={query}
        oninput={onSearchInput}
        placeholder="Filter by title…"
        spellcheck="false"
      />
    </div>

    <div class="selects-wrap">
      <select class="pill-select" onchange={setYear} value={String(year)}>
        <option value="">Any Year</option>
        {#each Array.from({ length: 36 }, (_, i) => new Date().getFullYear() + 1 - i) as y}
          <option value={String(y)}>{y}</option>
        {/each}
      </select>

      <select class="pill-select" onchange={setSort} value={sort}>
        {#each SORTS as s}
          <option value={s.value}>{s.label}</option>
        {/each}
      </select>
    </div>
  </div>

  <div class="chip-row">
    {#each FORMATS as f}
      <button class="chip" class:active={format === f.value} onclick={() => toggleFormat(f.value)}>
        {f.label}
      </button>
    {/each}
    <span class="chip-divider"></span>
    {#each STATUSES as s}
      <button class="chip" class:active={status === s.value} onclick={() => toggleStatus(s.value)}>
        {s.label}
      </button>
    {/each}
  </div>

  <div class="chip-row genres-row">
    {#each GENRES as g}
      <button class="chip" class:active={genres.includes(g)} onclick={() => toggleGenre(g)}>
        {g}
      </button>
    {/each}
  </div>

  {#if error}
    <div class="error-box">
      <span>{error}</span>
      <button class="btn secondary" onclick={() => load(true)}>Retry</button>
    </div>
  {/if}

  {#if firstLoad && loading}
    <div class="spinner"></div>
  {:else}
    <div class="grid">
      {#each cards as card (card.id)}
        <AnimeCardItem {card} />
      {/each}
    </div>

    {#if !loading && cards.length === 0}
      <div class="empty-state">
        <div class="big">∅</div>
        <p>No anime found matching your filters.</p>
        <p class="hint">Try removing some genres or searching with a different term.</p>
      </div>
    {/if}

    {#if hasNext}
      <div bind:this={sentinel} class="sentinel">
        {#if loading}
          <div class="spinner small"></div>
        {/if}
      </div>
    {/if}
  {/if}
</div>

<style>
  .browse-header {
    margin-bottom: 8px;
  }

  .filter-bar {
    display: flex;
    gap: 12px;
    margin-bottom: 18px;
    align-items: center;
    flex-wrap: wrap;
  }

  .search-wrap {
    position: relative;
    flex: 1;
    min-width: 260px;
  }

  .search-icon {
    position: absolute;
    left: 14px;
    top: 50%;
    transform: translateY(-50%);
    width: 15px;
    height: 15px;
    color: var(--text-faint);
    pointer-events: none;
  }

  .search-input {
    width: 100%;
    padding: 10px 18px 10px 40px;
    border-radius: 999px;
    background: var(--surface-2);
    border: 1px solid var(--border);
    font-size: 13.5px;
  }

  .search-input:focus {
    border-color: var(--accent);
    background: var(--surface-3);
  }

  .selects-wrap {
    display: flex;
    gap: 10px;
  }

  .pill-select {
    border-radius: 999px;
    background: var(--surface-2);
    border: 1px solid var(--border);
    padding: 9px 18px;
    font-size: 13px;
    font-weight: 600;
  }

  .pill-select:focus {
    border-color: var(--accent);
    background: var(--surface-3);
  }

  .chip-row {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
    margin-bottom: 12px;
  }

  .genres-row {
    margin-bottom: 28px;
  }

  .chip-divider {
    width: 1px;
    background: var(--border);
    margin: 2px 6px;
    height: 24px;
    align-self: center;
  }

  .sentinel {
    height: 80px;
  }

  .spinner.small {
    margin: 24px auto;
    width: 28px;
    height: 28px;
  }
</style>
