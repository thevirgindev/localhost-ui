<script lang="ts">
  import { onMount } from "svelte";
  import { router } from "../lib/router";
  import { userStore } from "../lib/userStore.svelte";
  import { api } from "../lib/api";
  import type { AnimeCard } from "../lib/types";

  interface AnimeContext {
    id: number;
    title: string;
    cover?: string;
    isWatchlisted: boolean;
  }

  interface EpisodeContext {
    animeId?: number;
    episodeNumber: number;
    title?: string;
  }

  let visible = $state(false);
  let posX = $state(0);
  let posY = $state(0);
  let animeCtx = $state<AnimeContext | null>(null);
  let episodeCtx = $state<EpisodeContext | null>(null);
  let toastMessage = $state("");
  let toastTimer: ReturnType<typeof setTimeout> | undefined;

  function showToast(msg: string) {
    toastMessage = msg;
    clearTimeout(toastTimer);
    toastTimer = setTimeout(() => {
      toastMessage = "";
    }, 2400);
  }

  onMount(() => {
    function handleContextMenu(e: MouseEvent) {
      // Allow default context menu if Shift key is pressed
      if (e.shiftKey) return;

      e.preventDefault();

      const target = e.target as HTMLElement | null;
      if (!target) return;

      // Check if clicked inside an anime card or element with anime metadata
      const animeEl = target.closest("[data-anime-id]") as HTMLElement | null;
      const episodeEl = target.closest("[data-episode-number]") as HTMLElement | null;

      if (animeEl) {
        const id = Number(animeEl.getAttribute("data-anime-id"));
        const title = animeEl.getAttribute("data-anime-title") || "Anime";
        const cover = animeEl.getAttribute("data-anime-cover") || undefined;
        
        // Check watchlist status asynchronously
        animeCtx = {
          id,
          title,
          cover,
          isWatchlisted: false,
        };
        api.inWatchlist(id).then((inList) => {
          if (animeCtx && animeCtx.id === id) {
            animeCtx.isWatchlisted = inList;
          }
        });
      } else {
        animeCtx = null;
      }

      if (episodeEl) {
        const epNum = Number(episodeEl.getAttribute("data-episode-number"));
        const animeId = Number(episodeEl.getAttribute("data-anime-id")) || undefined;
        const title = episodeEl.getAttribute("data-episode-title") || undefined;
        episodeCtx = {
          animeId,
          episodeNumber: epNum,
          title,
        };
      } else {
        episodeCtx = null;
      }

      // Calculate position with viewport boundaries
      const menuWidth = 240;
      const menuHeight = animeCtx ? 310 : 360;
      const screenW = window.innerWidth;
      const screenH = window.innerHeight;

      let x = e.clientX;
      let y = e.clientY;

      if (x + menuWidth > screenW - 12) {
        x = Math.max(12, screenW - menuWidth - 12);
      }
      if (y + menuHeight > screenH - 12) {
        y = Math.max(12, screenH - menuHeight - 12);
      }

      posX = x;
      posY = y;
      visible = true;
    }

    function handleGlobalClick(e: MouseEvent) {
      if (visible) {
        const target = e.target as HTMLElement | null;
        if (!target?.closest(".desktop-context-menu")) {
          closeMenu();
        }
      }
    }

    function handleKeydown(e: KeyboardEvent) {
      if (e.key === "Escape" && visible) {
        closeMenu();
      }
    }

    function handleScroll() {
      if (visible) closeMenu();
    }

    window.addEventListener("contextmenu", handleContextMenu);
    window.addEventListener("click", handleGlobalClick);
    window.addEventListener("keydown", handleKeydown);
    window.addEventListener("scroll", handleScroll, true);
    window.addEventListener("resize", closeMenu);

    return () => {
      window.removeEventListener("contextmenu", handleContextMenu);
      window.removeEventListener("click", handleGlobalClick);
      window.removeEventListener("keydown", handleKeydown);
      window.removeEventListener("scroll", handleScroll, true);
      window.removeEventListener("resize", closeMenu);
    };
  });

  function closeMenu() {
    visible = false;
    animeCtx = null;
    episodeCtx = null;
  }

  // Anime Actions
  function watchAnimeNow() {
    if (!animeCtx) return;
    const id = animeCtx.id;
    closeMenu();
    router.navigate({ name: "watch", id, episode: 1 });
  }

  function viewAnimeDetails() {
    if (!animeCtx) return;
    const id = animeCtx.id;
    closeMenu();
    router.navigate({ name: "details", id });
  }

  async function toggleAnimeWatchlist() {
    if (!animeCtx) return;
    const id = animeCtx.id;
    try {
      if (animeCtx.isWatchlisted) {
        await api.removeFromWatchlist(id);
        animeCtx.isWatchlisted = false;
        showToast(`Removed "${animeCtx.title}" from Watchlist`);
      } else {
        await api.addToWatchlist({
          id,
          title: animeCtx.title,
          cover: animeCtx.cover || "",
        } as AnimeCard);
        animeCtx.isWatchlisted = true;
        showToast(`Added "${animeCtx.title}" to Watchlist`);
      }
    } catch {
      showToast("Could not update watchlist");
    }
    closeMenu();
  }

  function copyAnimeLink() {
    if (!animeCtx) return;
    const url = `${window.location.origin}${window.location.pathname}#/details?id=${animeCtx.id}`;
    navigator.clipboard.writeText(url).then(() => {
      showToast("Anime link copied to clipboard!");
    }).catch(() => {
      showToast("Link: " + url);
    });
    closeMenu();
  }

  function openAnimeNewTab() {
    if (!animeCtx) return;
    const url = `${window.location.origin}${window.location.pathname}#/details?id=${animeCtx.id}`;
    window.open(url, "_blank");
    closeMenu();
  }

  function getCurrentAnimeId(): number {
    const cur = router.current();
    if ("id" in cur && typeof cur.id === "number") {
      return cur.id;
    }
    return 1;
  }

  // Episode Actions
  function playEpisode() {
    if (!episodeCtx) return;
    const epNum = episodeCtx.episodeNumber;
    const animeId = episodeCtx.animeId || getCurrentAnimeId();
    closeMenu();
    if (animeId) {
      router.navigate({ name: "watch", id: animeId, episode: epNum });
    }
  }

  async function markEpisodeWatched() {
    if (!episodeCtx) return;
    const epNum = episodeCtx.episodeNumber;
    const animeId = episodeCtx.animeId || getCurrentAnimeId();
    if (animeId) {
      try {
        await api.saveProgress(animeId, epNum, 1440, 1440);
        showToast(`Marked Episode ${epNum} as completed!`);
      } catch {
        showToast("Error updating progress");
      }
    }
    closeMenu();
  }

  // General Page Navigation
  function goBack() {
    closeMenu();
    window.history.back();
  }

  function goForward() {
    closeMenu();
    window.history.forward();
  }

  function reloadPage() {
    closeMenu();
    window.location.reload();
  }

  function clearAppCache() {
    closeMenu();
    try {
      sessionStorage.clear();
      showToast("App cache cleared successfully!");
    } catch {
      showToast("Cache cleared");
    }
  }

  function openSearch() {
    closeMenu();
    userStore.showSearchModal = true;
  }

  function goHome() {
    closeMenu();
    router.navigate({ name: "home" });
  }

  function goBrowse() {
    closeMenu();
    router.navigate({ name: "browse" });
  }

  function goSeason() {
    closeMenu();
    router.navigate({ name: "season" });
  }

  function goWatchlist() {
    closeMenu();
    router.navigate({ name: "list" });
  }

  function openSettings() {
    closeMenu();
    router.navigate({ name: "settings" });
  }

  function openChangelog() {
    closeMenu();
    userStore.showChangelogModal = true;
  }

  function toggleFullscreen() {
    closeMenu();
    if (!document.fullscreenElement) {
      document.documentElement.requestFullscreen().catch(() => {});
    } else {
      document.exitFullscreen().catch(() => {});
    }
  }
</script>

{#if visible}
  <div
    class="desktop-context-menu"
    style="left: {posX}px; top: {posY}px;"
    role="menu"
    tabindex="-1"
  >
    <!-- ANIME CARD CONTEXT -->
    {#if animeCtx}
      <div class="menu-header">
        {#if animeCtx.cover}
          <img src={animeCtx.cover} alt={animeCtx.title} class="header-thumb" />
        {/if}
        <div class="header-meta">
          <span class="header-badge">Anime Series</span>
          <span class="header-title" title={animeCtx.title}>{animeCtx.title}</span>
        </div>
      </div>

      <div class="menu-divider"></div>

      <button class="menu-item primary" onclick={watchAnimeNow} role="menuitem">
        <svg viewBox="0 0 24 24" fill="currentColor" class="item-icon">
          <path d="M8 5v14l11-7z" />
        </svg>
        <span class="item-label">Watch Now</span>
        <span class="item-shortcut">Play</span>
      </button>

      <button class="menu-item" onclick={toggleAnimeWatchlist} role="menuitem">
        {#if animeCtx.isWatchlisted}
          <svg viewBox="0 0 24 24" style="fill: var(--accent)" class="item-icon">
            <path d="M19 21l-7-5-7 5V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2z" />
          </svg>
          <span class="item-label">In Watchlist</span>
          <span class="item-shortcut active-sub"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" width="11" height="11" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"/></svg> Saved</span>
        {:else}
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="item-icon">
            <path d="M19 21l-7-5-7 5V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2z" />
          </svg>
          <span class="item-label">Add to Watchlist</span>
          <span class="item-shortcut">+ Bookmark</span>
        {/if}
      </button>

      <button class="menu-item" onclick={viewAnimeDetails} role="menuitem">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="item-icon">
          <circle cx="12" cy="12" r="10" />
          <line x1="12" y1="16" x2="12" y2="12" />
          <line x1="12" y1="8" x2="12.01" y2="8" />
        </svg>
        <span class="item-label">View Anime Details</span>
      </button>

      <div class="menu-divider"></div>

      <button class="menu-item" onclick={copyAnimeLink} role="menuitem">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="item-icon">
          <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71" />
          <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71" />
        </svg>
        <span class="item-label">Copy Anime Link</span>
      </button>

      <button class="menu-item" onclick={openAnimeNewTab} role="menuitem">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="item-icon">
          <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" />
          <polyline points="15 3 21 3 21 9" />
          <line x1="10" y1="14" x2="21" y2="3" />
        </svg>
        <span class="item-label">Open in New Tab</span>
      </button>

    <!-- EPISODE ITEM CONTEXT -->
    {:else if episodeCtx}
      <div class="menu-header">
        <span class="header-badge">Episode Entry</span>
        <span class="header-title">Episode {episodeCtx.episodeNumber}</span>
      </div>

      <div class="menu-divider"></div>

      <button class="menu-item primary" onclick={playEpisode} role="menuitem">
        <svg viewBox="0 0 24 24" fill="currentColor" class="item-icon">
          <path d="M8 5v14l11-7z" />
        </svg>
        <span class="item-label">Play Episode</span>
      </button>

      <button class="menu-item" onclick={markEpisodeWatched} role="menuitem">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="item-icon">
          <polyline points="20 6 9 17 4 12" />
        </svg>
        <span class="item-label">Mark as Watched</span>
      </button>

    <!-- GENERAL PAGE CONTEXT -->
    {:else}
      <div class="menu-group">
        <button class="menu-item" onclick={goBack} role="menuitem">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="item-icon">
            <polyline points="15 18 9 12 15 6" />
          </svg>
          <span class="item-label">Go Back</span>
          <span class="item-shortcut">Alt + ←</span>
        </button>

        <button class="menu-item" onclick={goForward} role="menuitem">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="item-icon">
            <polyline points="9 18 15 12 9 6" />
          </svg>
          <span class="item-label">Forward</span>
          <span class="item-shortcut">Alt + →</span>
        </button>

        <button class="menu-item" onclick={reloadPage} role="menuitem">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="item-icon">
            <path d="M23 4v6h-6" />
            <path d="M20.49 15a9 9 0 1 1-2.12-9.36L23 10" />
          </svg>
          <span class="item-label">Refresh</span>
          <span class="item-shortcut">Ctrl + R</span>
        </button>

        <button class="menu-item" onclick={clearAppCache} role="menuitem">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="item-icon">
            <polyline points="3 6 5 6 21 6" />
            <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" />
          </svg>
          <span class="item-label">Clear Cache</span>
        </button>
      </div>

      <div class="menu-divider"></div>

      <div class="menu-group">
        <button class="menu-item highlight-search" onclick={openSearch} role="menuitem">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="item-icon search-color">
            <circle cx="11" cy="11" r="8" />
            <line x1="21" y1="21" x2="16.65" y2="16.65" />
          </svg>
          <span class="item-label bold">Search Anime...</span>
          <span class="item-shortcut badge-key">Ctrl + K</span>
        </button>

        <button class="menu-item" onclick={goHome} role="menuitem">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="item-icon">
            <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" />
          </svg>
          <span class="item-label">Home</span>
        </button>

        <button class="menu-item" onclick={goBrowse} role="menuitem">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="item-icon">
            <circle cx="12" cy="12" r="10" />
            <polygon points="16.24 7.76 14.12 14.12 7.76 16.24 9.88 9.88 16.24 7.76" />
          </svg>
          <span class="item-label">Explore Catalog</span>
        </button>

        <button class="menu-item" onclick={goSeason} role="menuitem">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="item-icon">
            <rect x="3" y="4" width="18" height="18" rx="2" ry="2" />
            <line x1="16" y1="2" x2="16" y2="6" />
            <line x1="8" y1="2" x2="8" y2="6" />
            <line x1="3" y1="10" x2="21" y2="10" />
          </svg>
          <span class="item-label">Seasonal Lineup</span>
        </button>

        <button class="menu-item" onclick={goWatchlist} role="menuitem">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="item-icon">
            <path d="M19 21l-7-5-7 5V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2z" />
          </svg>
          <span class="item-label">My Watchlist</span>
        </button>
      </div>

      <div class="menu-divider"></div>

      <div class="menu-group">
        <button class="menu-item" onclick={toggleFullscreen} role="menuitem">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="item-icon">
            <path d="M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3" />
          </svg>
          <span class="item-label">Toggle Fullscreen</span>
          <span class="item-shortcut">F11</span>
        </button>

        <button class="menu-item" onclick={openSettings} role="menuitem">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="item-icon">
            <circle cx="12" cy="12" r="3" />
            <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" />
          </svg>
          <span class="item-label">Preferences & Settings</span>
        </button>

        <button class="menu-item" onclick={openChangelog} role="menuitem">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="item-icon">
            <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
            <polyline points="14 2 14 8 20 8" />
            <line x1="16" y1="13" x2="8" y2="13" />
            <line x1="16" y1="17" x2="8" y2="17" />
          </svg>
          <span class="item-label">Release Notes</span>
        </button>
      </div>
    {/if}
  </div>
{/if}

<!-- Toast Feedback -->
{#if toastMessage}
  <div class="context-toast" role="status">
    <span class="toast-dot"></span>
    <span>{toastMessage}</span>
  </div>
{/if}

<style>
  .desktop-context-menu {
    position: fixed;
    z-index: 9999;
    width: 235px;
    background: #14151a;
    border: 1px solid #282a33;
    border-radius: 8px;
    box-shadow: 0 16px 40px rgba(0, 0, 0, 0.92);
    padding: 6px;
    display: flex;
    flex-direction: column;
    animation: menuPop 0.12s cubic-bezier(0.16, 1, 0.3, 1);
    user-select: none;
  }

  @keyframes menuPop {
    from {
      opacity: 0;
      transform: scale(0.95) translateY(-4px);
    }
    to {
      opacity: 1;
      transform: scale(1) translateY(0);
    }
  }

  .menu-header {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 6px 8px 8px;
  }

  .header-thumb {
    width: 32px;
    height: 44px;
    border-radius: 4px;
    object-fit: cover;
    flex-shrink: 0;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.5);
  }

  .header-meta {
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .header-badge {
    font-size: 9.5px;
    font-weight: 800;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: var(--accent);
  }

  .header-title {
    font-size: 12.5px;
    font-weight: 700;
    color: #ffffff;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    line-height: 1.3;
  }

  .menu-divider {
    height: 1px;
    background: #23252d;
    margin: 4px 0;
  }

  .menu-group {
    display: flex;
    flex-direction: column;
    gap: 1px;
  }

  .menu-item {
    display: flex;
    align-items: center;
    gap: 9px;
    width: 100%;
    padding: 7px 9px;
    border-radius: 5px;
    background: transparent;
    border: none;
    color: #d1d2dc;
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    text-align: left;
    transition: background 0.1s ease, color 0.1s ease;
  }

  .menu-item:hover {
    background: #232630;
    color: #ffffff;
  }

  .menu-item.primary {
    color: var(--accent);
    font-weight: 700;
  }

  .menu-item.primary:hover {
    background: color-mix(in srgb, var(--accent) 16%, transparent);
    color: #c084fc;
  }

  .highlight-search {
    background: color-mix(in srgb, var(--accent) 8%, transparent);
  }

  .highlight-search:hover {
    background: color-mix(in srgb, var(--accent) 18%, transparent);
  }

  .search-color {
    color: var(--accent);
  }

  .item-icon {
    width: 15px;
    height: 15px;
    flex-shrink: 0;
    color: inherit;
    opacity: 0.9;
  }

  .item-label {
    flex: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .item-label.bold {
    font-weight: 700;
  }

  .item-shortcut {
    font-size: 10.5px;
    font-weight: 600;
    color: #6d6f78;
    letter-spacing: 0.02em;
    flex-shrink: 0;
  }

  .item-shortcut.active-sub {
    color: var(--accent);
    font-weight: 700;
  }

  .item-shortcut.badge-key {
    background: #1e2027;
    border: 1px solid #2e313b;
    padding: 1px 5px;
    border-radius: 3px;
    color: #9ba0aa;
    font-size: 10px;
    font-family: inherit;
  }

  /* Toast Notification */
  .context-toast {
    position: fixed;
    bottom: 24px;
    right: 24px;
    z-index: 10000;
    background: #181a20;
    border: 1px solid #2e3039;
    color: #ffffff;
    padding: 10px 18px;
    border-radius: 6px;
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.85);
    font-size: 13px;
    font-weight: 600;
    display: flex;
    align-items: center;
    gap: 8px;
    animation: toastIn 0.2s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .toast-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--accent);
    flex-shrink: 0;
  }

  @keyframes toastIn {
    from {
      opacity: 0;
      transform: translateY(10px) scale(0.96);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }
</style>
