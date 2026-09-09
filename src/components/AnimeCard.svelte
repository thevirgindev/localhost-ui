<script lang="ts">
  import type { AnimeCard } from "../lib/types";
  import { router } from "../lib/router";

  let { card }: { card: AnimeCard } = $props();

  const formatLabel = $derived.by(() => {
    switch (card.format) {
      case "TV": return "Series";
      case "TV_SHORT": return "Series";
      case "MOVIE": return "Movie";
      case "OVA": return "OVA";
      case "ONA": return "ONA";
      case "SPECIAL": return "Special";
      default: return "";
    }
  });

  const isAiring = $derived(
    card.status === "RELEASING" || card.status === "Currently Airing",
  );
</script>

<button class="card" onclick={() => router.navigate({ name: "details", id: card.id })}>
  <div class="thumb">
    {#if formatLabel}
      <span class="format-tag">{formatLabel}</span>
    {/if}
    {#if card.cover}
      <img src={card.cover} alt={card.title} loading="lazy" />
    {:else}
      <div class="no-img">{card.title.charAt(0)}</div>
    {/if}
    <div class="hover-shade">
      <span class="play-pill">
        <svg viewBox="0 0 24 24" fill="currentColor" width="16" height="16"><path d="M8 5v14l11-7z" /></svg>
        Watch
      </span>
    </div>
  </div>
  <div class="meta">
    <div class="title" title={card.title}>{card.title}</div>
    <div class="sub">
      {#if isAiring}
        <span class="airing-dot"></span>
        <span class="airing-text">New episode</span>
      {:else if card.episodes}
        <span>{card.episodes} episodes</span>
      {:else if card.seasonYear}
        <span>{card.seasonYear}</span>
      {/if}
    </div>
  </div>
</button>

<style>
  .card {
    display: block;
    text-align: left;
    border-radius: var(--radius);
  }

  .thumb {
    position: relative;
    aspect-ratio: 16 / 9;
    overflow: hidden;
    background: var(--surface);
    border-radius: var(--radius);
    transition: transform 0.16s ease, box-shadow 0.16s ease;
  }

  .card:hover .thumb {
    transform: translateY(-4px);
    box-shadow: var(--shadow);
  }

  .thumb img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .no-img {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 28px;
    font-weight: 800;
    color: var(--text-faint);
  }

  .hover-shade {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(0, 0, 0, 0.42);
    opacity: 0;
    transition: opacity 0.15s ease;
  }

  .card:hover .hover-shade {
    opacity: 1;
  }

  .play-pill {
    display: inline-flex;
    align-items: center;
    gap: 7px;
    background: var(--accent);
    color: #1a0d00;
    font-size: 12.5px;
    font-weight: 800;
    padding: 9px 18px;
    border-radius: 999px;
  }

  .meta {
    padding: 10px 2px 0;
  }

  .title {
    font-size: 13.5px;
    font-weight: 700;
    line-height: 1.32;
    letter-spacing: -0.01em;
    overflow: hidden;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    line-clamp: 2;
    -webkit-box-orient: vertical;
    min-height: 36px;
  }

  .card:hover .title {
    color: var(--accent-hover);
  }

  .sub {
    display: flex;
    align-items: center;
    gap: 6px;
    margin-top: 4px;
    font-size: 11.5px;
    font-weight: 600;
    color: var(--text-faint);
  }

  .airing-dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: var(--green);
  }
</style>
