<script lang="ts">
  import type { AnimeCard } from "../lib/types";
  import { router } from "../lib/router";
  import { api } from "../lib/api";
  import { formatDesc } from "../lib/format";

  let { cards }: { cards: AnimeCard[] } = $props();

  let index = $state(0);
  let timer: ReturnType<typeof setInterval> | undefined = undefined;
  let inList = $state(false);

  const slides = $derived(cards.slice(0, 6));
  const total = $derived(slides.length);
  const current = $derived(slides[index]);

  function startTimer() {
    if (timer) clearInterval(timer);
    if (total > 1) {
      timer = setInterval(() => {
        index = (index + 1) % total;
      }, 8000);
    }
  }

  $effect(() => {
    startTimer();
    return () => {
      if (timer) clearInterval(timer);
    };
  });

  $effect(() => {
    if (current) {
      api.inWatchlist(current.id).then((res) => (inList = res)).catch(() => {});
    }
  });

  async function toggleWatchlist() {
    if (!current) return;
    if (inList) {
      await api.removeFromWatchlist(current.id);
      inList = false;
    } else {
      await api.addToWatchlist(current, "planning");
      inList = true;
    }
  }

  function pick(i: number) {
    index = i;
    startTimer();
  }
</script>

{#if current}
  <section class="hero">
    {#each slides as card, i (card.id)}
      <div
        class="slide"
        class:visible={i === index}
        style={card.banner || card.cover ? `background-image: url('${card.banner || card.cover}')` : ""}
      ></div>
    {/each}

    <!-- Gradient fade to dark background -->
    <div class="shade"></div>

    <div class="hero-inner">
      <div class="content">
        <div class="tag-row">
          <span class="cr-tag">Series</span>
          <span class="cr-tag">Sub | Dub</span>
          {#if current.format === "MOVIE"}
            <span class="cr-tag">Movie</span>
          {/if}
        </div>

        <h1 class="title">{current.title}</h1>

        {#if current.status === "RELEASING" || current.seasonYear}
          <div class="tagline">
            {current.status === "RELEASING" ? "New episodes weekly" : `Released ${current.seasonYear ?? ""}`}
          </div>
        {/if}

        <p class="desc">
          {formatDesc(current.description).slice(0, 240)}{formatDesc(current.description).length > 240 ? "…" : ""}
        </p>

        <div class="actions">
          <button
            class="btn primary watch-pill"
            onclick={() => router.navigate({ name: "watch", id: current.id, episode: 1 })}
          >
            <svg viewBox="0 0 24 24" fill="currentColor" width="16" height="16">
              <path d="M8 5v14l11-7z" />
            </svg>
            <span>START WATCHING</span>
          </button>

          <button
            class="btn secondary"
            onclick={() => router.navigate({ name: "details", id: current.id })}
          >
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" width="15" height="15">
              <circle cx="12" cy="12" r="10" />
              <line x1="12" y1="16" x2="12" y2="12" />
              <line x1="12" y1="8" x2="12.01" y2="8" />
            </svg>
            <span>DETAILS</span>
          </button>

          <button class="btn secondary" onclick={toggleWatchlist}>
            {#if inList}
              <svg viewBox="0 0 24 24" fill="currentColor" width="15" height="15" class="check">
                <path d="M9 16.2L4.8 12l-1.4 1.4L9 19 21 7l-1.4-1.4L9 16.2z" />
              </svg>
              <span>IN MY LIST</span>
            {:else}
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" width="15" height="15">
                <line x1="12" y1="5" x2="12" y2="19" />
                <line x1="5" y1="12" x2="19" y2="12" />
              </svg>
              <span>ADD TO LIST</span>
            {/if}
          </button>
        </div>
      </div>

      <!-- Slide dash indicators -->
      <div class="dash-row">
        {#each slides as _, i}
          <button
            class="dash"
            class:active={i === index}
            onclick={() => pick(i)}
            aria-label={`Slide ${i + 1}`}
          ></button>
        {/each}
      </div>
    </div>
  </section>
{/if}

<style>
  .hero {
    position: relative;
    height: 72vh;
    min-height: 520px;
    max-height: 660px;
    background: #0d0d0d;
    overflow: hidden;
  }

  .slide {
    position: absolute;
    inset: 0;
    background-size: cover;
    background-position: center 25%;
    opacity: 0;
    transition: opacity 0.7s ease-in-out;
  }

  .slide.visible {
    opacity: 1;
  }

  /* Solid matte gradient, no neon glow */
  .shade {
    position: absolute;
    inset: 0;
    background:
      linear-gradient(to top, #0d0d0d 0%, rgba(13, 13, 13, 0.75) 40%, rgba(13, 13, 13, 0.15) 85%),
      linear-gradient(to right, rgba(13, 13, 13, 0.96) 0%, rgba(13, 13, 13, 0.7) 45%, transparent 75%);
  }

  .hero-inner {
    position: relative;
    height: 100%;
    max-width: 1560px;
    margin: 0 auto;
    padding: 0 44px 50px;
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
  }

  .content {
    max-width: 620px;
  }

  .tag-row {
    display: flex;
    gap: 8px;
    margin-bottom: 14px;
  }

  .cr-tag {
    font-size: 11px;
    font-weight: 800;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: #ffffff;
    background: rgba(255, 255, 255, 0.16);
    padding: 3px 8px;
    border-radius: var(--radius-sm);
  }

  .title {
    font-size: 42px;
    line-height: 1.08;
    letter-spacing: -0.025em;
    font-weight: 800;
    color: #ffffff;
    margin: 0 0 10px;
    text-shadow: 0 2px 10px rgba(0, 0, 0, 0.9);
  }

  .tagline {
    font-size: 14px;
    font-weight: 700;
    color: var(--accent);
    margin-bottom: 12px;
    letter-spacing: -0.01em;
  }

  .desc {
    color: #c8c8cb;
    font-size: 14px;
    line-height: 1.55;
    margin: 0 0 24px;
    display: -webkit-box;
    -webkit-line-clamp: 3;
    line-clamp: 3;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  .actions {
    display: flex;
    align-items: center;
    gap: 12px;
    flex-wrap: wrap;
  }

  .watch-pill {
    padding: 11px 24px;
    font-size: 13px;
    letter-spacing: 0.04em;
  }

  .check {
    color: var(--accent);
  }

  .dash-row {
    position: absolute;
    right: 44px;
    bottom: 50px;
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .dash {
    width: 24px;
    height: 3px;
    background: rgba(255, 255, 255, 0.25);
    border-radius: 999px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .dash:hover {
    background: rgba(255, 255, 255, 0.5);
  }

  .dash.active {
    width: 36px;
    background: var(--accent);
  }
</style>
