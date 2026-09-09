<script lang="ts">
  import { onMount } from "svelte";
  import { api } from "../lib/api";
  import { reportError } from "../lib/ipc";
  import type { LocalItem, WatchlistItem } from "../lib/types";
  import { router } from "../lib/router";

  type MainTab = "watchlist" | "continue" | "local";

  const LIST_TABS = [
    { value: "all", label: "All" },
    { value: "watching", label: "Watching" },
    { value: "planning", label: "Want to Watch" },
    { value: "completed", label: "Completed" },
    { value: "paused", label: "Paused" },
    { value: "dropped", label: "Dropped" },
  ];

  const STATUS_LABELS: Record<string, string> = {
    watching: "Watching",
    planning: "Want to Watch",
    completed: "Completed",
    paused: "Paused",
    dropped: "Dropped",
  };

  let activeTab = $state<MainTab>("watchlist");
  let listFilter = $state("all");

  // Watchlist state
  let watchlistItems: WatchlistItem[] = $state([]);
  let watchlistLoading = $state(false);

  // Continue watching state
  let continueItems: WatchlistItem[] = $state([]);
  let progressMap = $state(new Map<number, number>());
  let continueLoading = $state(false);

  // Local folders state
  let folders: string[] = $state([]);
  let localItems: LocalItem[] = $state([]);
  let localLoading = $state(false);
  let folderBusy = $state(false);

  let error = $state("");

  onMount(() => {
    readHashTab();
    loadActiveTab();
    const onHash = () => {
      readHashTab();
      loadActiveTab();
    };
    window.addEventListener("hashchange", onHash);
    return () => window.removeEventListener("hashchange", onHash);
  });

  function readHashTab() {
    const hash = window.location.hash;
    if (hash.includes("tab=continue")) activeTab = "continue";
    else if (hash.includes("tab=local")) activeTab = "local";
    else activeTab = "watchlist";
  }

  function switchTab(t: MainTab) {
    activeTab = t;
    window.location.hash = `#/library?tab=${t}`;
    loadActiveTab();
  }

  async function loadActiveTab() {
    error = "";
    if (activeTab === "watchlist") {
      await loadWatchlist();
    } else if (activeTab === "continue") {
      await loadContinueWatching();
    } else if (activeTab === "local") {
      await loadLocal();
    }
  }

  async function loadWatchlist() {
    watchlistLoading = true;
    try {
      watchlistItems = await api.watchlist(listFilter === "all" ? undefined : listFilter);
    } catch (e) {
      error = String(e);
      reportError("Library.loadWatchlist", e);
    } finally {
      watchlistLoading = false;
    }
  }

  async function setWatchlistFilter(f: string) {
    listFilter = f;
    await loadWatchlist();
  }

  async function removeFromWatchlist(id: number) {
    await api.removeFromWatchlist(id);
    await loadWatchlist();
  }

  async function updateStatus(id: number, status: string) {
    await api.setListStatus(id, status);
    await loadWatchlist();
  }

  async function loadContinueWatching() {
    continueLoading = true;
    try {
      const items = await api.continueWatching();
      continueItems = items;
      if (items.length > 0) {
        const entries = await Promise.all(items.map((w) => api.progress(w.animeId)));
        const map = new Map<number, number>();
        items.forEach((w, i) => {
          const last = entries[i].sort((a, b) => b.updatedAt - a.updatedAt)[0];
          if (last) map.set(w.animeId, last.episode);
        });
        progressMap = map;
      }
    } catch (e) {
      error = String(e);
      reportError("Library.loadContinue", e);
    } finally {
      continueLoading = false;
    }
  }

  async function loadLocal() {
    localLoading = true;
    try {
      folders = await api.listLibraryFolders();
      localItems = await api.scanLibrary();
    } catch (e) {
      error = String(e);
      reportError("Library.loadLocal", e);
    } finally {
      localLoading = false;
    }
  }

  async function pickFolder() {
    try {
      const { open } = await import("@tauri-apps/plugin-dialog");
      const selected = await open({ directory: true, multiple: false });
      if (typeof selected === "string" && selected) {
        folderBusy = true;
        await api.addLibraryFolder(selected);
        await loadLocal();
      }
    } catch (e) {
      error = String(e);
    } finally {
      folderBusy = false;
    }
  }

  async function removeFolder(path: string) {
    await api.removeLibraryFolder(path);
    await loadLocal();
  }
</script>

<div class="page">
  <div class="lib-header">
    <div>
      <h1 class="page-title">Library</h1>
      <p class="page-sub">Your personal collection, watchlists, and local storage on this machine.</p>
    </div>

    <!-- Main Library Section Tabs -->
    <div class="main-tabs">
      <button
        class="main-tab"
        class:active={activeTab === "watchlist"}
        onclick={() => switchTab("watchlist")}
      >
        Watchlist
      </button>
      <button
        class="main-tab"
        class:active={activeTab === "continue"}
        onclick={() => switchTab("continue")}
      >
        Continue Watching
      </button>
      <button
        class="main-tab"
        class:active={activeTab === "local"}
        onclick={() => switchTab("local")}
      >
        Local Folders
      </button>
    </div>
  </div>

  {#if error}
    <div class="error-box">
      <span>{error}</span>
      <button class="btn secondary" onclick={loadActiveTab}>Retry</button>
    </div>
  {/if}

  <!-- TAB 1: WATCHLIST -->
  {#if activeTab === "watchlist"}
    <div class="watchlist-section">
      <!-- Status chips -->
      <div class="chip-row">
        {#each LIST_TABS as t}
          <button
            class="chip"
            class:active={listFilter === t.value}
            onclick={() => setWatchlistFilter(t.value)}
          >
            {t.label}
          </button>
        {/each}
      </div>

      {#if watchlistLoading}
        <div class="spinner"></div>
      {:else if watchlistItems.length === 0}
        <div class="empty-state">
          <div class="big"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="40" height="40" stroke-linecap="round" stroke-linejoin="round"><path d="M12 17.75l-6.172 3.245 1.179-6.873-5-4.867 6.9-1.003L12 3l2.093 5.252 6.9 1.003-5 4.867 1.179 6.873z"/></svg></div>
          <p>Your watchlist is empty.</p>
          <p class="hint">Explore anime and click “Add to List” on any series page.</p>
        </div>
      {:else}
        <div class="list-rows">
          {#each watchlistItems as item (item.animeId)}
            <div
              class="list-row"
              data-anime-id={item.animeId}
              data-anime-title={item.title}
              data-anime-cover={item.cover || ""}
            >
              <button
                class="thumb-btn"
                onclick={() => router.navigate({ name: "details", id: item.animeId })}
              >
                {#if item.cover}
                  <img class="thumb" src={item.cover} alt={item.title} loading="lazy" />
                {:else}
                  <div class="thumb no-img"><svg viewBox="0 0 24 24" fill="currentColor" width="18" height="18"><polygon points="6 3 20 12 6 21 6 3"/></svg></div>
                {/if}
              </button>

              <button
                class="row-main"
                onclick={() => router.navigate({ name: "details", id: item.animeId })}
              >
                <div class="row-title">{item.title}</div>
                <div class="row-sub">
                  {#if item.episodesTotal}
                    <span>{item.episodesTotal} {item.episodesTotal === 1 ? "episode" : "episodes"}</span>
                  {/if}
                  <span class="sep">·</span>
                  <span>{item.progress > 0 ? `EP ${item.progress} watched` : "Not started"}</span>
                  {#if item.status === "RELEASING"}
                    <span class="sep">·</span>
                    <span class="airing">
                      <span class="airing-dot"></span>
                      <span>Airing</span>
                    </span>
                  {/if}
                </div>
                <div class="progress-track">
                  <div
                    class="progress-fill"
                    style="width: {item.episodesTotal ? Math.min(100, (item.progress / item.episodesTotal) * 100) : item.progress > 0 ? 8 : 0}%"
                  ></div>
                </div>
              </button>

              <div class="row-actions">
                <select
                  class="status-select"
                  value={item.listStatus}
                  onchange={(e) => updateStatus(item.animeId, (e.target as HTMLSelectElement).value)}
                >
                  {#each Object.entries(STATUS_LABELS) as [value, label]}
                    <option value={value}>{label}</option>
                  {/each}
                </select>

                <button
                  class="action-btn remove"
                  onclick={() => removeFromWatchlist(item.animeId)}
                  title="Remove from list"
                  aria-label="Remove from list"
                >
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" width="15" height="15">
                    <line x1="18" y1="6" x2="6" y2="18" />
                    <line x1="6" y1="6" x2="18" y2="18" />
                  </svg>
                </button>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </div>

  <!-- TAB 2: CONTINUE WATCHING -->
  {:else if activeTab === "continue"}
    <div class="continue-section">
      {#if continueLoading}
        <div class="spinner"></div>
      {:else if continueItems.length === 0}
        <div class="empty-state">
          <div class="big"><svg viewBox="0 0 24 24" fill="currentColor" width="40" height="40"><polygon points="6 3 20 12 6 21 6 3"/></svg></div>
          <p>No titles currently in progress.</p>
          <p class="hint">When you start watching an episode, it will appear here so you can resume anytime.</p>
        </div>
      {:else}
        <div class="continue-grid">
          {#each continueItems as item (item.animeId)}
            {@const lastEp = progressMap.get(item.animeId) ?? item.progress}
            {@const percent = item.episodesTotal ? Math.min(100, Math.max(10, (lastEp / item.episodesTotal) * 100)) : 50}
            <div
              class="cw-box"
              data-anime-id={item.animeId}
              data-anime-title={item.title}
              data-anime-cover={item.cover || ""}
            >
              <button
                class="cw-thumb-btn"
                onclick={() => router.navigate({ name: "watch", id: item.animeId, episode: Math.max(1, lastEp) })}
              >
                {#if item.cover}
                  <img src={item.cover} alt={item.title} loading="lazy" />
                {:else}
                  <div class="no-img"><svg viewBox="0 0 24 24" fill="currentColor" width="18" height="18"><polygon points="6 3 20 12 6 21 6 3"/></svg></div>
                {/if}
                <div class="cw-overlay">
                  <span class="cw-play-btn">
                    <svg viewBox="0 0 24 24" fill="currentColor" width="16" height="16">
                      <path d="M8 5v14l11-7z" />
                    </svg>
                  </span>
                </div>
                <div class="cw-progress">
                  <div class="cw-fill" style="width: {percent}%"></div>
                </div>
              </button>

              <div class="cw-details">
                <button
                  class="cw-title-btn"
                  onclick={() => router.navigate({ name: "details", id: item.animeId })}
                  title={item.title}
                >
                  {item.title}
                </button>
                <div class="cw-sub">
                  Episode {Math.max(1, lastEp)}{item.episodesTotal ? ` of ${item.episodesTotal}` : ""}
                </div>
                <button
                  class="btn primary resume-btn"
                  onclick={() => router.navigate({ name: "watch", id: item.animeId, episode: Math.max(1, lastEp) })}
                >
                  Resume EP {Math.max(1, lastEp)}
                </button>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </div>

  <!-- TAB 3: LOCAL FOLDERS -->
  {:else if activeTab === "local"}
    <div class="local-section">
      <div class="local-actions-bar">
        <div>
          <h3>Local Directories</h3>
          <p class="section-desc">Add folders with anime files to play offline over the local Tauri protocol.</p>
        </div>
        <button class="btn primary" onclick={pickFolder} disabled={folderBusy}>
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" width="16" height="16">
            <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z" />
          </svg>
          Add Folder
        </button>
      </div>

      {#if localLoading}
        <div class="spinner"></div>
      {:else}
        {#if folders.length > 0}
          <div class="folder-list">
            {#each folders as f (f)}
              <div class="folder-row">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="f-icon">
                  <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z" />
                </svg>
                <span class="f-path" title={f}>{f}</span>
                <button class="btn danger" onclick={() => removeFolder(f)}>Remove</button>
              </div>
            {/each}
          </div>
        {/if}

        {#if localItems.length > 0}
          <div class="section-title">
            <h3>Discovered Local Shows ({localItems.length})</h3>
            <button class="link" onclick={loadLocal}>Rescan</button>
          </div>
          <div class="local-grid">
            {#each localItems as item (item.title)}
              <div class="local-card">
                <div class="local-icon">
                  <svg viewBox="0 0 24 24" fill="currentColor"><path d="M8 5v14l11-7z" /></svg>
                </div>
                <div class="local-title" title={item.title}>{item.title}</div>
                <div class="local-count">{item.episodeCount} {item.episodeCount === 1 ? "episode" : "episodes"}</div>
              </div>
            {/each}
          </div>
        {:else if folders.length > 0}
          <div class="empty-state">
            <div class="big"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" width="40" height="40" stroke-linecap="round" stroke-linejoin="round"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z" /></svg></div>
            <p>No video files found in the watched folders.</p>
            <p class="hint">Name each folder after the series with episodes inside (.mp4, .mkv).</p>
          </div>
        {:else}
          <div class="empty-state">
            <div class="big"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" width="40" height="40" stroke-linecap="round" stroke-linejoin="round"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z" /></svg></div>
            <p>No local folders added yet.</p>
            <p class="hint">Click "Add Folder" to point to your offline collection.</p>
          </div>
        {/if}
      {/if}
    </div>
  {/if}
</div>

<style>
  .lib-header {
    display: flex;
    align-items: flex-end;
    justify-content: space-between;
    gap: 20px;
    margin-bottom: 24px;
    flex-wrap: wrap;
    border-bottom: 1px solid var(--border);
    padding-bottom: 16px;
  }

  .main-tabs {
    display: flex;
    gap: 6px;
    background: var(--surface);
    border: 1px solid var(--border);
    padding: 3px;
    border-radius: 999px;
  }

  .main-tab {
    padding: 8px 20px;
    border-radius: 999px;
    font-size: 13.5px;
    font-weight: 600;
    color: var(--text-dim);
    transition: all 0.13s ease;
  }

  .main-tab:hover {
    color: var(--text);
  }

  .main-tab.active {
    background: var(--accent);
    color: #0d0d0d;
    font-weight: 700;
  }

  .chip-row {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
    margin-bottom: 24px;
  }

  /* Watchlist list rows */
  .list-rows {
    display: flex;
    flex-direction: column;
    gap: 10px;
    max-width: 1080px;
  }

  .list-row {
    display: flex;
    align-items: center;
    gap: 16px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    padding: 12px 16px;
    transition: border-color 0.12s ease;
  }

  .list-row:hover {
    border-color: var(--surface-3);
  }

  .thumb-btn {
    flex-shrink: 0;
    cursor: pointer;
  }

  .thumb {
    width: 58px;
    height: 84px;
    border-radius: var(--radius-sm);
    object-fit: cover;
    background: var(--surface-2);
    display: block;
  }

  .no-img {
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--accent);
    font-size: 18px;
    font-weight: 800;
  }

  .row-main {
    flex: 1;
    min-width: 0;
    text-align: left;
    cursor: pointer;
    background: none;
    border: none;
    padding: 0;
  }

  .row-title {
    font-weight: 700;
    font-size: 14.5px;
    color: var(--text);
    margin-bottom: 4px;
    transition: color 0.12s ease;
  }

  .row-main:hover .row-title {
    color: var(--accent);
  }

  .row-sub {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 12.5px;
    color: var(--text-dim);
    margin-bottom: 8px;
  }

  .sep {
    color: var(--text-faint);
  }

  .airing {
    display: flex;
    align-items: center;
    gap: 5px;
    color: var(--accent);
    font-weight: 600;
  }

  .progress-track {
    height: 4px;
    background: var(--surface-3);
    border-radius: 2px;
    overflow: hidden;
    max-width: 320px;
  }

  .progress-fill {
    height: 100%;
    background: var(--accent);
    border-radius: 2px;
  }

  .row-actions {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .status-select {
    font-size: 13px;
    font-weight: 600;
    padding: 8px 14px;
    border-radius: 999px;
    background: var(--surface-2);
    border: 1px solid var(--border);
    color: var(--text);
  }

  .action-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 34px;
    height: 34px;
    border-radius: 50%;
    background: var(--surface-2);
    color: var(--text-dim);
    transition: all 0.12s ease;
  }

  .action-btn.remove:hover {
    color: #ff8585;
    background: #331818;
  }

  /* Continue watching grid */
  .continue-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
    gap: 20px;
  }

  .cw-box {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .cw-thumb-btn {
    position: relative;
    aspect-ratio: 16 / 9;
    width: 100%;
    overflow: hidden;
    background: var(--surface-2);
    cursor: pointer;
  }

  .cw-thumb-btn img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .cw-overlay {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(0, 0, 0, 0.45);
    opacity: 0;
    transition: opacity 0.14s ease;
  }

  .cw-thumb-btn:hover .cw-overlay {
    opacity: 1;
  }

  .cw-play-btn {
    width: 44px;
    height: 44px;
    border-radius: 50%;
    background: var(--accent);
    color: #0d0d0d;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .cw-progress {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    height: 4px;
    background: rgba(0, 0, 0, 0.7);
  }

  .cw-fill {
    height: 100%;
    background: var(--accent);
  }

  .cw-details {
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 6px;
    flex: 1;
  }

  .cw-title-btn {
    font-size: 14.5px;
    font-weight: 700;
    color: var(--text);
    text-align: left;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    transition: color 0.12s ease;
  }

  .cw-title-btn:hover {
    color: var(--accent);
  }

  .cw-sub {
    font-size: 12.5px;
    color: var(--text-dim);
    margin-bottom: 6px;
  }

  .resume-btn {
    align-self: flex-start;
    padding: 7px 16px;
    font-size: 12.5px;
  }

  /* Local folders */
  .local-actions-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    margin-bottom: 20px;
  }

  .local-actions-bar h3 {
    font-size: 18px;
    margin-bottom: 4px;
  }

  .section-desc {
    font-size: 13px;
    color: var(--text-dim);
    margin: 0;
  }

  .folder-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
    max-width: 920px;
    margin-bottom: 30px;
  }

  .folder-row {
    display: flex;
    align-items: center;
    gap: 14px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    padding: 12px 18px;
  }

  .f-icon {
    width: 18px;
    height: 18px;
    color: var(--accent);
    flex-shrink: 0;
  }

  .f-path {
    flex: 1;
    font-size: 13px;
    color: var(--text-dim);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    direction: rtl;
    text-align: left;
    font-family: var(--mono);
  }

  .local-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 14px;
  }

  .local-card {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    padding: 18px 16px;
    color: var(--text);
  }

  .local-icon {
    width: 36px;
    height: 36px;
    border-radius: 50%;
    background: var(--accent-dim);
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--accent);
    margin-bottom: 12px;
  }

  .local-title {
    font-weight: 700;
    font-size: 14px;
    margin-bottom: 4px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .local-count {
    font-size: 12px;
    color: var(--text-faint);
  }
</style>
