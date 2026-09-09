<script lang="ts">
  import type { Route } from "../lib/router";

  let { route }: { route: Route } = $props();

  const navItems = [
    { name: "home", label: "Home", icon: "home" },
    { name: "browse", label: "Browse", icon: "compass" },
    { name: "season", label: "This Season", icon: "calendar" },
    { name: "list", label: "My List", icon: "bookmark" },
    { name: "library", label: "Library", icon: "folder" },
    { name: "settings", label: "Settings", icon: "gear" },
  ] as const;

  function isActive(item: string): boolean {
    if (item === "home") return route.name === "home";
    if (item === "browse") return route.name === "browse";
    if (item === "season") return route.name === "season";
    if (item === "list") return route.name === "list";
    if (item === "library") return route.name === "library";
    if (item === "settings") return route.name === "settings";
    return false;
  }

  function go(name: string) {
    window.location.hash = name === "home" ? "#/" : `#/${name}`;
  }
</script>

<aside class="sidebar">
  <div class="logo" onclick={() => go("home")} role="button" tabindex="0">
    <span class="logo-mark">_</span>
    <span class="logo-text">localhost</span>
  </div>

  <nav>
    {#each navItems as item}
      <button
        class="nav-item"
        class:active={isActive(item.name)}
        onclick={() => go(item.name)}
      >
        <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          {#if item.icon === "home"}
            <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" />
            <polyline points="9 22 9 12 15 12 15 22" />
          {:else if item.icon === "compass"}
            <circle cx="12" cy="12" r="10" />
            <polygon points="16.24 7.76 14.12 14.12 7.76 16.24 9.88 9.88 16.24 7.76" />
          {:else if item.icon === "calendar"}
            <rect x="3" y="4" width="18" height="18" rx="2" ry="2" />
            <line x1="16" y1="2" x2="16" y2="6" />
            <line x1="8" y1="2" x2="8" y2="6" />
            <line x1="3" y1="10" x2="21" y2="10" />
          {:else if item.icon === "bookmark"}
            <path d="M19 21l-7-5-7 5V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2z" />
          {:else if item.icon === "folder"}
            <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z" />
          {:else if item.icon === "gear"}
            <circle cx="12" cy="12" r="3" />
            <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" />
          {/if}
        </svg>
        <span>{item.label}</span>
      </button>
    {/each}
  </nav>

  <div class="sidebar-foot">
    <div class="privacy-note">
      <span class="dot"></span>
      Local only · No tracking
    </div>
  </div>
</aside>

<style>
  .sidebar {
    width: var(--sidebar-w);
    min-width: var(--sidebar-w);
    background: var(--bg-raise);
    border-right: 1px solid var(--border-soft);
    display: flex;
    flex-direction: column;
    padding: 18px 12px;
  }

  .logo {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 4px 10px 18px;
    cursor: pointer;
  }

  .logo-mark {
    width: 26px;
    height: 26px;
    background: var(--accent);
    color: #14100c;
    font-weight: 800;
    font-size: 18px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 7px;
    padding-bottom: 6px;
  }

  .logo-text {
    font-weight: 700;
    font-size: 15px;
    letter-spacing: 0.2px;
  }

  nav {
    display: flex;
    flex-direction: column;
    gap: 2px;
    flex: 1;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 10px 12px;
    border-radius: var(--radius-sm);
    color: var(--text-dim);
    font-size: 13.5px;
    font-weight: 500;
    transition: all 0.1s ease;
    text-align: left;
  }

  .nav-item:hover {
    background: var(--surface);
    color: var(--text);
  }

  .nav-item.active {
    background: var(--surface-2);
    color: var(--accent);
  }

  .icon {
    width: 17px;
    height: 17px;
    flex-shrink: 0;
  }

  .sidebar-foot {
    padding: 12px 10px 4px;
  }

  .privacy-note {
    display: flex;
    align-items: center;
    gap: 7px;
    font-size: 11px;
    color: var(--text-faint);
  }

  .dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: var(--green);
  }
</style>
