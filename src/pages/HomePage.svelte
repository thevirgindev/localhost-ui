<script lang="ts">
  import { onMount } from "svelte";
  import { api } from "../lib/api";
  import { reportError } from "../lib/ipc";
  import type { AnimeCard, WatchlistItem } from "../lib/types";
  import Hero from "../components/Hero.svelte";
  import Row from "../components/Row.svelte";
  import { router } from "../lib/router";

  let heroCards: AnimeCard[] = $state([]);
  let trending: AnimeCard[] = $state([]);
  let popular: AnimeCard[] = $state([]);
  let seasonal: AnimeCard[] = $state([]);
  let continueItems: WatchlistItem[] = $state([]);
  let progressMap = $state(new Map<number, number>());
  let loading = $state(true);
  let error = $state("");

  onMount(async () => {
    try {
      const [t, p, info] = await Promise.all([api.trending(1), api.popular(1), api.seasonInfo()]);
      const s = await api.season(info.year, info.season, 1);

      trending = t.slice(0, 16);
      heroCards = t.slice(0, 6);
      popular = p.slice(0, 16);
      seasonal = s.slice(0, 16);

      continueItems = await api.continueWatching();
      if (continueItems.length > 0) {
        const entries = await Promise.all(continueItems.map((w) => api.progress(w.animeId)));
        const map = new Map<number, number>();
        continueItems.forEach((w, i) => {
          const last = entries[i].sort((a, b) => b.updatedAt - a.updatedAt)[0];
          if (last) map.set(w.animeId, last.episode);
        });
        progressMap = map;
      }
    } catch (e) {
      error = String(e);
      reportError("HomePage.load", e);
    } finally {
      loading = false;
    }
  });
</script>

<div class="home-view">
  {#if loading}
    <div class="spinner"></div>
  {:else if error}
    <div class="page">
      <div class="error-box">
        <span>Couldn't load catalog: {error}</span>
        <button class="btn secondary" onclick={() => location.reload()}>Retry</button>
      </div>
    </div>
  {:else}
    <Hero cards={heroCards} />

    <div class="page home-content">
      {#if continueItems.length > 0}
        <section class="cw-section">
          <div class="cw-header">
            <h3>Continue Watching</h3>
            <a class="cw-history-link" href="#/library?tab=continue">
              VIEW ALL <span class="chevron">›</span>
            </a>
          </div>

          <div class="cw-grid">
            {#each continueItems.slice(0, 6) as item (item.animeId)}
              {@const lastEp = progressMap.get(item.animeId) ?? item.progress}
              {@const percent = item.episodesTotal ? Math.min(100, Math.max(10, (lastEp / item.episodesTotal) * 100)) : 45}
              <button
                class="cw-card"
                onclick={() => router.navigate({ name: "watch", id: item.animeId, episode: Math.max(1, lastEp) })}
              >
                <div class="cw-thumb">
                  {#if item.cover}
                    <img src={item.cover} alt="" loading="lazy" />
                  {:else}
                    <div class="cw-noimg">▶</div>
                  {/if}

                  <div class="cw-hover-overlay">
                    <span class="cw-play-circle-white">
                      <svg viewBox="0 0 24 24" fill="currentColor">
                        <path d="M8 5v14l11-7z" />
                      </svg>
                    </span>
                  </div>

                  <div class="cw-progress-bar">
                    <div class="cw-progress-fill" style="width: {percent}%"></div>
                  </div>
                </div>

                <div class="cw-info">
                  <div class="cw-show-title" title={item.title}>{item.title}</div>
                  <div class="cw-ep-line">
                    Episode {Math.max(1, lastEp)}{item.episodesTotal ? ` of ${item.episodesTotal}` : ""}
                  </div>
                </div>
              </button>
            {/each}
          </div>
        </section>
      {/if}

      {#if trending.length}
        <Row title="Trending Now" cards={trending} />
      {/if}

      {#if seasonal.length}
        <Row title="This Season" cards={seasonal} moreHref="#/season" />
      {/if}

      {#if popular.length}
        <Row title="Most Popular" cards={popular} moreHref="#/browse" />
      {/if}
    </div>
  {/if}
</div>

<style>
  .home-view {
    padding-bottom: 60px;
  }

  .home-content {
    padding-top: 0;
  }

  .cw-section {
    margin: 8px 0 36px;
  }

  .cw-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 16px;
  }

  .cw-header h3 {
    font-size: 20px;
    letter-spacing: -0.02em;
    color: #ffffff;
    font-weight: 800;
  }

  .cw-history-link {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    font-size: 12px;
    font-weight: 800;
    color: var(--accent);
    letter-spacing: 0.08em;
    transition: color 0.13s ease;
  }

  .cw-history-link:hover {
    color: var(--accent-hover);
  }

  .chevron {
    font-size: 16px;
    line-height: 1;
  }

  .cw-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(240px, 1fr));
    gap: 16px;
  }

  .cw-card {
    display: block;
    text-align: left;
    width: 100%;
    cursor: pointer;
  }

  .cw-thumb {
    position: relative;
    aspect-ratio: 16 / 9;
    border-radius: var(--radius-sm);
    overflow: hidden;
    background: var(--surface-2);
    transition: outline 0.12s ease;
  }

  .cw-card:hover .cw-thumb {
    outline: 2px solid var(--accent);
    outline-offset: -2px;
  }

  .cw-thumb img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .cw-noimg {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--accent);
    font-size: 24px;
    background: var(--surface-2);
  }

  .cw-hover-overlay {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(0, 0, 0, 0.45);
    opacity: 0;
    transition: opacity 0.14s ease;
  }

  .cw-card:hover .cw-hover-overlay {
    opacity: 1;
  }

  .cw-play-circle-white {
    width: 44px;
    height: 44px;
    border-radius: 50%;
    background: var(--accent);
    color: #0d0d0d;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .cw-play-circle-white svg {
    width: 18px;
    height: 18px;
    margin-left: 2px;
  }

  .cw-progress-bar {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    height: 4px;
    background: rgba(0, 0, 0, 0.7);
  }

  .cw-progress-fill {
    height: 100%;
    background: var(--accent);
  }

  .cw-info {
    padding: 8px 1px 0;
  }

  .cw-show-title {
    font-size: 14px;
    font-weight: 700;
    color: #ffffff;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    letter-spacing: -0.01em;
    margin-bottom: 2px;
  }

  .cw-ep-line {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-dim);
  }

  .cw-card:hover .cw-show-title {
    color: var(--accent);
  }
</style>
