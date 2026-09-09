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
      <a class="brand" href="#/">localhost</a>
      <nav class="nav-links">
        {#each navItems as item}
          <a class="nav-link" class:active={isActive(item)} href={item.hash}>{item.label}</a>
        {/each}
      </nav>
    </div>

    <form class="search" onsubmit={onSearchSubmit}>
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="search-icon">
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

    <a class="icon-link" class:active={route.name === "settings"} href="#/settings" title="Settings">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="18" height="18">
        <circle cx="12" cy="12" r="3" />
        <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" />
      </svg>
    </a>
  </div>
</header>

<style>
  .navbar {
    position: sticky;
    top: 0;
    z-index: 50;
    background: rgba(19, 19, 23, 0.92);
    backdrop-filter: blur(12px);
    border-bottom: 1px solid var(--border);
  }

  .nav-inner {
    max-width: 1400px;
    margin: 0 auto;
    padding: 0 40px;
    height: var(--nav-h);
    display: flex;
    align-items: center;
    gap: 28px;
  }

  .left {
    display: flex;
    align-items: center;
    gap: 28px;
    flex: 1;
    min-width: 0;
  }

  .brand {
    font-size: 19px;
    font-weight: 700;
    letter-spacing: -0.02em;
    color: var(--accent);
  }

  .nav-links {
    display: flex;
    gap: 22px;
  }

  .nav-link {
    font-size: 13.5px;
    color: var(--text-dim);
    transition: color 0.15s ease;
  }

  .nav-link:hover {
    color: var(--accent);
  }

  .nav-link.active {
    color: var(--text);
    font-weight: 600;
  }

  .search {
    position: relative;
    width: 280px;
  }

  .search input {
    width: 100%;
    padding: 8px 14px 8px 36px;
    font-size: 13px;
    border-radius: var(--radius);
    background: var(--surface);
  }

  .search-icon {
    position: absolute;
    left: 11px;
    top: 50%;
    transform: translateY(-50%);
    width: 15px;
    height: 15px;
    color: var(--text-faint);
    pointer-events: none;
  }

  .icon-link {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 36px;
    border-radius: var(--radius);
    color: var(--text-dim);
    transition: color 0.15s ease, background 0.15s ease;
  }

  .icon-link:hover {
    color: var(--accent);
    background: var(--surface-2);
  }

  .icon-link.active {
    color: var(--accent);
  }
</style>
