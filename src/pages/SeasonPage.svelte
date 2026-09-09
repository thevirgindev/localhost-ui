<script lang="ts">
  import { onMount } from "svelte";
  import { api } from "../lib/api";
  import { reportError } from "../lib/ipc";
  import type { AnimeCard, SeasonInfo } from "../lib/types";
  import AnimeCardItem from "../components/AnimeCard.svelte";

  interface SeasonTab {
    season: string;
    year: number;
    label: string;
    isCurrent?: boolean;
  }

  let info: SeasonInfo | null = $state(null);
  let activeSeason = $state("WINTER");
  let activeYear = $state(2026);
  let cards: AnimeCard[] = $state([]);
  let loading = $state(true);
  let error = $state("");

  // Filters within the season
  let filterType = $state<"all" | "new" | "continuing">("all");
  let selectedGenre = $state("");
  let sortBy = $state<"popular" | "score" | "title">("popular");
  let searchQuery = $state("");

  // Archive modal / dropdown toggle
  let showArchivePicker = $state(false);
  let archiveYear = $state(2025);
  let archiveSeason = $state("FALL");

  const recentTabs = $derived.by<SeasonTab[]>(() => {
    return [
      { season: "WINTER", year: 2026, label: "Winter 2026", isCurrent: true },
      { season: "FALL", year: 2025, label: "Fall 2025" },
      { season: "SUMMER", year: 2025, label: "Summer 2025" },
      { season: "SPRING", year: 2025, label: "Spring 2025" },
      { season: "WINTER", year: 2025, label: "Winter 2025" },
    ];
  });

  const genresList = [
    "Action",
    "Adventure",
    "Comedy",
    "Drama",
    "Fantasy",
    "Music",
    "Mystery",
    "Psychological",
    "Romance",
    "Sci-Fi",
    "Slice of Life",
    "Sports",
    "Supernatural",
  ];

  onMount(async () => {
    try {
      info = await api.seasonInfo();
      if (info) {
        activeSeason = info.season;
        activeYear = info.year;
      }
      await load();
    } catch (e) {
      error = String(e);
      reportError("SeasonPage.init", e);
      loading = false;
    }
  });

  async function load() {
    loading = true;
    error = "";
    try {
      cards = await api.season(activeYear, activeSeason, 1);
    } catch (e) {
      error = String(e);
      reportError("SeasonPage.load", e);
    } finally {
      loading = false;
    }
  }

  function selectTab(tab: SeasonTab) {
    activeSeason = tab.season;
    activeYear = tab.year;
    showArchivePicker = false;
    load();
  }

  function applyArchive() {
    activeSeason = archiveSeason;
    activeYear = archiveYear;
    showArchivePicker = false;
    load();
  }

  const filteredCards = $derived.by(() => {
    let list = [...cards];

    if (searchQuery.trim()) {
      const q = searchQuery.trim().toLowerCase();
      list = list.filter(
        (c) =>
          c.title.toLowerCase().includes(q) ||
          (c.titleEnglish && c.titleEnglish.toLowerCase().includes(q)) ||
          c.genres.some((g) => g.toLowerCase().includes(q)),
      );
    }

    if (selectedGenre) {
      list = list.filter((c) => c.genres.includes(selectedGenre));
    }

    if (filterType === "new") {
      list = list.filter((c) => !c.title.includes("Season") && !c.title.includes("2nd") && !c.title.includes("Part"));
    } else if (filterType === "continuing") {
      list = list.filter((c) => c.title.includes("Season") || c.title.includes("2nd") || c.title.includes("Part") || c.status === "RELEASING");
    }

    if (sortBy === "score") {
      list.sort((a, b) => (b.averageScore ?? 0) - (a.averageScore ?? 0));
    } else if (sortBy === "title") {
      list.sort((a, b) => a.title.localeCompare(b.title));
    } else {
      list.sort((a, b) => (b.popularity ?? 0) - (a.popularity ?? 0));
    }

    return list;
  });

  const activeSeasonLabel = $derived.by(() => {
    const s = activeSeason.charAt(0) + activeSeason.slice(1).toLowerCase();
    return `${s} ${activeYear}`;
  });
</script>

<div class="season-page">
  <!-- Page Header matching Crunchyroll's Seasonal Lineup -->
  <header class="page-header">
    <div class="header-inner">
      <div class="header-text">
        <h1 class="page-title">Seasonal Anime</h1>
        <p class="page-subtitle">
          Explore fresh releases and returning favorites broadcasting in {activeSeasonLabel}.
        </p>
      </div>

      <div class="header-stats">
        {#if !loading}
          <span class="count-badge">{filteredCards.length} Titles</span>
        {/if}
      </div>
    </div>

    <!-- Season Navigation Tabs (Crunchyroll style) -->
    <nav class="season-tabs-bar">
      <div class="tabs-scroll">
        {#each recentTabs as tab}
          <button
            class="season-tab-btn"
            class:active={activeSeason === tab.season && activeYear === tab.year}
            onclick={() => selectTab(tab)}
          >
            <span class="tab-label">{tab.label}</span>
            {#if tab.isCurrent}
              <span class="current-pill">NOW AIRING</span>
            {/if}
            {#if activeSeason === tab.season && activeYear === tab.year}
              <span class="tab-indicator"></span>
            {/if}
          </button>
        {/each}

        <button
          class="season-tab-btn archive-toggle"
          class:active={showArchivePicker}
          onclick={() => (showArchivePicker = !showArchivePicker)}
        >
          <span>More Seasons <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" width="12" height="12" stroke-linecap="round" stroke-linejoin="round"><polyline points="6 9 12 15 18 9"/></svg></span>
        </button>
      </div>

      {#if showArchivePicker}
        <div class="archive-dropdown">
          <div class="archive-title">Browse Archive</div>
          <div class="archive-controls">
            <select bind:value={archiveSeason}>
              <option value="WINTER">Winter</option>
              <option value="SPRING">Spring</option>
              <option value="SUMMER">Summer</option>
              <option value="FALL">Fall</option>
            </select>
            <select bind:value={archiveYear}>
              {#each [2026, 2025, 2024, 2023, 2022, 2021, 2020] as y}
                <option value={y}>{y}</option>
              {/each}
            </select>
            <button class="cr-orange-pill sm" onclick={applyArchive}>Apply</button>
          </div>
        </div>
      {/if}
    </nav>
  </header>

  <!-- Filter & Sort Sub-bar -->
  <section class="filter-toolbar">
    <div class="filter-left">
      <!-- Sub-tabs: All / New / Continuing -->
      <div class="segmented-pills">
        <button
          class="seg-btn"
          class:active={filterType === "all"}
          onclick={() => (filterType = "all")}
        >
          All
        </button>
        <button
          class="seg-btn"
          class:active={filterType === "new"}
          onclick={() => (filterType = "new")}
        >
          New Series
        </button>
        <button
          class="seg-btn"
          class:active={filterType === "continuing"}
          onclick={() => (filterType = "continuing")}
        >
          Continuing
        </button>
      </div>

      <!-- Genre Dropdown -->
      <div class="select-wrapper">
        <select bind:value={selectedGenre}>
          <option value="">All Genres</option>
          {#each genresList as g}
            <option value={g}>{g}</option>
          {/each}
        </select>
      </div>
    </div>

    <div class="filter-right">
      <!-- Search Input within Season -->
      <div class="search-input-wrap">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="search-icon">
          <circle cx="11" cy="11" r="8" />
          <line x1="21" y1="21" x2="16.65" y2="16.65" />
        </svg>
        <input
          type="text"
          bind:value={searchQuery}
          placeholder="Filter {activeSeasonLabel}..."
          spellcheck="false"
        />
        {#if searchQuery}
          <button class="clear-icon" onclick={() => (searchQuery = "")}><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" width="13" height="13" stroke-linecap="round"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg></button>
        {/if}
      </div>

      <!-- Sort Dropdown -->
      <div class="select-wrapper">
        <select bind:value={sortBy}>
          <option value="popular">Most Popular</option>
          <option value="score">Top Rated</option>
          <option value="title">Alphabetical (A–Z)</option>
        </select>
      </div>
    </div>
  </section>

  <!-- Content Grid -->
  <main class="season-content">
    {#if loading}
      <div class="loading-box">
        <div class="spinner"></div>
      </div>
    {:else if error}
      <div class="error-box">
        <span>{error}</span>
        <button class="cr-orange-pill sm" onclick={load}>Retry</button>
      </div>
    {:else if filteredCards.length === 0}
      <div class="empty-state">
        <div class="empty-icon">∅</div>
        <div class="empty-title">No anime found matching your filter</div>
        <p class="empty-desc">Try clearing your filters or selecting another season above.</p>
        <button
          class="cr-outline-pill sm"
          onclick={() => {
            filterType = "all";
            selectedGenre = "";
            searchQuery = "";
          }}
        >
          Reset Filters
        </button>
      </div>
    {:else}
      <div class="cr-grid">
        {#each filteredCards as card (card.id)}
          <AnimeCardItem {card} />
        {/each}
      </div>
    {/if}
  </main>
</div>

<style>
  .season-page {
    min-height: 100vh;
    padding-top: var(--nav-h);
    padding-bottom: 80px;
    background: var(--bg);
  }

  .page-header {
    background: #141519;
    border-bottom: 1px solid var(--border);
  }

  .header-inner {
    max-width: 1600px;
    margin: 0 auto;
    padding: 36px 44px 20px;
    display: flex;
    align-items: flex-end;
    justify-content: space-between;
    gap: 20px;
  }

  .page-title {
    font-size: 34px;
    font-weight: 800;
    letter-spacing: -0.025em;
    color: var(--text);
    margin: 0 0 6px;
  }

  .page-subtitle {
    font-size: 14px;
    color: var(--text-dim);
    margin: 0;
    max-width: 600px;
  }

  .count-badge {
    font-size: 12px;
    font-weight: 700;
    color: var(--text-dim);
    background: rgba(255, 255, 255, 0.06);
    padding: 6px 14px;
    border-radius: 999px;
    border: 1px solid var(--border);
  }

  /* Horizontal season tabs bar matching Crunchyroll */
  .season-tabs-bar {
    position: relative;
    max-width: 1600px;
    margin: 0 auto;
    padding: 0 44px;
  }

  .tabs-scroll {
    display: flex;
    align-items: center;
    gap: 4px;
    overflow-x: auto;
    scrollbar-width: none;
  }

  .tabs-scroll::-webkit-scrollbar {
    display: none;
  }

  .season-tab-btn {
    position: relative;
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 16px 18px;
    font-size: 13.5px;
    font-weight: 700;
    letter-spacing: 0.05em;
    text-transform: uppercase;
    color: var(--text-dim);
    background: transparent;
    transition: color 0.12s ease;
    white-space: nowrap;
  }

  .season-tab-btn:hover {
    color: var(--text);
  }

  .season-tab-btn.active {
    color: var(--text);
  }

  .tab-indicator {
    position: absolute;
    bottom: 0;
    left: 8px;
    right: 8px;
    height: 3px;
    background: var(--accent);
    border-radius: 2px 2px 0 0;
  }

  .current-pill {
    font-size: 9.5px;
    font-weight: 800;
    letter-spacing: 0.06em;
    background: rgba(168, 85, 247, 0.2);
    color: var(--accent);
    padding: 2px 6px;
    border-radius: 999px;
    border: 1px solid rgba(168, 85, 247, 0.4);
  }

  .archive-toggle {
    color: var(--text-faint);
  }

  .archive-dropdown {
    position: absolute;
    right: 44px;
    top: calc(100% + 4px);
    z-index: 50;
    background: #1e2025;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    padding: 16px;
    box-shadow: 0 12px 32px rgba(0, 0, 0, 0.7);
  }

  .archive-title {
    font-size: 12px;
    font-weight: 800;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: var(--text-dim);
    margin-bottom: 10px;
  }

  .archive-controls {
    display: flex;
    gap: 8px;
    align-items: center;
  }

  .archive-controls select {
    background: var(--surface-2);
    color: var(--text);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    padding: 6px 10px;
    font-size: 13px;
  }

  /* Filter toolbar */
  .filter-toolbar {
    max-width: 1600px;
    margin: 0 auto;
    padding: 24px 44px;
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
  }

  .filter-left,
  .filter-right {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  /* Segmented pills */
  .segmented-pills {
    display: inline-flex;
    background: #1a1c21;
    padding: 3px;
    border-radius: 999px;
    border: 1px solid var(--border);
  }

  .seg-btn {
    padding: 6px 14px;
    font-size: 12.5px;
    font-weight: 700;
    color: var(--text-dim);
    border-radius: 999px;
    transition: all 0.14s ease;
    white-space: nowrap;
  }

  .seg-btn:hover {
    color: var(--text);
  }

  .seg-btn.active {
    background: var(--accent);
    color: #0d0d0d;
  }

  /* Select dropdown wrappers */
  .select-wrapper select {
    background: #1a1c21;
    color: var(--text);
    border: 1px solid var(--border);
    border-radius: 999px;
    padding: 7px 16px;
    font-size: 12.5px;
    font-weight: 600;
    outline: none;
    cursor: pointer;
    transition: border-color 0.12s ease;
  }

  .select-wrapper select:hover {
    border-color: var(--text-faint);
  }

  .select-wrapper select:focus {
    border-color: var(--accent);
  }

  /* Search input inside season */
  .search-input-wrap {
    position: relative;
    display: flex;
    align-items: center;
  }

  .search-icon {
    position: absolute;
    left: 12px;
    width: 14px;
    height: 14px;
    color: var(--text-faint);
    pointer-events: none;
  }

  .search-input-wrap input {
    background: #1a1c21;
    color: var(--text);
    border: 1px solid var(--border);
    border-radius: 999px;
    padding: 7px 30px 7px 34px;
    font-size: 12.5px;
    width: 180px;
    transition: width 0.2s ease, border-color 0.12s ease;
  }

  .search-input-wrap input:focus {
    width: 240px;
    border-color: var(--accent);
  }

  .clear-icon {
    position: absolute;
    right: 10px;
    color: var(--text-faint);
    font-size: 11px;
    padding: 2px;
  }

  /* Main content grid */
  .season-content {
    max-width: 1600px;
    margin: 0 auto;
    padding: 0 44px;
  }

  .cr-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(185px, 1fr));
    gap: 24px 18px;
  }

  .loading-box {
    display: flex;
    justify-content: center;
    padding: 80px 0;
  }

  .empty-state {
    text-align: center;
    padding: 80px 20px;
  }

  .empty-icon {
    font-size: 36px;
    color: var(--text-faint);
    margin-bottom: 12px;
  }

  .empty-title {
    font-size: 18px;
    font-weight: 700;
    color: var(--text);
    margin-bottom: 6px;
  }

  .empty-desc {
    font-size: 13.5px;
    color: var(--text-dim);
    margin-bottom: 20px;
  }

  .cr-orange-pill {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    background: var(--accent);
    color: #0d0d0d;
    font-weight: 700;
    font-size: 13px;
    padding: 8px 18px;
    border-radius: 999px;
    transition: background 0.12s ease;
  }

  .cr-orange-pill.sm {
    padding: 6px 14px;
    font-size: 12px;
  }

  .cr-orange-pill:hover {
    background: var(--accent-hover);
  }

  .cr-outline-pill {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    color: var(--text);
    border: 1px solid var(--border);
    font-weight: 700;
    font-size: 13px;
    padding: 8px 18px;
    border-radius: 999px;
    transition: all 0.12s ease;
  }

  .cr-outline-pill.sm {
    padding: 6px 14px;
    font-size: 12px;
  }

  .cr-outline-pill:hover {
    border-color: var(--text-dim);
    background: rgba(255, 255, 255, 0.05);
  }
</style>
