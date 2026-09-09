<script lang="ts">
  import { onMount } from "svelte";
  import { userStore } from "../lib/userStore.svelte";
  import { api } from "../lib/api";
  import type { AnimeCard } from "../lib/types";

  let searchInput = $state<HTMLInputElement | null>(null);
  let query = $state("");
  let selectedCategory = $state("All");
  let results = $state<AnimeCard[]>([]);
  let loading = $state(false);
  let loadingMore = $state(false);
  let page = $state(1);
  let hasMore = $state(true);
  let debounceTimer: ReturnType<typeof setTimeout> | undefined;

  const categories = [
    "All",
    "Action",
    "Fantasy",
    "Shounen",
    "Romance",
    "Sci-Fi",
    "Comedy",
    "Drama",
  ];

  const trendingSearches = [
    "Attack on Titan",
    "Demon Slayer",
    "Jujutsu Kaisen",
    "One Piece",
    "Chainsaw Man",
    "Spy x Family",
    "Solo Leveling",
    "Bocchi the Rock",
  ];

  onMount(() => {
    // Focus search input on open
    setTimeout(() => {
      searchInput?.focus();
    }, 50);

    // Initial search or default popular items
    performSearch("");

    function handleKeydown(e: KeyboardEvent) {
      if (e.key === "Escape") {
        closeSearch();
      }
    }
    window.addEventListener("keydown", handleKeydown);
    return () => window.removeEventListener("keydown", handleKeydown);
  });

  function closeSearch() {
    userStore.showSearchModal = false;
  }

  function onInput() {
    clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
      performSearch(query);
    }, 200);
  }

  async function performSearch(term: string) {
    loading = true;
    page = 1;
    hasMore = true;
    try {
      const genre = selectedCategory === "All" ? undefined : selectedCategory;
      const resp = await api.browse({
        query: term.trim() || undefined,
        genre,
        page: 1,
        perPage: 12,
      });
      results = resp.cards;
      hasMore = resp.cards.length >= 12;
    } catch {
      results = [];
      hasMore = false;
    } finally {
      loading = false;
    }
  }

  async function loadMore() {
    if (loadingMore || !hasMore) return;
    loadingMore = true;
    const nextPage = page + 1;
    try {
      const genre = selectedCategory === "All" ? undefined : selectedCategory;
      const resp = await api.browse({
        query: query.trim() || undefined,
        genre,
        page: nextPage,
        perPage: 12,
      });
      if (resp.cards.length > 0) {
        // filter duplicates
        const existingIds = new Set(results.map((c) => c.id));
        const newCards = resp.cards.filter((c) => !existingIds.has(c.id));
        results = [...results, ...newCards];
        page = nextPage;
        hasMore = resp.cards.length >= 12;
      } else {
        hasMore = false;
      }
    } catch {
      hasMore = false;
    } finally {
      loadingMore = false;
    }
  }

  function selectCategory(cat: string) {
    selectedCategory = cat;
    performSearch(query);
  }

  function pickTrending(term: string) {
    query = term;
    performSearch(term);
    searchInput?.focus();
  }

  function openAnime(id: number) {
    closeSearch();
    window.location.hash = `#/details?id=${id}`;
  }

  function clearQuery() {
    query = "";
    performSearch("");
    searchInput?.focus();
  }
</script>

<!-- Backdrop -->
<div class="search-backdrop" onclick={closeSearch} role="button" tabindex="0" onkeydown={(e) => e.key === "Escape" && closeSearch()}>
  <!-- Centered Search Card -->
  <div
    class="search-card"
    onclick={(e) => e.stopPropagation()}
    role="dialog"
    aria-label="Universal Anime Search"
    tabindex="-1"
    onkeydown={(e) => e.key === "Escape" && closeSearch()}
  >
    <!-- Card Header / Search Input -->
    <div class="search-header">
      <div class="search-input-wrapper">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="search-icon">
          <circle cx="11" cy="11" r="8" />
          <line x1="21" y1="21" x2="16.65" y2="16.65" />
        </svg>
        <input
          bind:this={searchInput}
          bind:value={query}
          oninput={onInput}
          type="text"
          class="main-search-input"
          placeholder="Search anime titles, genres, studios, or characters..."
          spellcheck="false"
        />
        {#if query}
          <button class="clear-icon-btn" onclick={clearQuery} title="Clear text">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" width="14" height="14" stroke-linecap="round"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
          </button>
        {/if}
      </div>

      <div class="header-actions">
        <span class="esc-badge">ESC</span>
        <button class="close-btn" onclick={closeSearch} title="Close search">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" width="14" height="14" stroke-linecap="round"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
        </button>
      </div>
    </div>

    <!-- Quick Category Filter Chips -->
    <div class="category-chips">
      {#each categories as cat}
        <button
          class="cat-chip"
          class:active={selectedCategory === cat}
          onclick={() => selectCategory(cat)}
        >
          {cat}
        </button>
      {/each}
    </div>

    <!-- Main Results or Trending Body -->
    <div class="search-body">
      {#if !query.trim()}
        <!-- Trending Searches Bar -->
        <div class="trending-section">
          <span class="section-label">Trending Now</span>
          <div class="trending-pills">
            {#each trendingSearches as t}
              <button class="trending-pill" onclick={() => pickTrending(t)}>
                <span class="fire-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="13" height="13" stroke-linecap="round" stroke-linejoin="round"><path d="M8.5 14.5A2.5 2.5 0 0 0 11 12c0-1.38-.5-2-1-3-1.072-2.143-.224-4.054 2-6 .5 2.5 2 4.9 4 6.5 2 1.6 3 3.5 3 5.5a7 7 0 1 1-14 0c0-1.153.433-2.294 1-3a2.5 2.5 0 0 0 2.5 2.5z" /></svg></span>
                <span>{t}</span>
              </button>
            {/each}
          </div>
        </div>
      {/if}

      <!-- Results Grid -->
      <div class="results-header">
        <span class="results-title">
          {query.trim() ? `Search Results for "${query}"` : "Popular Anime on Luci"}
        </span>
        {#if loading}
          <span class="loading-tag">Searching...</span>
        {:else}
          <span class="count-tag">{results.length} titles</span>
        {/if}
      </div>

      {#if results.length === 0 && !loading}
        <div class="no-results">
          <p class="no-results-title">No anime found matching "{query}"</p>
          <p class="no-results-sub">Try searching by a broader keyword, alternative title, or choose another genre category.</p>
        </div>
      {:else}
        <div class="results-grid">
          {#each results as anime (anime.id)}
            <div
              class="result-item"
              onclick={() => openAnime(anime.id)}
              role="button"
              tabindex="0"
              onkeydown={(e) => e.key === "Enter" && openAnime(anime.id)}
            >
              <div class="result-cover">
                {#if anime.cover}
                  <img src={anime.cover} alt={anime.title} loading="lazy" />
                {:else}
                  <div class="no-cover"><svg viewBox="0 0 24 24" fill="currentColor" width="18" height="18"><polygon points="6 3 20 12 6 21 6 3"/></svg></div>
                {/if}
                <div class="play-overlay">
                  <span class="mini-play"><svg viewBox="0 0 24 24" fill="currentColor" width="12" height="12"><polygon points="6 3 20 12 6 21 6 3"/></svg></span>
                </div>
              </div>

              <div class="result-info">
                <div class="result-meta">
                  <span class="res-badge">{anime.format === "MOVIE" ? "Movie" : "Series"}</span>
                  {#if anime.averageScore}
                    <span class="res-score"><svg viewBox="0 0 24 24" fill="currentColor" width="11" height="11"><path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01z"/></svg> {anime.averageScore}%</span>
                  {/if}
                  <span class="res-episodes">{anime.episodes ? `${anime.episodes} eps` : "24 eps"}</span>
                </div>

                <h4 class="result-title" title={anime.title}>{anime.title}</h4>

                <div class="result-genres">
                  {#each (anime.genres || []).slice(0, 3) as g}
                    <span class="genre-tag">{g}</span>
                  {/each}
                </div>
              </div>
            </div>
          {/each}
        </div>

        {#if hasMore}
          <div class="load-more-wrap">
            <button class="load-more-btn" onclick={loadMore} disabled={loadingMore}>
              {#if loadingMore}
                <div class="spinner-inline"></div>
                <span>Loading more anime...</span>
              {:else}
                <span>Show More Anime <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" width="12" height="12" stroke-linecap="round" stroke-linejoin="round"><line x1="12" y1="5" x2="12" y2="19"/><polyline points="19 12 12 19 5 12"/></svg></span>
              {/if}
            </button>
          </div>
        {/if}
      {/if}
    </div>

    <!-- Bottom Footer in Search Card -->
    <div class="search-card-footer">
      <div class="tip-left">
        <kbd>↑</kbd> <kbd>↓</kbd> to navigate • <kbd>ENTER</kbd> to select • <kbd>ESC</kbd> to dismiss
      </div>
      <a class="view-all-link" href="#/browse" onclick={closeSearch}>
        Open Full Catalog in Explore <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" width="12" height="12" stroke-linecap="round" stroke-linejoin="round"><line x1="5" y1="12" x2="19" y2="12"/><polyline points="12 5 19 12 12 19"/></svg>
      </a>
    </div>
  </div>
</div>

<style>
  .search-backdrop {
    position: fixed;
    inset: 0;
    z-index: 2000;
    background: rgba(0, 0, 0, 0.82);
    backdrop-filter: blur(8px);
    -webkit-backdrop-filter: blur(8px);
    display: flex;
    justify-content: center;
    align-items: center;
    padding: 24px 20px;
    animation: fadeIn 0.18s ease-out;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  /* Centered Search Card */
  .search-card {
    width: 100%;
    max-width: 960px;
    max-height: min(88vh, 820px);
    background: #141519;
    border: 1px solid #282a32;
    border-radius: 10px;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    box-shadow: 0 24px 70px rgba(0, 0, 0, 0.95), 0 0 0 1px rgba(255, 255, 255, 0.05);
    animation: modalScalePop 0.22s cubic-bezier(0.16, 1, 0.3, 1);
  }

  @keyframes modalScalePop {
    from {
      transform: scale(0.96) translateY(-8px);
      opacity: 0;
    }
    to {
      transform: scale(1) translateY(0);
      opacity: 1;
    }
  }

  /* Search Header */
  .search-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 20px;
    border-bottom: 1px solid #23252b;
    gap: 16px;
    background: #181a20;
  }

  .search-input-wrapper {
    display: flex;
    align-items: center;
    gap: 12px;
    flex: 1;
  }

  .search-icon {
    width: 22px;
    height: 22px;
    color: var(--accent);
    flex-shrink: 0;
  }

  .main-search-input {
    width: 100%;
    background: transparent;
    border: none;
    outline: none;
    font-size: 17px;
    font-weight: 600;
    color: #ffffff;
    font-family: inherit;
  }

  .main-search-input::placeholder {
    color: #6e7078;
    font-weight: 400;
  }

  .clear-icon-btn {
    background: transparent;
    border: none;
    color: #8c8c92;
    cursor: pointer;
    font-size: 14px;
    padding: 4px;
    border-radius: 4px;
  }

  .clear-icon-btn:hover {
    color: #ffffff;
  }

  .header-actions {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .esc-badge {
    font-size: 11px;
    font-weight: 800;
    color: #8a8a92;
    background: #23252b;
    border: 1px solid #33363e;
    padding: 2px 6px;
    border-radius: 3px;
    letter-spacing: 0.05em;
  }

  .close-btn {
    width: 32px;
    height: 32px;
    border-radius: 4px;
    background: transparent;
    border: none;
    color: #a0a0a5;
    font-size: 16px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background 0.12s ease, color 0.12s ease;
  }

  .close-btn:hover {
    background: #282a32;
    color: #ffffff;
  }

  /* Category Chips */
  .category-chips {
    display: flex;
    gap: 8px;
    padding: 12px 20px;
    border-bottom: 1px solid #23252b;
    background: #141519;
    overflow-x: auto;
    scrollbar-width: none;
  }

  .cat-chip {
    padding: 5px 12px;
    font-size: 12.5px;
    font-weight: 700;
    color: #a0a0a8;
    background: #1d1f26;
    border: 1px solid #282a33;
    border-radius: 4px;
    cursor: pointer;
    white-space: nowrap;
    transition: all 0.12s ease;
  }

  .cat-chip:hover {
    color: #ffffff;
    background: #282a33;
  }

  .cat-chip.active {
    background: var(--accent);
    color: var(--accent-contrast, #000000);
    border-color: var(--accent);
  }

  /* Body Content */
  .search-body {
    flex: 1;
    overflow-y: auto;
    padding: 16px 20px;
    max-height: 480px;
  }

  .trending-section {
    display: flex;
    align-items: center;
    gap: 10px;
    margin-bottom: 18px;
    flex-wrap: wrap;
  }

  .section-label {
    font-size: 11px;
    font-weight: 900;
    color: #787a82;
    letter-spacing: 0.06em;
  }

  .trending-pills {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
  }

  .trending-pill {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    padding: 3px 9px;
    font-size: 12px;
    font-weight: 700;
    color: #cfcfd4;
    background: #1c1e25;
    border: 1px solid #2a2c35;
    border-radius: 4px;
    cursor: pointer;
    transition: background 0.12s ease, border-color 0.12s ease;
  }

  .trending-pill:hover {
    background: #252832;
    border-color: var(--accent);
    color: #ffffff;
  }

  .fire-icon {
    font-size: 11px;
  }

  .results-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 14px;
  }

  .results-title {
    font-size: 14px;
    font-weight: 800;
    color: #ffffff;
    text-transform: uppercase;
    letter-spacing: 0.03em;
  }

  .count-tag,
  .loading-tag {
    font-size: 12px;
    color: #888890;
    font-weight: 600;
  }

  .loading-tag {
    color: var(--accent);
  }

  /* Results Grid */
  .results-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
    gap: 12px;
  }

  .result-item {
    display: flex;
    gap: 12px;
    background: #1a1c22;
    border: 1px solid #23252e;
    border-radius: 4px;
    padding: 8px;
    cursor: pointer;
    transition: background 0.14s ease, border-color 0.14s ease, transform 0.14s ease;
  }

  .result-item:hover {
    background: #21242c;
    border-color: var(--accent);
    transform: translateY(-1px);
  }

  .result-cover {
    position: relative;
    width: 60px;
    height: 84px;
    border-radius: 3px;
    overflow: hidden;
    background: #111215;
    flex-shrink: 0;
  }

  .result-cover img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .no-cover {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--accent);
    font-size: 16px;
  }

  .play-overlay {
    position: absolute;
    inset: 0;
    background: rgba(0, 0, 0, 0.45);
    display: flex;
    align-items: center;
    justify-content: center;
    opacity: 0;
    transition: opacity 0.12s ease;
  }

  .result-item:hover .play-overlay {
    opacity: 1;
  }

  .mini-play {
    color: var(--accent);
    font-size: 14px;
  }

  .result-info {
    display: flex;
    flex-direction: column;
    justify-content: center;
    overflow: hidden;
    gap: 3px;
  }

  .result-meta {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 11px;
    font-weight: 700;
  }

  .res-badge {
    background: rgba(255, 255, 255, 0.1);
    color: #e0e0e5;
    padding: 1px 4px;
    border-radius: 2px;
    font-size: 10px;
    text-transform: uppercase;
  }

  .res-score {
    color: var(--accent);
  }

  .res-episodes {
    color: #8c8c92;
  }

  .result-title {
    margin: 0;
    font-size: 13.5px;
    font-weight: 800;
    color: #ffffff;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .result-genres {
    display: flex;
    gap: 4px;
    flex-wrap: wrap;
    margin-top: 2px;
  }

  .genre-tag {
    font-size: 10.5px;
    color: #888892;
    background: #141519;
    padding: 1px 5px;
    border-radius: 2px;
  }

  .no-results {
    text-align: center;
    padding: 48px 20px;
  }

  .no-results-title {
    font-size: 16px;
    font-weight: 800;
    color: #ffffff;
    margin-bottom: 6px;
  }

  .no-results-sub {
    font-size: 13px;
    color: #888892;
  }

  .load-more-wrap {
    display: flex;
    justify-content: center;
    padding: 16px 0 8px;
  }

  .load-more-btn {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 10px 24px;
    background: #20222a;
    border: 1px solid #323542;
    border-radius: 999px;
    color: #ffffff;
    font-size: 13px;
    font-weight: 700;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .load-more-btn:hover:not(:disabled) {
    background: var(--accent);
    color: var(--accent-contrast, #000000);
    border-color: var(--accent);
    transform: translateY(-1px);
  }

  .load-more-btn:disabled {
    opacity: 0.7;
    cursor: not-allowed;
  }

  .spinner-inline {
    width: 14px;
    height: 14px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-top-color: #ffffff;
    border-radius: 50%;
    animation: spin 0.7s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  /* Card Footer */
  .search-card-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 20px;
    border-top: 1px solid #23252b;
    background: #111215;
    font-size: 12px;
    color: #7a7a82;
  }

  kbd {
    background: #23252b;
    border: 1px solid #33363e;
    color: #cfcfd4;
    padding: 1px 5px;
    border-radius: 3px;
    font-family: inherit;
    font-size: 10px;
    font-weight: 700;
  }

  .view-all-link {
    color: var(--accent);
    text-decoration: none;
    font-weight: 700;
    transition: color 0.12s ease;
  }

  .view-all-link:hover {
    color: var(--accent-hover);
    text-decoration: underline;
  }
</style>
