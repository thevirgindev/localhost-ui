<script lang="ts">
  import { onMount } from "svelte";
  import { api } from "../lib/api";
  import type { WatchlistItem } from "../lib/types";
  import { router } from "../lib/router";

  const TABS = [
    { value: "all", label: "All" },
    { value: "watching", label: "Watching" },
    { value: "planning", label: "Want to Watch" },
    { value: "completed", label: "Completed" },
    { value: "paused", label: "Paused" },
    { value: "dropped", label: "Dropped" },
  ];

  let items: WatchlistItem[] = $state([]);
  let tab = $state("all");
  let loading = $state(true);
  let error = $state("");

  async function load() {
    loading = true;
    try {
      items = await api.watchlist(tab === "all" ? undefined : tab);
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  onMount(load);

  function setTab(t: string) {
    tab = t;
    load();
  }

  async function remove(id: number) {
    await api.removeFromWatchlist(id);
    await load();
  }

  async function setStatus(id: number, status: string) {
    await api.setListStatus(id, status);
    await load();
  }

  const statusLabel: Record<string, string> = {
    watching: "Watching",
    planning: "Want to Watch",
    completed: "Completed",
    paused: "Paused",
    dropped: "Dropped",
  };

  let activeMenu: number | null = $state(null);
</script>

<div class="page">
  <h1 class="page-title">My List</h1>
  <p class="page-sub">Your personal watchlist — stored on this machine only.</p>

  <div class="chip-row">
    {#each TABS as t}
      <button class="chip" class:active={tab === t.value} onclick={() => setTab(t.value)}>{t.label}</button>
    {/each}
  </div>

  {#if loading}
    <div class="spinner"></div>
  {:else if error}
    <div class="error-box"><span>{error}</span><button class="btn" onclick={load}>Retry</button></div>
  {:else if items.length === 0}
    <div class="empty-state">
      <div class="big">☆</div>
      <p>Nothing here yet.</p>
      <p class="hint">Open any anime and press “Add to list”.</p>
    </div>
  {:else}
    <div class="list-rows">
      {#each items as item (item.animeId)}
        <div class="list-row">
          <button class="thumb-btn" onclick={() => router.navigate({ name: "details", id: item.animeId })}>
            {#if item.cover}
              <img class="thumb" src={item.cover} alt="" />
            {:else}
              <div class="thumb no-img">_</div>
            {/if}
          </button>
          <div class="row-main" onclick={() => router.navigate({ name: "details", id: item.animeId })}>
            <div class="row-title">{item.title}</div>
            <div class="row-sub">
              {#if item.episodesTotal}
                <span>{item.episodesTotal} ep</span>
              {/if}
              <span class="sep">·</span>
              <span>{item.progress > 0 ? `EP ${item.progress} watched` : "not started"}</span>
              {#if item.status === "RELEASING"}
                <span class="sep">·</span>
                <span class="airing">airing</span>
              {/if}
            </div>
            <div class="progress-track">
              <div class="progress-fill" style="width: {item.episodesTotal ? Math.min(100, (item.progress / item.episodesTotal) * 100) : item.progress > 0 ? 6 : 0}%"></div>
            </div>
          </div>
          <select
            class="status-select"
            value={item.listStatus}
            onchange={(e) => setStatus(item.animeId, (e.target as HTMLSelectElement).value)}
          >
            {#each Object.entries(statusLabel) as [value, label]}
              <option value={value}>{label}</option>
            {/each}
          </select>
          <button class="btn secondary remove-btn" onclick={() => remove(item.animeId)} title="Remove">
            ✕
          </button>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .chip-row {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
    margin-bottom: 24px;
  }

  .list-rows {
    display: flex;
    flex-direction: column;
    gap: 10px;
    max-width: 980px;
  }

  .list-row {
    display: flex;
    align-items: center;
    gap: 14px;
    background: var(--surface);
    border: 1px solid var(--border-soft);
    border-radius: var(--radius);
    padding: 10px;
  }

  .thumb {
    width: 52px;
    height: 72px;
    border-radius: 6px;
    object-fit: cover;
    cursor: pointer;
    background: var(--surface-2);
    flex-shrink: 0;
  }

  .no-img {
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-faint);
    font-weight: 800;
  }

  .row-main {
    flex: 1;
    min-width: 0;
    cursor: pointer;
  }

  .row-title {
    font-weight: 600;
    font-size: 14px;
    margin-bottom: 3px;
  }

  .row-sub {
    display: flex;
    gap: 8px;
    font-size: 12px;
    color: var(--text-dim);
    margin-bottom: 8px;
  }

  .sep {
    opacity: 0.5;
  }

  .airing {
    color: var(--green);
    font-weight: 600;
  }

  .progress-track {
    height: 4px;
    background: var(--surface-2);
    border-radius: 2px;
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background: var(--accent);
    border-radius: 2px;
  }

  .status-select {
    font-size: 12.5px;
    padding: 7px 10px;
  }

  .remove-btn {
    padding: 7px 11px;
    font-size: 13px;
  }

  .thumb-btn {
    flex-shrink: 0;
    border-radius: 6px;
  }
</style>
