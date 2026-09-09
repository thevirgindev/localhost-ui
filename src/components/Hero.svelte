<script lang="ts">
  import type { AnimeCard } from "../lib/types";
  import { router } from "../lib/router";
  import { formatDesc } from "../lib/format";

  let { cards }: { cards: AnimeCard[] } = $props();

  let index = $state(0);
  let timer: ReturnType<typeof setInterval> | undefined = undefined;

  const slides = $derived(cards.slice(0, 5));
  const total = $derived(slides.length);
  const current = $derived(slides[index]);

  function startTimer() {
    if (timer) clearInterval(timer);
    if (total > 1) {
      timer = setInterval(() => {
        index = (index + 1) % total;
      }, 7000);
    }
  }

  $effect(() => {
    startTimer();
    return () => {
      if (timer) clearInterval(timer);
    };
  });

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
    <div class="shade"></div>

    <div class="hero-inner">
      <div class="content">
        <div class="eyebrow">FEATURED SERIES</div>
        <h1>{current.title}</h1>
        <p class="desc">
          {formatDesc(current.description).slice(0, 240)}{formatDesc(current.description).length > 240 ? "…" : ""}
        </p>
        <div class="actions">
          <button class="btn primary" onclick={() => router.navigate({ name: "details", id: current.id })}>
            <svg viewBox="0 0 24 24" fill="currentColor" width="16" height="16">
              <path d="M8 5v14l11-7z" />
            </svg>
            Start Watching
          </button>
          <button class="btn outline" onclick={() => router.navigate({ name: "details", id: current.id })}>
            Details
          </button>
        </div>
      </div>

      <div class="dashes">
        {#each slides as _, i}
          <button
            class="dash"
            class:active={i === index}
            onclick={() => pick(i)}
            aria-label="Slide {i + 1}"
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
    min-height: 500px;
    max-height: 680px;
    overflow: hidden;
    background: var(--surface);
    margin-bottom: 24px;
  }

  .slide {
    position: absolute;
    inset: 0;
    background-size: cover;
    background-position: center 25%;
    opacity: 0;
    transition: opacity 0.8s ease-in-out;
    transform: scale(1.02);
  }

  .slide.visible {
    opacity: 1;
  }

  .shade {
    position: absolute;
    inset: 0;
    background:
      linear-gradient(to top, #0d0d0d 4%, rgba(13, 13, 13, 0.6) 45%, rgba(13, 13, 13, 0.15) 85%),
      linear-gradient(to right, rgba(13, 13, 13, 0.95) 0%, rgba(13, 13, 13, 0.55) 45%, transparent 75%);
  }

  .hero-inner {
    position: relative;
    height: 100%;
    max-width: 1560px;
    margin: 0 auto;
    padding: 0 44px 54px;
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
  }

  .content {
    max-width: 660px;
  }

  .eyebrow {
    font-size: 11.5px;
    font-weight: 800;
    letter-spacing: 0.16em;
    color: var(--accent);
    margin-bottom: 12px;
  }

  h1 {
    font-size: 46px;
    line-height: 1.05;
    letter-spacing: -0.028em;
    margin-bottom: 14px;
    color: var(--text);
    text-shadow: 0 2px 20px rgba(0, 0, 0, 0.8);
  }

  .desc {
    color: #e2e2e4;
    font-size: 14.5px;
    line-height: 1.6;
    margin: 0 0 26px;
    display: -webkit-box;
    -webkit-line-clamp: 3;
    line-clamp: 3;
    -webkit-box-orient: vertical;
    overflow: hidden;
    text-shadow: 0 1px 8px rgba(0, 0, 0, 0.7);
  }

  .actions {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .dashes {
    position: absolute;
    right: 44px;
    bottom: 54px;
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .dash {
    width: 22px;
    height: 4px;
    border-radius: 999px;
    background: rgba(255, 255, 255, 0.28);
    padding: 0;
    transition: all 0.25s ease;
    cursor: pointer;
  }

  .dash:hover {
    background: rgba(255, 255, 255, 0.5);
  }

  .dash.active {
    width: 34px;
    background: var(--accent);
  }
</style>
