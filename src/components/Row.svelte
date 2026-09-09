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
    scroller.scrollBy({ left: dir * scroller.clientWidth * 0.85, behavior: "smooth" });
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
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.6"><polyline points="15 18 9 12 15 6" /></svg>
      </button>
      <button class="arrow" onclick={() => scrollBy(1)} aria-label="Scroll right">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.6"><polyline points="9 18 15 12 9 6" /></svg>
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
    margin: 30px 0;
  }

  .row-actions {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .link {
    font-size: 12.5px;
    margin-right: 8px;
  }

  .arrow {
    width: 30px;
    height: 30px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
    background: var(--surface);
    color: var(--text-dim);
    transition: all 0.12s ease;
  }

  .arrow:hover {
    background: var(--surface-2);
    color: var(--text);
  }

  .arrow svg {
    width: 14px;
    height: 14px;
  }

  .scroller {
    display: flex;
    gap: 16px;
    overflow-x: auto;
    scroll-behavior: smooth;
    padding-bottom: 8px;
  }

  .scroller::-webkit-scrollbar {
    height: 8px;
  }

  .cell {
    width: 250px;
    min-width: 250px;
  }
</style>
