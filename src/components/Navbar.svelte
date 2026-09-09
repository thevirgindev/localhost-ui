<script lang="ts">
  import type { Route } from "../lib/router";

  let { route }: { route: Route } = $props();

  let query = $state("");
  let debounceTimer: ReturnType<typeof setTimeout> | undefined;

  const navItems = [
    { name: "home", label: "Home", hash: "#/" },
    { name: "browse", label: "Browse", hash: "#/browse" },
    { name: "season", label: "Seasonal", hash: "#/season" },
    { name: "list", label: "My List", hash: "#/list" },
    { name: "library", label: "Library", hash: "#/library" },
  ];

  function isActive(item: { name: string }): boolean {
    if (item.name === "home") return route.name === "home";
    return route.name === item.name;
  }

  function onSearchInput() {
    clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
      if (query.trim()) {
        window.location.hash = `#/browse?q=${encodeURIComponent(query.trim())}`;
      } else if (route.name === "browse") {
        window.location.hash = "#/browse";
      }
    }, 500);
  }

  function onSearchSubmit(e: SubmitEvent) {
    e.preventDefault();
    clearTimeout(debounceTimer);
    if (query.trim()) {
      window.location.hash = `#/browse?q=${encodeURIComponent(query.trim())}`;
    }
  }
</script>

<header class="navbar">
  <div class="nav-inner">
    <div class="left">
      <a class="brand" href="#/">
        <svg class="cr-logo" viewBox="0 0 28 28" fill="none">
          <circle cx="14" cy="14" r="14" fill="#F47521" />
          <path
            d="M17.5 14C17.5 15.933 15.933 17.5 14 17.5C12.067 17.5 10.5 15.933 10.5 14C10.5 12.067 12.067 10.5 14 10.5C15.933 10.5 17.5 12.067 17.5 14Z"
            fill="#0D0D0D"
          />
          <path
            d="M14 6C18.4183 6 22 9.58172 22 14C22 16.4853 20.8647 18.7061 19.0833 20.177C18.3582 17.3995 15.8291 15.3333 12.8333 15.3333C10.4398 15.3333 8.35626 16.6575 7.29167 18.6083C6.4782 17.2974 6 15.7262 6 14C6 9.58172 9.58172 6 14 6Z"
            fill="#0D0D0D"
          />
        </svg>
        <span class="brand-text">localhost</span>
      </a>
      <nav class="nav-links">
        {#each navItems as item}
          <a class="nav-link" class:active={isActive(item)} href={item.hash}>
            {item.label}
            {#if isActive(item)}
              <span class="active-indicator"></span>
            {/if}
          </a>
        {/each}
      </nav>
    </div>

    <form class="search" onsubmit={onSearchSubmit}>
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" class="search-icon">
        <circle cx="11" cy="11" r="8" />
        <line x1="21" y1="21" x2="16.65" y2="16.65" />
      </svg>
      <input
        bind:value={query}
        oninput={onSearchInput}
        type="text"
        placeholder="Search anime…"
        spellcheck="false"
      />
    </form>

    <div class="right">
      <a class="icon-btn" class:active={route.name === "settings"} href="#/settings" title="Settings">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="18" height="18">
          <circle cx="12" cy="12" r="3" />
          <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" />
        </svg>
      </a>
    </div>
  </div>
</header>

<style>
  .navbar {
    position: sticky;
    top: 0;
    z-index: 50;
    background: rgba(20, 21, 25, 0.94);
    backdrop-filter: blur(16px);
    border-bottom: 1px solid var(--border);
    height: var(--nav-h);
  }

  .nav-inner {
    max-width: 1560px;
    margin: 0 auto;
    padding: 0 44px;
    height: 100%;
    display: flex;
    align-items: center;
    gap: 32px;
  }

  .left {
    display: flex;
    align-items: center;
    gap: 32px;
    flex: 1;
    min-width: 0;
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .cr-logo {
    width: 28px;
    height: 28px;
    flex-shrink: 0;
  }

  .brand-text {
    font-size: 20px;
    font-weight: 800;
    letter-spacing: -0.03em;
    color: var(--text);
  }

  .nav-links {
    display: flex;
    gap: 24px;
    height: var(--nav-h);
    align-items: center;
  }

  .nav-link {
    position: relative;
    font-size: 14px;
    font-weight: 600;
    color: var(--text-dim);
    transition: color 0.15s ease;
    height: 100%;
    display: flex;
    align-items: center;
  }

  .nav-link:hover {
    color: var(--accent);
  }

  .nav-link.active {
    color: var(--text);
    font-weight: 700;
  }

  .active-indicator {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    height: 3px;
    background: var(--accent);
    border-radius: 3px 3px 0 0;
  }

  .search {
    position: relative;
    width: 300px;
  }

  .search input {
    width: 100%;
    padding: 8px 16px 8px 38px;
    font-size: 13px;
    border-radius: 999px;
    background: var(--surface-2);
    border: 1px solid transparent;
  }

  .search input:focus {
    border-color: var(--accent);
    background: var(--surface-3);
  }

  .search-icon {
    position: absolute;
    left: 13px;
    top: 50%;
    transform: translateY(-50%);
    width: 15px;
    height: 15px;
    color: var(--text-faint);
    pointer-events: none;
  }

  .right {
    display: flex;
    align-items: center;
  }

  .icon-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 38px;
    height: 38px;
    border-radius: 50%;
    color: var(--text-dim);
    transition: color 0.15s ease, background 0.15s ease;
  }

  .icon-btn:hover {
    color: var(--text);
    background: var(--surface-2);
  }

  .icon-btn.active {
    color: var(--accent);
    background: var(--surface-2);
  }
</style>
