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
      <p class="page-sub">{info && isNow(season, year) ? "The current simulcast season" : "Season archive"}</p>
    </div>
    <div class="year-nav">
      <button class="btn outline" onclick={() => shiftYear(-1)} aria-label="Previous year">←</button>
      <span class="year">{year}</span>
      <button class="btn outline" onclick={() => shiftYear(1)} aria-label="Next year">→</button>
    </div>
  </div>

  <div class="chip-row">
    {#each SEASONS as s}
      <button class="chip" class:active={season === s} onclick={() => pickSeason(s)}>
        {s.charAt(0) + s.slice(1).toLowerCase()}
        {#if info && isNow(s, year)}<span class="now-dot"></span>{/if}
      </button>
    {/each}
  </div>

  {#if loading}
    <div class="spinner"></div>
  {:else if error}
    <div class="error-box"><span>{error}</span><button class="btn secondary" onclick={load}>Retry</button></div>
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
  }

  .year-nav {
    display: flex;
    align-items: center;
    gap: 14px;
  }

  .year {
    font-size: 18px;
    font-weight: 800;
    min-width: 56px;
    text-align: center;
  }

  .chip-row {
    display: flex;
    gap: 8px;
    margin: 20px 0 24px;
  }

  .now-dot {
    display: inline-block;
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: var(--green);
    margin-left: 7px;
    vertical-align: middle;
  }
</style>
