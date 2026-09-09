<script lang="ts">
  import { onMount } from "svelte";
  import type { AnimeCard } from "../lib/types";
  import { router } from "../lib/router";
  import { api } from "../lib/api";

  let { card }: { card: AnimeCard } = $props();

  let isWatchlisted = $state(false);

  onMount(() => {
    api.inWatchlist(card.id).then((inList) => {
      isWatchlisted = inList;
    }).catch(() => {});
  });

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

  async function toggleWatchlist(e: MouseEvent) {
    e.stopPropagation();
    try {
      if (isWatchlisted) {
        await api.removeFromWatchlist(card.id);
        isWatchlisted = false;
      } else {
        await api.addToWatchlist(card);
        isWatchlisted = true;
      }
    } catch {
      // Fallback
    }
  }

  function navigateToDetails() {
    router.navigate({ name: "details", id: card.id });
  }
</script>

<div
  class="card"
  data-anime-card="true"
  data-anime-id={card.id}
  data-anime-title={card.title}
  data-anime-cover={card.cover || ""}
  role="button"
  tabindex="0"
  onclick={navigateToDetails}
  onkeydown={(e) => (e.key === "Enter" || e.key === " ") && navigateToDetails()}
>
  <div class="thumb">
    {#if formatLabel}
      <span class="format-tag">{formatLabel}</span>
    {/if}

    <!-- Quick Watchlist Toggle -->
    <button
      type="button"
      class="quick-bookmark"
      class:active={isWatchlisted}
      onclick={toggleWatchlist}
      title={isWatchlisted ? "Remove from Watchlist" : "Add to Watchlist"}
      aria-label={isWatchlisted ? "Remove from Watchlist" : "Add to Watchlist"}
    >
      {#if isWatchlisted}
        <svg viewBox="0 0 24 24" fill="currentColor" width="13" height="13">
          <path d="M19 21l-7-5-7 5V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2z" />
        </svg>
      {:else}
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" width="13" height="13">
          <path d="M19 21l-7-5-7 5V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2z" />
        </svg>
      {/if}
    </button>

    {#if card.cover}
      <img src={card.cover} alt={card.title} loading="lazy" />
    {:else}
      <div class="no-img">{card.title.charAt(0)}</div>
    {/if}

    {#if card.averageScore}
      <div class="score-badge">
        <svg viewBox="0 0 24 24" fill="currentColor" width="12" height="12"><path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01z"/></svg> {card.averageScore}%
      </div>
    {/if}

    <div class="hover-shade">
      <span class="play-pill">
        <svg viewBox="0 0 24 24" fill="currentColor" width="14" height="14">
          <path d="M8 5v14l11-7z" />
        </svg>
        Watch
      </span>
    </div>
  </div>

  <div class="meta">
    <div class="title" title={card.title}>{card.title}</div>
    <div class="sub">
      {#if isAiring}
        <span class="airing-pulse"></span>
        <span class="airing-text">New Episode</span>
      {:else if formatLabel}
        <span>{formatLabel} • Subtitled</span>
      {:else}
        <span>Subtitled</span>
      {/if}
    </div>
  </div>
</div>

<style>
  .card {
    display: block;
    text-align: left;
    width: 100%;
    cursor: pointer;
    background: transparent;
    border: none;
    padding: 0;
    outline: none;
    user-select: none;
  }

  .thumb {
    position: relative;
    aspect-ratio: 2 / 3;
    overflow: hidden;
    background: var(--surface);
    border-radius: var(--radius);
    border: 1px solid var(--border);
    transition: transform 0.22s cubic-bezier(0.16, 1, 0.3, 1),
                box-shadow 0.22s cubic-bezier(0.16, 1, 0.3, 1),
                border-color 0.2s ease;
  }

  /* Sleek, refined hover highlight - NO tacky orange glow */
  .card:hover .thumb,
  .card:focus-visible .thumb {
    transform: translateY(-4px);
    border-color: rgba(255, 255, 255, 0.35);
    box-shadow: 0 14px 28px rgba(0, 0, 0, 0.55);
  }

  .thumb img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    transition: transform 0.3s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .card:hover .thumb img,
  .card:focus-visible .thumb img {
    transform: scale(1.03);
  }

  .no-img {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 32px;
    font-weight: 800;
    color: var(--text-faint);
  }

  .format-tag {
    position: absolute;
    top: 8px;
    left: 8px;
    background: #a855f7;
    color: #0d0d0d;
    font-size: 10px;
    font-weight: 800;
    padding: 3px 7px;
    border-radius: 3px;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    z-index: 3;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.6);
  }

  .quick-bookmark {
    position: absolute;
    top: 8px;
    right: 8px;
    z-index: 4;
    width: 28px;
    height: 28px;
    border-radius: 50%;
    background: rgba(16, 17, 22, 0.82);
    backdrop-filter: blur(4px);
    -webkit-backdrop-filter: blur(4px);
    border: 1px solid rgba(255, 255, 255, 0.15);
    color: #ffffff;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    opacity: 0;
    transform: scale(0.85);
    transition: opacity 0.18s ease, transform 0.18s cubic-bezier(0.16, 1, 0.3, 1), background 0.15s ease;
  }

  .quick-bookmark.active {
    opacity: 1;
    transform: scale(1);
    background: #a855f7;
    color: #0d0d0d;
    border-color: #a855f7;
  }

  .card:hover .quick-bookmark,
  .card:focus-visible .quick-bookmark {
    opacity: 1;
    transform: scale(1);
  }

  .quick-bookmark:hover {
    background: #c084fc;
    color: #0d0d0d;
    transform: scale(1.1) !important;
  }

  .score-badge {
    position: absolute;
    bottom: 8px;
    right: 8px;
    background: rgba(14, 15, 19, 0.85);
    backdrop-filter: blur(4px);
    -webkit-backdrop-filter: blur(4px);
    border: 1px solid rgba(255, 255, 255, 0.12);
    color: #ffffff;
    font-size: 10px;
    font-weight: 700;
    padding: 2px 6px;
    border-radius: 4px;
    z-index: 2;
    transition: opacity 0.15s ease;
  }

  .card:hover .score-badge {
    opacity: 0;
  }

  .hover-shade {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    background: radial-gradient(circle at center, rgba(13, 13, 13, 0.25) 0%, rgba(13, 13, 13, 0.75) 100%);
    backdrop-filter: blur(1.5px);
    -webkit-backdrop-filter: blur(1.5px);
    opacity: 0;
    transition: opacity 0.18s ease;
    z-index: 2;
  }

  .card:hover .hover-shade,
  .card:focus-visible .hover-shade {
    opacity: 1;
  }

  .play-pill {
    display: inline-flex;
    align-items: center;
    gap: 7px;
    background: #a855f7;
    color: #000000;
    font-size: 13px;
    font-weight: 800;
    padding: 8px 18px;
    border-radius: 999px;
    transform: scale(0.88) translateY(6px);
    opacity: 0;
    box-shadow: 0 6px 20px rgba(0, 0, 0, 0.65), 0 0 14px rgba(168, 85, 247, 0.45);
    transition: transform 0.22s cubic-bezier(0.16, 1, 0.3, 1),
                opacity 0.18s ease,
                background 0.12s ease;
  }

  .card:hover .play-pill,
  .card:focus-visible .play-pill {
    transform: scale(1) translateY(0);
    opacity: 1;
  }

  .meta {
    padding: 10px 2px 0;
  }

  .title {
    font-size: 14px;
    font-weight: 700;
    line-height: 1.35;
    letter-spacing: -0.012em;
    overflow: hidden;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    line-clamp: 2;
    -webkit-box-orient: vertical;
    min-height: 38px;
    color: #ffffff;
    transition: color 0.14s ease;
  }

  .card:hover .title,
  .card:focus-visible .title {
    color: #a855f7;
  }

  .sub {
    display: flex;
    align-items: center;
    gap: 6px;
    margin-top: 4px;
    font-size: 12px;
    font-weight: 600;
    color: #8c9099;
  }

  .meta-sep {
    color: #4a4d55;
  }

  .genre-preview {
    color: #a0a4af;
  }

  .airing-pulse {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: #a855f7;
    display: inline-block;
    flex-shrink: 0;
    box-shadow: 0 0 8px #a855f7;
    animation: pulse 1.8s infinite;
  }

  @keyframes pulse {
    0% {
      transform: scale(0.95);
      box-shadow: 0 0 0 0 rgba(168, 85, 247, 0.7);
    }
    70% {
      transform: scale(1.05);
      box-shadow: 0 0 0 6px rgba(168, 85, 247, 0);
    }
    100% {
      transform: scale(0.95);
      box-shadow: 0 0 0 0 rgba(168, 85, 247, 0);
    }
  }

  .airing-text {
    color: #cfcfd4;
    font-weight: 600;
  }
</style>
