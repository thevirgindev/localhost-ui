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
      heroCards = t.slice(0, 5);
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

<div class="page home">
  {#if loading}
    <div class="spinner"></div>
  {:else if error}
    <div class="error-box">
      <span>Couldn't load the catalog: {error}</span>
      <button class="btn secondary" onclick={() => location.reload()}>Retry</button>
    </div>
  {:else}
    <Hero cards={heroCards} />

    {#if continueItems.length > 0}
      <section>
        <div class="section-title">
          <h3>Continue Watching</h3>
        </div>
        <div class="cw-grid">
          {#each continueItems.slice(0, 10) as item (item.animeId)}
            {@const lastEp = progressMap.get(item.animeId) ?? item.progress}
            <button
              class="cw-card"
              onclick={() => router.navigate({ name: "watch", id: item.animeId, episode: Math.max(1, lastEp) })}
            >
              {#if item.cover}
                <img src={item.cover} alt="" />
              {:else}
                <div class="cw-noimg">▶</div>
              {/if}
              <div class="cw-info">
                <div class="cw-title">{item.title}</div>
                <div class="cw-ep">
                  {lastEp >= (item.episodesTotal ?? Infinity) ? "Finished" : `Episode ${Math.max(1, lastEp)}`}
                </div>
              </div>
              <svg class="cw-play" viewBox="0 0 24 24" fill="currentColor"><path d="M8 5v14l11-7z" /></svg>
            </button>
          {/each}
        </div>
      </section>
    {/if}

    {#if trending.length}
      <Row title="Trending Now" cards={trending} />
    {/if}

    {#if seasonal.length}
      <Row title="Simulcast Season" cards={seasonal} moreHref="#/season" />
    {/if}

    {#if popular.length}
      <Row title="Popular" cards={popular} moreHref="#/browse" />
    {/if}
  {/if}
</div>

<style>
  .home {
    padding-top: 0;
  }

  .cw-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 12px;
  }

  .cw-card {
    display: flex;
    align-items: center;
    gap: 12px;
    background: var(--surface);
    border-radius: var(--radius);
    padding: 8px 14px 8px 8px;
    text-align: left;
    transition: background 0.12s ease;
  }

  .cw-card:hover {
    background: var(--surface-2);
  }

  .cw-card img,
  .cw-noimg {
    width: 84px;
    height: 48px;
    object-fit: cover;
    border-radius: 6px;
    background: var(--surface-2);
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--accent);
    font-weight: 800;
  }

  .cw-info {
    flex: 1;
    min-width: 0;
  }

  .cw-title {
    font-size: 13px;
    font-weight: 700;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .cw-ep {
    font-size: 11.5px;
    color: var(--text-faint);
    margin-top: 3px;
  }

  .cw-play {
    width: 22px;
    height: 22px;
    color: var(--text-faint);
    flex-shrink: 0;
  }

  .cw-card:hover .cw-play {
    color: var(--accent);
  }
</style>
