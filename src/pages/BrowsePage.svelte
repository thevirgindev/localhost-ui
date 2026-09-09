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

  const SORTS = [
    { value: "POPULARITY_DESC", label: "Popular" },
    { value: "START_DATE_DESC", label: "Newest" },
    { value: "SCORE_DESC", label: "Top Rated" },
    { value: "FAVOURITES_DESC", label: "Favorites" },
  ];

  const FORMATS = [
    { value: "", label: "All Formats" },
    { value: "TV", label: "Series" },
    { value: "MOVIE", label: "Movie" },
    { value: "ONA", label: "ONA" },
    { value: "OVA", label: "OVA" },
    { value: "SPECIAL", label: "Special" },
  ];

  const STATUSES = [
    { value: "", label: "All Statuses" },
    { value: "RELEASING", label: "Currently Airing" },
    { value: "FINISHED", label: "Completed" },
    { value: "NOT_YET_RELEASED", label: "Upcoming" },
  ];

  let query = $state("");
  let selectedGenre = $state("");
  let format = $state("");
  let status = $state("");
  let sort = $state("POPULARITY_DESC");
  let year = $state<number | "">("");
  let gridLayout = $state<"2x2" | "4x4" | "dense">("4x4");

  let cards: AnimeCard[] = $state([]);
  let page = $state(1);
  let hasNext = $state(false);
  let loading = $state(false);
  let firstLoad = $state(true);
  let error = $state("");
  let sentinel: HTMLDivElement | undefined = $state();
  let debounceTimer: ReturnType<typeof setTimeout> | undefined;

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
      if (queryParam !== null && queryParam !== query) {
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
        genres: selectedGenre ? [selectedGenre] : null,
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
    } catch (e) {
      error = String(e);
      reportError("BrowsePage.load", e);
    } finally {
      loading = false;
    }
  }

  function onSearchInput() {
    clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => load(true), 400);
  }

  function clearSearch() {
    query = "";
    load(true);
  }

  function clearAllFilters() {
    query = "";
    selectedGenre = "";
    format = "";
    status = "";
    year = "";
    sort = "POPULARITY_DESC";
    load(true);
  }

  const hasActiveFilters = $derived(
    query.trim() !== "" || selectedGenre !== "" || format !== "" || status !== "" || year !== "",
  );

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
  <div class="explore-top">
    <div>
      <h1 class="page-title">Explore</h1>
    </div>

    <!-- Crunchyroll-style segmented sorting tabs -->
    <div class="sort-tabs">
      {#each SORTS as s}
        <button
          class="sort-tab"
          class:active={sort === s.value}
          onclick={() => {
            if (sort !== s.value) {
              sort = s.value;
              load(true);
            }
          }}
        >
          {s.label}
        </button>
      {/each}
    </div>
  </div>

  <!-- Clean, unified filter bar -->
  <div class="filter-toolbar">
    <div class="search-box">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" class="s-icon">
        <circle cx="11" cy="11" r="8" />
        <line x1="21" y1="21" x2="16.65" y2="16.65" />
      </svg>
      <input
        class="search-input"
        bind:value={query}
        oninput={onSearchInput}
        placeholder="Filter by title..."
        spellcheck="false"
      />
      {#if query}
        <button class="clear-btn" onclick={clearSearch} title="Clear search">✕</button>
      {/if}
    </div>

    <div class="selects-group">
      <!-- Genre Select -->
      <select
        class="filter-select"
        class:highlighted={selectedGenre !== ""}
        bind:value={selectedGenre}
        onchange={() => load(true)}
      >
        <option value="">All Genres</option>
        {#each GENRES as g}
          <option value={g}>{g}</option>
        {/each}
      </select>

      <!-- Format Select -->
      <select
        class="filter-select"
        class:highlighted={format !== ""}
        bind:value={format}
        onchange={() => load(true)}
      >
        {#each FORMATS as f}
          <option value={f.value}>{f.label}</option>
        {/each}
      </select>

      <!-- Status Select -->
      <select
        class="filter-select"
        class:highlighted={status !== ""}
        bind:value={status}
        onchange={() => load(true)}
      >
        {#each STATUSES as st}
          <option value={st.value}>{st.label}</option>
        {/each}
      </select>

      <!-- Year Select -->
      <select
        class="filter-select"
        class:highlighted={year !== ""}
        bind:value={year}
        onchange={() => load(true)}
      >
        <option value="">All Years</option>
        {#each Array.from({ length: 36 }, (_, i) => new Date().getFullYear() + 1 - i) as y}
          <option value={y}>{y}</option>
        {/each}
      </select>

      <!-- Grid density picker: 2x2 / 4x4 / Dense -->
      <div class="grid-density-picker">
        <button
          class="density-btn"
          class:active={gridLayout === "2x2"}
          onclick={() => (gridLayout = "2x2")}
          title="2x2 Large Showcase Grid"
        >
          2×2
        </button>
        <button
          class="density-btn"
          class:active={gridLayout === "4x4"}
          onclick={() => (gridLayout = "4x4")}
          title="4x4 Standard Catalog Grid"
        >
          4×4
        </button>
        <button
          class="density-btn"
          class:active={gridLayout === "dense"}
          onclick={() => (gridLayout = "dense")}
          title="Dense Dynamic Grid"
        >
          Dense
        </button>
      </div>
    </div>
  </div>

  <!-- Active filter badges (only shown when filtered) -->
  {#if hasActiveFilters}
    <div class="active-filters-row">
      <span class="active-label">Filters:</span>
      {#if query.trim()}
        <button class="active-tag" onclick={clearSearch}>
          "{query.trim()}" ✕
        </button>
      {/if}
      {#if selectedGenre}
        <button class="active-tag" onclick={() => { selectedGenre = ""; load(true); }}>
          Genre: {selectedGenre} ✕
        </button>
      {/if}
      {#if format}
        <button class="active-tag" onclick={() => { format = ""; load(true); }}>
          Format: {FORMATS.find((f) => f.value === format)?.label} ✕
        </button>
      {/if}
      {#if status}
        <button class="active-tag" onclick={() => { status = ""; load(true); }}>
          Status: {STATUSES.find((s) => s.value === status)?.label} ✕
        </button>
      {/if}
      {#if year !== ""}
        <button class="active-tag" onclick={() => { year = ""; load(true); }}>
          Year: {year} ✕
        </button>
      {/if}
      <button class="clear-all-btn" onclick={clearAllFilters}>Clear all</button>
    </div>
  {/if}

  {#if error}
    <div class="error-box">
      <span>{error}</span>
      <button class="btn secondary" onclick={() => load(true)}>Retry</button>
    </div>
  {/if}

  {#if firstLoad && loading}
    <div class="spinner"></div>
  {:else}
    <div class="grid layout-{gridLayout}">
      {#each cards as card (card.id)}
        <AnimeCardItem {card} />
      {/each}
    </div>

    {#if !loading && cards.length === 0}
      <div class="empty-state">
        <div class="big">∅</div>
        <p>No titles found matching those criteria.</p>
        <button class="btn secondary" style="margin-top: 14px;" onclick={clearAllFilters}>Reset Filters</button>
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
  .explore-top {
    display: flex;
    align-items: flex-end;
    justify-content: space-between;
    gap: 20px;
    margin-bottom: 20px;
    flex-wrap: wrap;
  }

  .sort-tabs {
    display: flex;
    gap: 4px;
    background: var(--surface);
    border: 1px solid var(--border);
    padding: 3px;
    border-radius: 999px;
  }

  .sort-tab {
    padding: 7px 18px;
    border-radius: 999px;
    font-size: 13px;
    font-weight: 600;
    color: var(--text-dim);
    transition: all 0.13s ease;
  }

  .sort-tab:hover {
    color: var(--text);
  }

  .sort-tab.active {
    background: var(--accent);
    color: #0d0d0d;
    font-weight: 700;
  }

  .filter-toolbar {
    display: flex;
    gap: 12px;
    margin-bottom: 16px;
    align-items: center;
    flex-wrap: wrap;
  }

  .search-box {
    position: relative;
    flex: 1;
    min-width: 260px;
  }

  .s-icon {
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
    padding: 10px 38px 10px 40px;
    border-radius: 4px;
    background: var(--surface);
    border: 1px solid var(--border);
    font-size: 13.5px;
    color: var(--text);
  }

  .search-input:focus {
    border-color: var(--accent);
    background: var(--surface-2);
  }

  .clear-btn {
    position: absolute;
    right: 12px;
    top: 50%;
    transform: translateY(-50%);
    width: 20px;
    height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-dim);
    font-size: 11px;
    border-radius: 4px;
  }

  .clear-btn:hover {
    color: var(--text);
  }

  .selects-group {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
    align-items: center;
  }

  .filter-select {
    border-radius: 4px;
    background: var(--surface);
    border: 1px solid var(--border);
    padding: 9px 14px;
    font-size: 13px;
    font-weight: 600;
    color: var(--text);
  }

  .filter-select:focus {
    border-color: var(--accent);
  }

  .filter-select.highlighted {
    border-color: var(--accent);
    color: var(--accent);
  }

  .grid-density-picker {
    display: flex;
    align-items: center;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 4px;
    padding: 2px;
    gap: 2px;
  }

  .density-btn {
    padding: 6px 12px;
    font-size: 12px;
    font-weight: 700;
    border-radius: 3px;
    background: transparent;
    border: none;
    color: var(--text-dim);
    cursor: pointer;
    transition: all 0.12s ease;
  }

  .density-btn:hover {
    color: #ffffff;
  }

  .density-btn.active {
    background: var(--accent);
    color: #0d0d0d;
    font-weight: 800;
  }

  /* Grid layouts */
  .grid.layout-2x2 {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 36px 28px;
  }

  .grid.layout-4x4 {
    display: grid;
    grid-template-columns: repeat(4, minmax(0, 1fr));
    gap: 28px 20px;
  }

  .grid.layout-dense {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(185px, 1fr));
    gap: 24px 16px;
  }

  @media (max-width: 1200px) {
    .grid.layout-4x4 {
      grid-template-columns: repeat(3, minmax(0, 1fr));
    }
  }

  @media (max-width: 768px) {
    .grid.layout-2x2,
    .grid.layout-4x4 {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }
  }

  .active-filters-row {
    display: flex;
    align-items: center;
    gap: 8px;
    flex-wrap: wrap;
    margin-bottom: 24px;
    padding-top: 4px;
  }

  .active-label {
    font-size: 12px;
    font-weight: 700;
    color: var(--text-faint);
    text-transform: uppercase;
    letter-spacing: 0.08em;
  }

  .active-tag {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 4px 12px;
    border-radius: 999px;
    background: var(--surface-2);
    border: 1px solid var(--border);
    color: var(--accent);
    font-size: 12px;
    font-weight: 700;
    cursor: pointer;
    transition: all 0.12s ease;
  }

  .active-tag:hover {
    border-color: var(--accent);
    background: var(--surface-3);
  }

  .clear-all-btn {
    font-size: 12px;
    font-weight: 700;
    color: var(--text-faint);
    margin-left: 4px;
    text-decoration: underline;
    cursor: pointer;
  }

  .clear-all-btn:hover {
    color: var(--accent);
  }

  .sentinel {
    height: 80px;
  }

  .spinner.small {
    margin: 20px auto;
    width: 28px;
    height: 28px;
  }
</style>
