<script lang="ts">
  // Topbar with search + window controls area. Uses native decorations,
  // so this is just search.
  import { router } from "../lib/router";

  let query = $state("");

  function submit(e: SubmitEvent) {
    e.preventDefault();
    if (!query.trim()) return;
    router.navigate({ name: "browse" });
    // Give Browse page the query through the hash param.
    window.location.hash = `#/browse?q=${encodeURIComponent(query.trim())}`;
    query = "";
  }
</script>

<header class="topbar">
  <form class="search" onsubmit={submit}>
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="search-icon">
      <circle cx="11" cy="11" r="8" />
      <line x1="21" y1="21" x2="16.65" y2="16.65" />
    </svg>
    <input
      bind:value={query}
      type="text"
      placeholder="Search anime…"
      spellcheck="false"
    />
    <kbd>/</kbd>
  </form>
</header>

<style>
  .topbar {
    height: var(--topbar-h);
    min-height: var(--topbar-h);
    display: flex;
    align-items: center;
    padding: 0 28px;
    border-bottom: 1px solid var(--border-soft);
    background: var(--bg-raise);
  }

  .search {
    position: relative;
    width: min(420px, 100%);
  }

  .search input {
    width: 100%;
    padding: 9px 40px 9px 38px;
    border-radius: 8px;
    font-size: 13.5px;
    background: var(--surface);
  }

  .search-icon {
    position: absolute;
    left: 12px;
    top: 50%;
    transform: translateY(-50%);
    width: 16px;
    height: 16px;
    color: var(--text-faint);
    pointer-events: none;
  }

  .search kbd {
    position: absolute;
    right: 10px;
    top: 50%;
    transform: translateY(-50%);
    background: var(--surface-2);
    border: 1px solid var(--border);
    border-radius: 4px;
    padding: 1px 7px;
    font-size: 11px;
    color: var(--text-faint);
    pointer-events: none;
  }
</style>
