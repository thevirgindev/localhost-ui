<script lang="ts">
  import { onMount } from "svelte";
  import type { Route } from "../lib/router";
  import { userStore } from "../lib/userStore.svelte";

  let { route }: { route: Route } = $props();

  let showProfileMenu = $state(false);
  let showNotifsMenu = $state(false);

  const navItems = [
    { name: "home", label: "Home", hash: "#/" },
    { name: "browse", label: "Browse", hash: "#/browse" },
    { name: "season", label: "Seasons", hash: "#/season" },
    { name: "list", label: "My Lists", hash: "#/library?tab=watchlist" },
  ];

  function isActive(item: { name: string }): boolean {
    if (item.name === "home") return route.name === "home";
    if (item.name === "list") return route.name === "list" || route.name === "library";
    return route.name === item.name;
  }

  function toggleProfileMenu(e: MouseEvent) {
    e.stopPropagation();
    showProfileMenu = !showProfileMenu;
    showNotifsMenu = false;
  }

  function toggleNotifsMenu(e: MouseEvent) {
    e.stopPropagation();
    showNotifsMenu = !showNotifsMenu;
    showProfileMenu = false;
  }

  function openSearch() {
    userStore.showSearchModal = true;
    showProfileMenu = false;
    showNotifsMenu = false;
  }

  function openSettings() {
    window.location.hash = "#/settings";
    showProfileMenu = false;
  }

  function openProfileModal() {
    userStore.showProfileModal = true;
    showProfileMenu = false;
  }

  function openChangelog() {
    userStore.showChangelogModal = true;
    showProfileMenu = false;
  }

  // Custom frameless-window controls (no-op outside Tauri).
  async function windowCtrl(action: "minimize" | "maximize" | "close") {
    try {
      const { getCurrentWindow } = await import("@tauri-apps/api/window");
      const w = getCurrentWindow();
      if (action === "minimize") await w.minimize();
      else if (action === "maximize") await w.toggleMaximize();
      else await w.close();
    } catch {
      // Browser preview — no window controls
    }
  }

  onMount(() => {
    function handleDocumentClick() {
      showProfileMenu = false;
      showNotifsMenu = false;
    }

    function handleGlobalKeys(e: KeyboardEvent) {
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "k") {
        e.preventDefault();
        openSearch();
      } else if (e.key === "/" && !["INPUT", "TEXTAREA"].includes((e.target as HTMLElement)?.tagName)) {
        e.preventDefault();
        openSearch();
      }
    }

    document.addEventListener("click", handleDocumentClick);
    window.addEventListener("keydown", handleGlobalKeys);

    return () => {
      document.removeEventListener("click", handleDocumentClick);
      window.removeEventListener("keydown", handleGlobalKeys);
    };
  });
</script>

<header class="navbar" data-tauri-drag-region>
  <div class="nav-inner">
    <!-- Far Left: Crunchyroll Logo -->
    <div class="nav-left">
      <a class="brand" href="#/">
        <svg class="cr-logo" viewBox="0 0 24 24" fill="none">
          <rect x="1" y="1" width="22" height="22" rx="6" fill="#a855f7" />
          <path
            d="M12 5.5l1.55 4.35a2 2 0 0 0 1.2 1.2L19.1 12.6l-4.35 1.55a2 2 0 0 0-1.2 1.2L12 19.7l-1.55-4.35a2 2 0 0 0-1.2-1.2L4.9 12.6l4.35-1.55a2 2 0 0 0 1.2-1.2z"
            fill="#0d0d0d"
          />
          <circle cx="17.8" cy="6.2" r="1.3" fill="#0d0d0d" />
        </svg>
        <span class="brand-text">Luci</span>
      </a>
    </div>

    <!-- Center: Navigation Links matching Crunchyroll header precisely -->
    <nav class="nav-center">
      {#each navItems as item}
        <a class="nav-link" class:active={isActive(item)} href={item.hash}>
          <span>{item.label}</span>
          {#if isActive(item)}
            <span class="active-indicator"></span>
          {/if}
        </a>
      {/each}
      <button class="nav-link nav-btn" onclick={openChangelog}>
        <span>News</span>
      </button>
    </nav>

    <!-- Far Right: Search Icon + Watchlist + Airings Notification + Profile -->
    <div class="nav-right">
      <!-- Search Icon that initiates centered bottom large card -->
      <button
        class="nav-icon-btn"
        onclick={openSearch}
        title="Search anime (Ctrl+K or /)"
        aria-label="Universal Search"
      >
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.4" width="20" height="20">
          <circle cx="11" cy="11" r="8" />
          <line x1="21" y1="21" x2="16.65" y2="16.65" />
        </svg>
      </button>

      <!-- Crunchyroll Watchlist Ribbon Icon -->
      <a class="nav-icon-btn" href="#/library?tab=watchlist" title="My Lists / Watchlist" aria-label="Watchlist">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" width="20" height="20">
          <path d="M19 21l-7-5-7 5V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2z" />
        </svg>
      </a>

      <!-- Notifications Bell Icon with Airings Dropdown -->
      <div class="dropdown-anchor">
        <button
          class="nav-icon-btn bell-icon-btn"
          onclick={toggleNotifsMenu}
          title="Airing Notifications"
          aria-label="Airing Notifications"
        >
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" width="20" height="20">
            <path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9" />
            <path d="M13.73 21a2 2 0 0 1-3.46 0" />
          </svg>
          {#if userStore.unreadNotificationsCount > 0}
            <span class="unread-dot">{userStore.unreadNotificationsCount}</span>
          {/if}
        </button>

        <!-- Airings Notification Dropdown Card -->
        {#if showNotifsMenu}
          <div
            class="notifs-menu"
            onclick={(e) => e.stopPropagation()}
            role="dialog"
            aria-label="Recent Airings"
            tabindex="-1"
          >
            <div class="menu-header">
              <span class="menu-title">Airing Broadcasts</span>
              <button class="mark-read-btn" onclick={() => userStore.markAllNotificationsRead()}>
                Mark read
              </button>
            </div>

            <div class="menu-list notifs-list">
              {#each userStore.notifications as notif (notif.id)}
                <div
                  class="notif-card"
                  class:unread={!notif.read}
                  onclick={() => {
                    userStore.markNotificationRead(notif.id);
                    showNotifsMenu = false;
                    window.location.hash = `#/details?id=${notif.animeId}`;
                  }}
                  role="button"
                  tabindex="0"
                  onkeydown={(e) => e.key === "Enter" && (window.location.hash = `#/details?id=${notif.animeId}`)}
                >
                  <img src={notif.cover} alt={notif.animeTitle} class="notif-img" />
                  <div class="notif-body">
                    <div class="notif-heading">
                      <span class="notif-title">{notif.animeTitle}</span>
                      <span class="notif-time">{notif.timeAgo}</span>
                    </div>
                    <p class="notif-text">{notif.message}</p>
                  </div>
                </div>
              {/each}
            </div>

            <div class="menu-footer">
              <a href="#/season" class="menu-footer-link" onclick={() => (showNotifsMenu = false)}>
                View Full Seasonal Airings Schedule <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" width="12" height="12" stroke-linecap="round" stroke-linejoin="round"><line x1="5" y1="12" x2="19" y2="12"/><polyline points="12 5 19 12 12 19"/></svg>
              </a>
            </div>
          </div>
        {/if}
      </div>

      <!-- Profile Avatar Button with Dropdown Menu -->
      <div class="dropdown-anchor">
        <button
          class="profile-pill-btn"
          onclick={toggleProfileMenu}
          title="User Profile & Settings"
          aria-label="Profile Menu"
        >
          <div class="avatar-ring" style="border-color: {userStore.activeProfile.color}">
            <img
              class="avatar-img"
              src={userStore.activeProfile.avatar}
              alt={userStore.activeProfile.name}
            />
          </div>
          <span class="profile-name-text">{userStore.activeProfile.name}</span>
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="chevron-icon">
            <polyline points="6 9 12 15 18 9" />
          </svg>
        </button>

        <!-- Profile Dropdown Menu -->
        {#if showProfileMenu}
          <div
            class="profile-dropdown-menu"
            onclick={(e) => e.stopPropagation()}
            role="menu"
            tabindex="-1"
          >
            <!-- User Header -->
            <div class="profile-summary">
              <img src={userStore.activeProfile.avatar} alt="Avatar" class="summary-avatar" />
              <div>
                <div class="summary-name">{userStore.activeProfile.name}</div>
                <div class="summary-status">
                  <span class="status-indicator"></span>
                  <span>Local Mode Active</span>
                </div>
              </div>
            </div>

            <div class="dropdown-divider"></div>

            <!-- Action Items -->
            <button class="dropdown-item" onclick={openSettings} role="menuitem">
              <span class="item-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="15" height="15" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3" /><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" /></svg></span>
              <span>Settings Center</span>
            </button>

            <button class="dropdown-item" onclick={openProfileModal} role="menuitem">
              <span class="item-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="15" height="15" stroke-linecap="round" stroke-linejoin="round"><path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" /><circle cx="9" cy="7" r="4" /><path d="M23 21v-2a4 4 0 0 0-3-3.87" /><path d="M16 3.13a4 4 0 0 1 0 7.75" /></svg></span>
              <span>Switch Profile</span>
            </button>

            <a
              href="#/library?tab=watchlist"
              class="dropdown-item link-item"
              onclick={() => (showProfileMenu = false)}
              role="menuitem"
            >
              <span class="item-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="15" height="15" stroke-linecap="round" stroke-linejoin="round"><path d="M19 21l-7-5-7 5V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2z" /></svg></span>
              <span>My Lists & Watchlist</span>
            </a>

            <a
              href="#/library?tab=history"
              class="dropdown-item link-item"
              onclick={() => (showProfileMenu = false)}
              role="menuitem"
            >
              <span class="item-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="15" height="15" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10" /><polyline points="12 6 12 12 16 14" /></svg></span>
              <span>Watch History</span>
            </a>

            <button class="dropdown-item" onclick={openChangelog} role="menuitem">
              <span class="item-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="15" height="15" stroke-linecap="round" stroke-linejoin="round"><path d="M12 3l1.9 5.8a2 2 0 0 0 1.3 1.3L21 12l-5.8 1.9a2 2 0 0 0-1.3 1.3L12 21l-1.9-5.8a2 2 0 0 0-1.3-1.3L3 12l5.8-1.9a2 2 0 0 0 1.3-1.3z" /></svg></span>
              <span>What's New (v2.4.0)</span>
            </button>

            <div class="dropdown-divider"></div>

            <button class="dropdown-item logout-item" onclick={openProfileModal} role="menuitem">
              <span class="item-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="15" height="15" stroke-linecap="round" stroke-linejoin="round"><path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" /><polyline points="16 17 21 12 16 7" /><line x1="21" y1="12" x2="9" y2="12" /></svg></span>
              <span>Log Out</span>
            </button>
          </div>
        {/if}
      </div>

      <!-- Frameless window controls -->
      <div class="window-controls">
        <button class="win-btn" onclick={() => windowCtrl("minimize")} aria-label="Minimize" title="Minimize">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="14" height="14" stroke-linecap="round"><line x1="5" y1="12" x2="19" y2="12" /></svg>
        </button>
        <button class="win-btn" onclick={() => windowCtrl("maximize")} aria-label="Maximize" title="Maximize">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="12" height="12" stroke-linecap="round" stroke-linejoin="round"><rect x="4" y="4" width="16" height="16" rx="2" /></svg>
        </button>
        <button class="win-btn close" onclick={() => windowCtrl("close")} aria-label="Close" title="Close">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="14" height="14" stroke-linecap="round"><line x1="18" y1="6" x2="6" y2="18" /><line x1="6" y1="6" x2="18" y2="18" /></svg>
        </button>
      </div>
    </div>
  </div>
</header>

<style>
  .navbar {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    z-index: 1000;
    height: 64px;
    background: rgba(10, 10, 14, 0.6);
    -webkit-backdrop-filter: blur(18px) saturate(140%);
    backdrop-filter: blur(18px) saturate(140%);
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
    pointer-events: auto;
  }

  .window-controls {
    display: flex;
    align-items: center;
    gap: 2px;
    margin-left: 6px;
  }

  .win-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 34px;
    height: 30px;
    border-radius: 4px;
    color: #c4c4cb;
    background: transparent;
    border: none;
    cursor: pointer;
    transition: background 0.12s ease, color 0.12s ease;
  }

  .win-btn:hover {
    background: rgba(255, 255, 255, 0.1);
    color: #ffffff;
  }

  .win-btn.close:hover {
    background: #e81123;
    color: #ffffff;
  }

  .nav-inner {
    max-width: 1680px;
    margin: 0 auto;
    padding: 0 36px;
    height: 100%;
    display: grid;
    grid-template-columns: auto 1fr auto;
    align-items: center;
    gap: 32px;
  }

  .nav-left {
    display: flex;
    align-items: center;
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 12px;
    text-decoration: none;
  }

  .cr-logo {
    width: 32px;
    height: 32px;
    flex-shrink: 0;
  }

  .brand-text {
    font-size: 24px;
    font-weight: 900;
    letter-spacing: -0.03em;
    color: var(--accent, #a855f7);
    font-family: inherit;
    line-height: 1;
  }

  /* Centered Navigation Links */
  .nav-center {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    gap: 32px;
  }

  .nav-link {
    position: relative;
    display: inline-flex;
    align-items: center;
    height: 100%;
    padding: 0 4px;
    font-size: 15px;
    font-weight: 800;
    letter-spacing: 0.06em;
    color: #b0b0b8;
    text-transform: uppercase;
    text-decoration: none;
    transition: color 0.15s ease;
    background: transparent;
    border: none;
    cursor: pointer;
    font-family: inherit;
  }

  .nav-link:hover {
    color: #ffffff;
  }

  .nav-link.active {
    color: #ffffff;
  }

  .active-indicator {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    height: 3px;
    background: #a855f7;
    border-radius: 2px 2px 0 0;
  }

  /* Right Icons */
  .nav-right {
    display: flex;
    align-items: center;
    gap: 14px;
  }

  .nav-icon-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 38px;
    height: 38px;
    border-radius: 4px; /* Crisp, not rounded bubble */
    color: #c4c4cb;
    background: transparent;
    border: none;
    cursor: pointer;
    text-decoration: none;
    transition: color 0.15s ease, background 0.15s ease;
  }

  .nav-icon-btn:hover {
    color: #a855f7;
    background: rgba(255, 255, 255, 0.06);
  }

  .bell-icon-btn {
    position: relative;
  }

  .unread-dot {
    position: absolute;
    top: 6px;
    right: 6px;
    width: 16px;
    height: 16px;
    background: #a855f7;
    color: #000000;
    font-size: 10px;
    font-weight: 900;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    line-height: 1;
  }

  /* Profile Pill Button */
  .dropdown-anchor {
    position: relative;
  }

  .profile-pill-btn {
    display: flex;
    align-items: center;
    gap: 9px;
    padding: 4px 10px 4px 6px;
    border-radius: 4px; /* Crisp border radius */
    background: #181920;
    border: 1px solid #282a33;
    cursor: pointer;
    transition: background 0.12s ease, border-color 0.12s ease;
    font-family: inherit;
  }

  .profile-pill-btn:hover {
    background: #22242e;
    border-color: #a855f7;
  }

  .avatar-ring {
    width: 28px;
    height: 28px;
    border-radius: 4px;
    overflow: hidden;
    border: 2px solid #a855f7;
    flex-shrink: 0;
  }

  .avatar-img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .profile-name-text {
    font-size: 13.5px;
    font-weight: 800;
    color: #ffffff;
  }

  .chevron-icon {
    width: 12px;
    height: 12px;
    color: #8c8c94;
  }

  /* Dropdown Menus */
  .profile-dropdown-menu {
    position: absolute;
    top: calc(100% + 10px);
    right: 0;
    width: 250px;
    background: #141519;
    border: 1px solid #282a32;
    border-radius: 4px;
    box-shadow: 0 16px 40px rgba(0, 0, 0, 0.9);
    padding: 8px 0;
    z-index: 1200;
    animation: dropIn 0.12s ease-out;
  }

  .notifs-menu {
    position: absolute;
    top: calc(100% + 10px);
    right: -70px;
    width: 360px;
    background: #141519;
    border: 1px solid #282a32;
    border-radius: 4px;
    box-shadow: 0 16px 40px rgba(0, 0, 0, 0.9);
    z-index: 1200;
    animation: dropIn 0.12s ease-out;
  }

  @keyframes dropIn {
    from {
      opacity: 0;
      transform: translateY(-6px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .profile-summary {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 16px;
  }

  .summary-avatar {
    width: 38px;
    height: 38px;
    border-radius: 4px;
    object-fit: cover;
  }

  .summary-name {
    font-size: 14px;
    font-weight: 800;
    color: #ffffff;
  }

  .summary-status {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 11px;
    color: #10b981;
    font-weight: 700;
  }

  .status-indicator {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: #10b981;
  }

  .dropdown-divider {
    height: 1px;
    background: #20222b;
    margin: 6px 0;
  }

  .dropdown-item {
    display: flex;
    align-items: center;
    gap: 12px;
    width: 100%;
    padding: 10px 16px;
    background: transparent;
    border: none;
    color: #cfcfd5;
    font-size: 13.5px;
    font-weight: 700;
    text-align: left;
    cursor: pointer;
    font-family: inherit;
    text-decoration: none;
    transition: background 0.12s ease, color 0.12s ease;
  }

  .dropdown-item:hover {
    background: #1d1f27;
    color: #ffffff;
  }

  .item-icon {
    font-size: 14px;
  }

  .logout-item {
    color: #ef4444;
  }

  .logout-item:hover {
    background: rgba(239, 68, 68, 0.12);
    color: #f87171;
  }

  /* Notification Menu Details */
  .menu-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    border-bottom: 1px solid #23252e;
  }

  .menu-title {
    font-size: 13.5px;
    font-weight: 800;
    color: #ffffff;
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .mark-read-btn {
    background: transparent;
    border: none;
    color: #a855f7;
    font-size: 11.5px;
    font-weight: 700;
    cursor: pointer;
  }

  .mark-read-btn:hover {
    text-decoration: underline;
  }

  .notifs-list {
    max-height: 340px;
    overflow-y: auto;
  }

  .notif-card {
    display: flex;
    gap: 12px;
    padding: 12px 16px;
    border-bottom: 1px solid #1c1d25;
    cursor: pointer;
    transition: background 0.12s ease;
  }

  .notif-card:hover {
    background: #191b22;
  }

  .notif-card.unread {
    background: rgba(168, 85, 247, 0.06);
    border-left: 3px solid #a855f7;
  }

  .notif-img {
    width: 44px;
    height: 60px;
    border-radius: 3px;
    object-fit: cover;
    flex-shrink: 0;
  }

  .notif-body {
    flex: 1;
  }

  .notif-heading {
    display: flex;
    justify-content: space-between;
    margin-bottom: 3px;
  }

  .notif-title {
    font-size: 13px;
    font-weight: 800;
    color: #ffffff;
  }

  .notif-time {
    font-size: 11px;
    color: #72737c;
  }

  .notif-text {
    margin: 0;
    font-size: 12px;
    color: #92939c;
    line-height: 1.4;
  }

  .menu-footer {
    padding: 10px 16px;
    border-top: 1px solid #23252e;
    background: #101115;
    text-align: center;
  }

  .menu-footer-link {
    color: #a855f7;
    font-size: 12px;
    font-weight: 700;
    text-decoration: none;
  }

  .menu-footer-link:hover {
    text-decoration: underline;
  }
</style>
