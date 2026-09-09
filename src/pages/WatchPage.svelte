<script lang="ts">
  import { onDestroy } from "svelte";
  import type HlsType from "hls.js";
  import { api } from "../lib/api";
  import type { AnimeCard, EpisodeEntry } from "../lib/types";
  import { router } from "../lib/router";

  let { id, episode }: { id: number; episode: number } = $props();

  let card: AnimeCard | null = $state(null);
  let episodes: EpisodeEntry[] = $state([]);
  let source = $state("");
  let streamUrl = $state("");
  let isEmbed = $state(false);
  let loading = $state(true);
  let error = $state("");

  let videoEl: HTMLVideoElement | undefined = $state();
  let hls: HlsType | undefined;
  let saveTimer: ReturnType<typeof setInterval> | undefined;
  let progressDuration = $state(0);
  let wiredFor = $state("");

  function clearPlayback() {
    if (saveTimer) {
      clearInterval(saveTimer);
      saveTimer = undefined;
    }
    if (hls) {
      hls.destroy();
      hls = undefined;
    }
    if (videoEl) {
      videoEl.pause();
      videoEl.removeAttribute("src");
      videoEl.load();
    }
    progressDuration = 0;
    wiredFor = "";
  }

  onDestroy(clearPlayback);

  // Load show + episode data whenever the route changes.
  $effect(() => {
    const animeId = id;
    const ep = episode;
    loadEpisode(animeId, ep);
  });

  async function loadEpisode(animeId: number, epNumber: number) {
    loading = true;
    error = "";
    streamUrl = "";
    isEmbed = false;
    clearPlayback();
    try {
      if (!card) {
        card = await api.details(animeId);
      }
      if (episodes.length === 0) {
        const src = await api.episodes(animeId, card.title, card.titleEnglish);
        episodes = src.episodes;
        source = src.source;
      }

      const entry = episodes.find((e) => e.number === epNumber);
      if (!entry) {
        error = `Episode ${epNumber} is not available for this title.`;
        loading = false;
        return;
      }

      if (!entry.url) {
        error =
          "No playable source for this episode. Add the show's folder in Library for local playback.";
        loading = false;
        return;
      }

      if (entry.url.startsWith("/") || /^[A-Za-z]:[\\/]/.test(entry.url)) {
        const { convertFileSrc } = await import("@tauri-apps/api/core");
        streamUrl = convertFileSrc(entry.url);
      } else if (entry.url.includes("crunchyroll.com/embed")) {
        isEmbed = true;
        streamUrl = entry.url;
      } else {
        streamUrl = entry.url;
      }
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  // Attach the stream once the video element and URL are ready.
  $effect(() => {
    const el = videoEl;
    const url = streamUrl;
    if (!el || !url || isEmbed || wiredFor === url) return;
    wiredFor = url;
    if (url.endsWith(".m3u8")) {
      import("hls.js").then((mod) => {
        const Hls = mod.default;
        if (Hls.isSupported()) {
          hls = new Hls({ enableWorker: true });
          hls.loadSource(url);
          hls.attachMedia(el);
        } else if (el.canPlayType("application/vnd.apple.mpegurl")) {
          el.src = url;
        } else {
          error = "This browser cannot play HLS streams.";
        }
      });
    } else {
      el.src = url;
    }
  });

  async function onLoadedMetadata() {
    if (!videoEl) return;
    progressDuration = videoEl.duration || 0;
    try {
      const pos = await api.resumePoint(id, episode);
      if (pos > 5 && videoEl) {
        videoEl.currentTime = pos;
      }
    } catch {
      // No saved progress — start from the beginning
    }
    if (!saveTimer) {
      saveTimer = setInterval(saveProgress, 10000);
    }
  }

  async function saveProgress() {
    if (!videoEl || !progressDuration) return;
    try {
      await api.saveProgress(id, episode, videoEl.currentTime, progressDuration);
    } catch {
      // non-fatal
    }
  }

  function onTimeUpdate() {
    if (videoEl && videoEl.duration) {
      progressDuration = videoEl.duration;
    }
  }

  function onEnded() {
    saveProgress();
    if (episodes.some((e) => e.number === episode + 1)) {
      window.location.hash = `#/watch/${id}/${episode + 1}`;
    }
  }

  function prevEp() {
    if (episode > 1) window.location.hash = `#/watch/${id}/${episode - 1}`;
  }

  function nextEp() {
    if (episodes.some((e) => e.number === episode + 1)) {
      window.location.hash = `#/watch/${id}/${episode + 1}`;
    }
  }

  const hasNext = $derived(episodes.some((e) => e.number === episode + 1));
</script>

<div class="player-page">
  <div class="player-top">
    <button class="back-btn" onclick={() => router.navigate({ name: "details", id })}>
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" width="16" height="16">
        <polyline points="15 18 9 12 15 6" />
      </svg>
      <span>Back</span>
    </button>

    <div class="now-playing">
      <div class="np-title">{card?.title ?? "Loading anime…"}</div>
      <div class="np-ep">Episode {episode}{episodes.length ? ` of ${episodes.length}` : ""}</div>
    </div>

    <div class="top-actions">
      <button class="nav-ep-btn" disabled={episode <= 1} onclick={prevEp} title="Previous episode">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" width="14" height="14">
          <polyline points="15 18 9 12 15 6" />
        </svg>
        <span>Prev</span>
      </button>
      <button class="nav-ep-btn" disabled={!hasNext} onclick={nextEp} title="Next episode">
        <span>Next</span>
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" width="14" height="14">
          <polyline points="9 18 15 12 9 6" />
        </svg>
      </button>
    </div>
  </div>

  {#if loading}
    <div class="player-loading">
      <div class="spinner"></div>
    </div>
  {:else if error}
    <div class="player-error">
      <div class="error-box">
        <span>{error}</span>
        <button class="btn primary" onclick={() => router.navigate({ name: "details", id })}>
          Back to Details
        </button>
      </div>
    </div>
  {:else if isEmbed}
    <div class="frame-wrap">
      <iframe
        src={streamUrl}
        title="Anime Stream Embed"
        allow="autoplay; encrypted-media; fullscreen; picture-in-picture"
        allowfullscreen
      ></iframe>
    </div>
    <p class="embed-note">Playing via stream embed. Controls provided by source player.</p>
  {:else}
    <div class="video-wrap">
      <video
        bind:this={videoEl}
        controls
        autoplay
        onloadedmetadata={onLoadedMetadata}
        ontimeupdate={onTimeUpdate}
        onended={onEnded}
      ></video>
    </div>
  {/if}

  {#if !loading && !error && episodes.length > 0}
    <section class="ep-strip-wrap">
      <div class="strip-head">
        <h3>All Episodes</h3>
        {#if source === "local"}
          <span class="src-note">Playing from your local library</span>
        {/if}
      </div>
      <div class="strip">
        {#each episodes as ep (ep.number)}
          <button
            class="strip-item"
            class:current={ep.number === episode}
            onclick={() => {
              if (ep.number !== episode) {
                window.location.hash = `#/watch/${id}/${ep.number}`;
              }
            }}
          >
            {ep.number}
          </button>
        {/each}
      </div>
    </section>
  {/if}
</div>

<style>
  .player-page {
    height: 100vh;
    display: flex;
    flex-direction: column;
    background: #000000;
  }

  .player-top {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 28px;
    gap: 20px;
    background: #0d0d0d;
    border-bottom: 1px solid var(--border);
    height: 56px;
  }

  .back-btn {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    color: var(--text-dim);
    font-size: 13.5px;
    font-weight: 600;
    padding: 6px 12px;
    border-radius: 999px;
    background: var(--surface-2);
    transition: all 0.14s ease;
  }

  .back-btn:hover {
    color: var(--text);
    background: var(--surface-3);
  }

  .now-playing {
    flex: 1;
    min-width: 0;
    text-align: center;
  }

  .np-title {
    font-weight: 700;
    font-size: 15px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    color: var(--text);
  }

  .np-ep {
    font-size: 12px;
    color: var(--text-dim);
    margin-top: 1px;
  }

  .top-actions {
    display: flex;
    gap: 8px;
  }

  .nav-ep-btn {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 6px 14px;
    border-radius: 999px;
    background: var(--surface-2);
    color: var(--text);
    font-size: 13px;
    font-weight: 600;
    transition: all 0.13s ease;
  }

  .nav-ep-btn:hover:not(:disabled) {
    background: var(--surface-3);
    color: var(--accent);
  }

  .nav-ep-btn:disabled {
    opacity: 0.35;
    cursor: not-allowed;
  }

  .player-loading,
  .player-error {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 40px;
  }

  .frame-wrap {
    flex: 1;
    min-height: 0;
    position: relative;
    background: #000;
  }

  .frame-wrap iframe {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    border: 0;
  }

  .embed-note {
    text-align: center;
    color: var(--text-faint);
    font-size: 12px;
    padding: 8px 0 10px;
    margin: 0;
    background: #0d0d0d;
  }

  .video-wrap {
    flex: 1;
    min-height: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    background: #000;
  }

  video {
    width: 100%;
    height: 100%;
    max-height: 78vh;
    background: #000;
    outline: none;
  }

  .ep-strip-wrap {
    padding: 14px 28px 20px;
    background: #0d0d0d;
    border-top: 1px solid var(--border);
  }

  .strip-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 10px;
  }

  .strip-head h3 {
    font-size: 14px;
    font-weight: 700;
    letter-spacing: -0.01em;
  }

  .src-note {
    font-size: 12px;
    color: var(--text-faint);
  }

  .strip {
    display: flex;
    gap: 8px;
    overflow-x: auto;
    padding-bottom: 6px;
    scrollbar-width: thin;
  }

  .strip-item {
    min-width: 44px;
    height: 36px;
    padding: 0 12px;
    border-radius: var(--radius-sm);
    background: var(--surface-2);
    border: 1px solid var(--border);
    font-size: 13px;
    font-weight: 700;
    color: var(--text-dim);
    transition: all 0.12s ease;
    flex-shrink: 0;
  }

  .strip-item:hover {
    color: var(--text);
    border-color: var(--accent);
  }

  .strip-item.current {
    background: var(--accent);
    color: #0d0d0d;
    border-color: var(--accent);
  }
</style>
