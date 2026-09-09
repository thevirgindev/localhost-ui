<script lang="ts">
  import type { AnimeCard } from "../lib/types";
  import { router } from "../lib/router";
  import { formatDesc } from "../lib/format";

  let { cards }: { cards: AnimeCard[] } = $props();

  let index = $state(0);
  let timer: ReturnType<typeof setInterval> | undefined = undefined;

  const total = $derived(cards.length);
  const current = $derived(cards[index]);

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
    {#each cards.slice(0, 5) as card, i (card.id)}
      <div
        class="slide"
        class:visible={i === index}
        style={card.cover ? `background-image: url('${card.cover}')` : ""}
      ></div>
    {/each}
    <div class="shade"></div>

    <div class="hero-inner">
      <div class="content">
        <div class="eyebrow">FEATURED SERIES</div>
        <h1>{current.title}</h1>
        <p class="desc">
          {formatDesc(current.description).slice(0, 230)}{formatDesc(current.description).length > 230 ? "…" : ""}
        </p>
        <div class="actions">
          <button class="btn primary" onclick={() => router.navigate({ name: "details", id: current.id })}>
            <svg viewBox="0 0 24 24" fill="currentColor" width="15" height="15"><path d="M8 5v14l11-7z" /></svg>
            Start Watching
          </button>
          <button class="btn outline" onclick={() => router.navigate({ name: "details", id: current.id })}>
            Details
          </button>
        </div>
      </div>

      <div class="dots">
        {#each cards.slice(0, 5) as _, i}
          <button class="dot" class:on={i === index} onclick={() => pick(i)} aria-label="Slide {i + 1}"></button>
        {/each}
      </div>
    </div>
  </section>
{/if}

<style>
  .hero {
    position: relative;
    height: 66vh;
    min-height: 460px;
    overflow: hidden;
    background: var(--surface);
  }

  .slide {
    position: absolute;
    inset: 0;
    background-size: cover;
    background-position: center 22%;
    opacity: 0;
    transition: opacity 0.7s ease;
    transform: scale(1.02);
  }

  .slide.visible {
    opacity: 1;
  }

  .shade {
    position: absolute;
    inset: 0;
    background:
      linear-gradient(to top, #0d0d0d 4%, rgba(13, 13, 13, 0.5) 48%, rgba(13, 13, 13, 0.12) 80%),
      linear-gradient(to right, rgba(13, 13, 13, 0.9) 0%, rgba(13, 13, 13, 0.45) 42%, transparent 70%);
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
    max-width: 640px;
  }

  .eyebrow {
    font-size: 12px;
    font-weight: 800;
    letter-spacing: 0.18em;
    color: var(--accent);
    margin-bottom: 12px;
  }

  h1 {
    font-size: 44px;
    line-height: 1.04;
    letter-spacing: -0.028em;
    margin-bottom: 14px;
    text-shadow: 0 2px 18px rgba(0, 0, 0, 0.7);
  }

  .desc {
    color: #d9d9db;
    font-size: 14px;
    line-height: 1.6;
    margin: 0 0 24px;
    display: -webkit-box;
    -webkit-line-clamp: 3;
    line-clamp: 3;
    -webkit-box-orient: vertical;
    overflow: hidden;
    text-shadow: 0 1px 8px rgba(0, 0, 0, 0.65);
  }

  .actions {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .dots {
    position: absolute;
    right: 44px;
    bottom: 60px;
    display: flex;
    gap: 7px;
  }

  .dot {
    width: 22px;
    height: 4px;
    border-radius: 2px;
    background: rgba(255, 255, 255, 0.3);
    padding: 0;
    transition: background 0.2s ease;
  }

  .dot.on {
    background: var(--accent);
  }
</style>
