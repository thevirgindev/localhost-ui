<script lang="ts">
  import { onMount } from "svelte";
  import { api } from "../lib/api";
  import { reportError } from "../lib/ipc";
  import type { AnimeCard, SeasonInfo } from "../lib/types";
  import AnimeCardItem from "../components/AnimeCard.svelte";

  const SEASONS = ["WINTER", "SPRING", "SUMMER", "FALL"];

  let info: SeasonInfo | null = $state(null);
  let season = $state("");
  let year = $state(0);
  let cards: AnimeCard[] = $state([]);
  let loading = $state(true);
  let error = $state("");

  onMount(async () => {
    try {
      info = await api.seasonInfo();
      season = info.season;
      year = info.year;
      await load();
    } catch (e) {
      error = String(e);
      reportError("SeasonPage.init", e);
      loading = false;
    }
  });

  async function load() {
    loading = true;
    error = "";
    try {
      cards = await api.season(year, season, 1);
    } catch (e) {
      error = String(e);
      reportError("SeasonPage.load", e);
    } finally {
      loading = false;
    }
  }

  function pickSeason(s: string) {
    season = s;
    load();
  }

  function shiftYear(delta: number) {
    year += delta;
    load();
  }

  function isNow(s: string, y: number): boolean {
    return info != null && info.season === s && info.year === y;
  }

  const seasonLabel = $derived(season.charAt(0) + season.slice(1).toLowerCase());
</script>

<div class="page">
  <div class="head">
    <div>
      <h1 class="page-title">{seasonLabel} {year}</h1>
      <p class="page-sub">{info && isNow(season, year) ? "Current simulcast season" : "Season archive"}</p>
    </div>
    <div class="year-nav">
      <button class="year-btn" onclick={() => shiftYear(-1)} aria-label="Previous year">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" width="16" height="16">
          <polyline points="15 18 9 12 15 6" />
        </svg>
      </button>
      <span class="year">{year}</span>
      <button class="year-btn" onclick={() => shiftYear(1)} aria-label="Next year">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" width="16" height="16">
          <polyline points="9 18 15 12 9 6" />
        </svg>
      </button>
    </div>
  </div>

  <div class="chip-row">
    {#each SEASONS as s}
      <button class="chip" class:active={season === s} onclick={() => pickSeason(s)}>
        <span>{s.charAt(0) + s.slice(1).toLowerCase()}</span>
        {#if info && isNow(s, year)}
          <span class="airing-dot"></span>
        {/if}
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
  {:else if cards.length === 0}
    <div class="empty-state">
      <div class="big">∅</div>
      <p>No series listed for this season yet.</p>
    </div>
  {:else}
    <div class="grid">
      {#each cards as card (card.id)}
        <AnimeCardItem {card} />
      {/each}
    </div>
  {/if}
</div>

<style>
  .head {
    display: flex;
    align-items: flex-end;
    justify-content: space-between;
    margin-bottom: 8px;
  }

  .year-nav {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .year-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 36px;
    border-radius: 50%;
    background: var(--surface-2);
    border: 1px solid var(--border);
    color: var(--text-dim);
    transition: all 0.14s ease;
  }

  .year-btn:hover {
    background: var(--surface-3);
    color: var(--text);
    border-color: var(--text-faint);
  }

  .year {
    font-size: 18px;
    font-weight: 800;
    min-width: 54px;
    text-align: center;
  }

  .chip-row {
    display: flex;
    gap: 8px;
    margin: 18px 0 28px;
  }
</style>
