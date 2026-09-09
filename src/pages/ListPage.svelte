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
</script>

<div class="page">
  <h1 class="page-title">My List</h1>
  <p class="page-sub">Your personal watchlist and progress — saved locally on this machine.</p>

  <div class="chip-row">
    {#each TABS as t}
      <button class="chip" class:active={tab === t.value} onclick={() => setTab(t.value)}>
        {t.label}
      </button>
    {/each}
  </div>

  {#if loading}
    <div class="spinner"></div>
  {:else if error}
    <div class="error-box">
      <span>{error}</span>
      <button class="btn secondary" onclick={load}>Retry</button>
    </div>
  {:else if items.length === 0}
    <div class="empty-state">
      <div class="big">☆</div>
      <p>Your watchlist is empty.</p>
      <p class="hint">Browse titles and click “My List” to add anime here.</p>
    </div>
  {:else}
    <div class="list-rows">
      {#each items as item (item.animeId)}
        <div class="list-row">
          <button
            class="thumb-btn"
            onclick={() => router.navigate({ name: "details", id: item.animeId })}
            aria-label="View {item.title}"
          >
            {#if item.cover}
              <img class="thumb" src={item.cover} alt={item.title} loading="lazy" />
            {:else}
              <div class="thumb no-img">▶</div>
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
              onchange={(e) => setStatus(item.animeId, (e.target as HTMLSelectElement).value)}
            >
              {#each Object.entries(statusLabel) as [value, label]}
                <option value={value}>{label}</option>
              {/each}
            </select>

            <button
              class="remove-btn"
              onclick={() => remove(item.animeId)}
              title="Remove from list"
              aria-label="Remove from list"
            >
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" width="16" height="16">
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

<style>
  .chip-row {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
    margin-bottom: 28px;
  }

  .list-rows {
    display: flex;
    flex-direction: column;
    gap: 12px;
    max-width: 1040px;
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
    font-size: 15px;
    color: var(--text);
    margin-bottom: 4px;
    transition: color 0.12s ease;
  }

  .row-main:hover .row-title {
    color: var(--accent-hover);
  }

  .row-sub {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 12.5px;
    color: var(--text-dim);
    margin-bottom: 10px;
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
  }

  .remove-btn {
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

  .remove-btn:hover {
    color: #ff8585;
    background: #331818;
  }
</style>
