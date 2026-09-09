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
      // no saved progress — start from the beginning
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
    <button class="btn ghost" onclick={() => router.navigate({ name: "details", id })}>
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" width="14" height="14"><polyline points="15 18 9 12 15 6" /></svg>
      Back
    </button>
    <div class="now-playing">
      <div class="np-title">{card?.title ?? "…"}</div>
      <div class="np-ep">Episode {episode}{episodes.length ? ` of ${episodes.length}` : ""}</div>
    </div>
    <div class="top-actions">
      <button class="btn" disabled={episode <= 1} onclick={prevEp}>‹ Prev</button>
      <button class="btn" disabled={!hasNext} onclick={nextEp}>Next ›</button>
    </div>
  </div>

  {#if loading}
    <div class="spinner"></div>
  {:else if error}
    <div class="error-box">
      <span>{error}</span>
      <button class="btn" onclick={() => router.navigate({ name: "details", id })}>Back to details</button>
    </div>
  {:else if isEmbed}
    <div class="frame-wrap">
      <iframe
        src={streamUrl}
        allow="autoplay; encrypted-media; fullscreen; picture-in-picture"
        allowfullscreen
      ></iframe>
    </div>
    <p class="embed-note">Playing via the official streaming embed. Player controls come from the provider.</p>
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
    <section class="ep-strip">
      <div class="strip-head">
        <h3>Episodes</h3>
        {#if source === "local"}<span class="src-note">from your library</span>{/if}
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
    background: #06070a;
  }

  .player-top {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 20px;
    gap: 16px;
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
  }

  .np-ep {
    font-size: 12px;
    color: var(--text-dim);
  }

  .top-actions {
    display: flex;
    gap: 8px;
  }

  .frame-wrap {
    flex: 1;
    min-height: 0;
    position: relative;
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
    padding: 8px 0 14px;
    margin: 0;
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
    max-height: 76vh;
    background: #000;
    outline: none;
  }

  .ep-strip {
    padding: 4px 20px 24px;
  }

  .strip-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 10px;
  }

  .strip-head h3 {
    font-size: 15px;
  }

  .strip {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
  }

  .strip-item {
    min-width: 44px;
    height: 34px;
    padding: 0 10px;
    border-radius: 6px;
    background: var(--surface);
    border: 1px solid var(--border-soft);
    font-size: 13px;
    font-weight: 600;
    color: var(--text-dim);
  }

  .strip-item:hover {
    color: var(--text);
    border-color: var(--accent);
  }

  .strip-item.current {
    background: var(--accent);
    color: #14100c;
    border-color: var(--accent);
  }

  .src-note {
    font-size: 12px;
    color: var(--text-faint);
  }
</style>
