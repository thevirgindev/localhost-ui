<script lang="ts">
  import AnimeCardItem from "./AnimeCard.svelte";
  import type { AnimeCard } from "../lib/types";

  let {
    title,
    cards,
    moreHref = null,
  }: { title: string; cards: AnimeCard[]; moreHref?: string | null } = $props();

  let scroller: HTMLDivElement | undefined = $state();

  function scrollBy(dir: number) {
    if (!scroller) return;
    scroller.scrollBy({ left: dir * scroller.clientWidth * 0.82, behavior: "smooth" });
  }
</script>

<section class="row-section">
  <div class="section-title">
    <h3>{title}</h3>
    <div class="row-actions">
      {#if moreHref}
        <a class="link" href={moreHref}>View all</a>
      {/if}
      <button class="arrow" onclick={() => scrollBy(-1)} aria-label="Scroll left">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
          <polyline points="15 18 9 12 15 6" />
        </svg>
      </button>
      <button class="arrow" onclick={() => scrollBy(1)} aria-label="Scroll right">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
          <polyline points="9 18 15 12 9 6" />
        </svg>
      </button>
    </div>
  </div>
  <div class="scroller" bind:this={scroller}>
    {#each cards as card (card.id)}
      <div class="cell">
        <AnimeCardItem {card} />
      </div>
    {/each}
  </div>
</section>

<style>
  .row-section {
    margin: 32px 0 38px;
  }

  .row-actions {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .link {
    font-size: 13px;
    font-weight: 700;
    color: var(--text-dim);
    margin-right: 12px;
    transition: color 0.12s ease;
  }

  .link:hover {
    color: var(--accent);
  }

  .arrow {
    width: 34px;
    height: 34px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
    background: var(--surface-2);
    border: 1px solid var(--border);
    color: var(--text-dim);
    transition: all 0.14s ease;
  }

  .arrow:hover {
    background: var(--surface-3);
    color: var(--text);
    border-color: var(--text-faint);
  }

  .arrow svg {
    width: 15px;
    height: 15px;
  }

  .scroller {
    display: flex;
    gap: 16px;
    overflow-x: auto;
    scroll-behavior: smooth;
    padding-bottom: 8px;
    scrollbar-width: thin;
  }

  .cell {
    width: 190px;
    min-width: 190px;
    flex-shrink: 0;
  }
</style>
