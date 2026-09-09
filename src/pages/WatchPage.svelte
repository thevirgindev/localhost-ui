<script lang="ts">
  import { onDestroy } from "svelte";
  import type HlsType from "hls.js";
  import { api } from "../lib/api";
  import type { AnimeCard, EpisodeEntry } from "../lib/types";
  import { router } from "../lib/router";

  let { id, episode }: { id: number; episode: number } = $props();

  let card = $state<AnimeCard | null>(null);
  let episodes = $state<EpisodeEntry[]>([]);
  let streamUrl = $state("");
  let streamKind = $state<"hls" | "file" | "embed">("file");
  let serverName = $state("");
  let triedUrls = $state<string[]>([]);
  let switching = $state(false);
  let loading = $state(true);
  let error = $state("");

  // Video playback state
  let videoEl: HTMLVideoElement | undefined = $state();
  let playerContainer: HTMLDivElement | undefined = $state();
  let hls: HlsType | undefined;
  let saveTimer: ReturnType<typeof setInterval> | undefined;
  let hideControlsTimer: ReturnType<typeof setTimeout> | undefined;

  let isPlaying = $state(false);
  let isMuted = $state(false);
  let volume = $state(1);
  let currentTime = $state(0);
  let duration = $state(0);
  let bufferedPercent = $state(0);
  let isFullscreen = $state(false);
  let showControls = $state(true);
  let playbackSpeed = $state(1);
  let showSpeedMenu = $state(false);
  let showAudioSubMenu = $state(false);
  let showEpisodeDrawer = $state(false);
  let selectedAudio = $state<"ja" | "en">("ja");
  let selectedSubtitle = $state<"en" | "off">("en");
  let showUpNextPrompt = $state(false);
  let upNextCountdown = $state(10);
  let countdownTimer: ReturnType<typeof setInterval> | undefined;
  let wiredFor = $state("");

  function formatTime(seconds: number): string {
    if (isNaN(seconds) || seconds < 0) return "0:00";
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins}:${secs < 10 ? "0" : ""}${secs}`;
  }

  function clearPlayback() {
    if (saveTimer) {
      clearInterval(saveTimer);
      saveTimer = undefined;
    }
    if (countdownTimer) {
      clearInterval(countdownTimer);
      countdownTimer = undefined;
    }
    if (hideControlsTimer) {
      clearTimeout(hideControlsTimer);
      hideControlsTimer = undefined;
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
    currentTime = 0;
    duration = 0;
    showUpNextPrompt = false;
    wiredFor = "";
  }

  onDestroy(clearPlayback);

  // Load show + episode data whenever the route changes
  $effect(() => {
    const animeId = id;
    const ep = episode;
    loadEpisode(animeId, ep);
  });

  function applyPlayable(p: { url: string; kind: string; server: string }) {
    if (p.url.startsWith("/") || /^[A-Za-z]:[\\/]/.test(p.url)) {
      // Local file — needs the asset-protocol conversion.
      import("@tauri-apps/api/core").then(({ convertFileSrc }) => {
        streamUrl = convertFileSrc(p.url);
        streamKind = "file";
        serverName = p.server;
      });
      return;
    }
    streamUrl = p.url;
    streamKind = p.kind === "hls" || p.kind === "embed" ? p.kind : "file";
    serverName = p.server;
  }

  async function loadEpisode(animeId: number, epNumber: number) {
    loading = true;
    error = "";
    streamUrl = "";
    triedUrls = [];
    switching = false;
    clearPlayback();
    try {
      if (!card) {
        card = await api.details(animeId);
      }
      if (episodes.length === 0) {
        const src = await api.episodes(animeId, card.title, card.titleEnglish);
        episodes = src.episodes;
      }

      const entry = episodes.find((e) => e.number === epNumber);
      if (!entry) {
        error = `Episode ${epNumber} is not available for this title.`;
        loading = false;
        return;
      }

      // Ask the backend for a live, probed server. Its chain:
      // local library → ani.zip direct HLS → embeds → user mirrors.
      const playable = await api.resolveEpisode(animeId, card.title, card.titleEnglish, epNumber, triedUrls);
      triedUrls = [...triedUrls, playable.url];
      applyPlayable(playable);
    } catch (e) {
      error = String(e).replace(/^Error:\s*/, "");
    } finally {
      loading = false;
    }
  }

  // Current server died (probe miss, HLS fatal, decode error) — walk the chain.
  async function switchServer() {
    if (!card || switching) return;
    switching = true;
    clearPlayback();
    streamUrl = "";
    try {
      const playable = await api.resolveEpisode(id, card.title, card.titleEnglish, episode, triedUrls);
      triedUrls = [...triedUrls, playable.url];
      applyPlayable(playable);
    } catch (e) {
      error = `All servers exhausted. ${String(e).replace(/^Error:\s*/, "")}`;
    } finally {
      switching = false;
    }
  }

  function onVideoError() {
    if (streamUrl && !switching) switchServer();
  }

  // Attach stream once video element is ready
  $effect(() => {
    const el = videoEl;
    const url = streamUrl;
    if (!el || !url || streamKind === "embed" || wiredFor === url) return;
    wiredFor = url;

    if (streamKind === "hls" || url.includes(".m3u8")) {
      import("hls.js").then((mod) => {
        const Hls = mod.default;
        if (Hls.isSupported()) {
          hls = new Hls({ enableWorker: true });
          hls.loadSource(url);
          hls.attachMedia(el);
          hls.on(Hls.Events.ERROR, (_evt, data) => {
            if (data.fatal) switchServer();
          });
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
    duration = videoEl.duration || 0;
    try {
      const pos = await api.resumePoint(id, episode);
      if (pos > 5 && videoEl) {
        videoEl.currentTime = pos;
      }
    } catch {
      // Start from beginning
    }
    if (!saveTimer) {
      saveTimer = setInterval(saveProgress, 10000);
    }
    triggerControlsActivity();
  }

  async function saveProgress() {
    if (!videoEl || !duration) return;
    try {
      await api.saveProgress(id, episode, videoEl.currentTime, duration);
    } catch {
      // non-fatal
    }
  }

  function onTimeUpdate() {
    if (!videoEl) return;
    currentTime = videoEl.currentTime;
    if (videoEl.duration) {
      duration = videoEl.duration;
    }

    if (videoEl.buffered.length > 0 && duration > 0) {
      const end = videoEl.buffered.end(videoEl.buffered.length - 1);
      bufferedPercent = Math.min(100, (end / duration) * 100);
    }

    // Up next prompt within 20s of ending
    if (duration > 30 && duration - currentTime <= 20 && !showUpNextPrompt && hasNext) {
      showUpNextPrompt = true;
      upNextCountdown = 15;
      countdownTimer = setInterval(() => {
        upNextCountdown -= 1;
        if (upNextCountdown <= 0) {
          nextEp();
        }
      }, 1000);
    }
  }

  function togglePlay() {
    if (!videoEl) return;
    if (videoEl.paused) {
      videoEl.play();
      isPlaying = true;
    } else {
      videoEl.pause();
      isPlaying = false;
    }
    triggerControlsActivity();
  }

  function seek(e: MouseEvent) {
    if (!videoEl || !duration) return;
    const bar = e.currentTarget as HTMLElement;
    const rect = bar.getBoundingClientRect();
    const pos = Math.max(0, Math.min(1, (e.clientX - rect.left) / rect.width));
    videoEl.currentTime = pos * duration;
    currentTime = videoEl.currentTime;
  }

  function skipIntro() {
    if (!videoEl) return;
    videoEl.currentTime = Math.min(duration, videoEl.currentTime + 85);
    triggerControlsActivity();
  }

  function skipTime(delta: number) {
    if (!videoEl) return;
    videoEl.currentTime = Math.max(0, Math.min(duration, videoEl.currentTime + delta));
    triggerControlsActivity();
  }

  function setVolume(v: number) {
    volume = v;
    if (videoEl) {
      videoEl.volume = v;
      videoEl.muted = v === 0;
    }
    isMuted = v === 0;
  }

  function toggleMute() {
    if (!videoEl) return;
    isMuted = !isMuted;
    videoEl.muted = isMuted;
  }

  function setSpeed(speed: number) {
    playbackSpeed = speed;
    if (videoEl) {
      videoEl.playbackRate = speed;
    }
    showSpeedMenu = false;
  }

  function toggleFullscreen() {
    if (!playerContainer) return;
    if (!document.fullscreenElement) {
      playerContainer.requestFullscreen().catch(() => {});
      isFullscreen = true;
    } else {
      document.exitFullscreen().catch(() => {});
      isFullscreen = false;
    }
  }

  function triggerControlsActivity() {
    showControls = true;
    if (hideControlsTimer) clearTimeout(hideControlsTimer);
    hideControlsTimer = setTimeout(() => {
      if (isPlaying && !showSpeedMenu && !showAudioSubMenu && !showEpisodeDrawer) {
        showControls = false;
      }
    }, 3500);
  }

  function onEnded() {
    saveProgress();
    if (hasNext) {
      nextEp();
    }
  }

  function prevEp() {
    if (episode > 1) {
      window.location.hash = `#/watch/${id}/${episode - 1}`;
    }
  }

  function nextEp() {
    if (episodes.some((e) => e.number === episode + 1)) {
      window.location.hash = `#/watch/${id}/${episode + 1}`;
    }
  }

  const hasNext = $derived(episodes.some((e) => e.number === episode + 1));
  const currentEpEntry = $derived(episodes.find((e) => e.number === episode));

  // Season Navigation Logic
  const totalAnimeEpisodes = $derived((card as AnimeCard | null)?.episodes || episodes.length || 24);
  const seasonCount = $derived(totalAnimeEpisodes > 12 ? Math.ceil(totalAnimeEpisodes / 12) : (episodes.length > 12 ? 2 : 1));
  const currentSeason = $derived(Math.ceil(episode / 12));
  const hasNextSeason = $derived(currentSeason < seasonCount || episodes.some((e) => e.number > currentSeason * 12));

  function goToNextSeason() {
    const nextSeasonFirstEp = currentSeason * 12 + 1;
    const targetEp = episodes.find((e) => e.number >= nextSeasonFirstEp) || episodes[episodes.length - 1];
    if (targetEp) {
      window.location.hash = `#/watch/${id}/${targetEp.number}`;
    }
  }
</script>

<div
  class="watch-page-container"
  bind:this={playerContainer}
  role="application"
  aria-label="Video player"
  onmousemove={triggerControlsActivity}
  onmouseleave={() => {
    if (isPlaying) showControls = false;
  }}
>
  <!-- Top Navigation Header -->
  <header class="watch-header" class:hidden={!showControls && isPlaying}>
    <button
      class="icon-back-btn"
      onclick={() => router.navigate({ name: "details", id })}
      aria-label="Back to details"
    >
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.4" width="18" height="18">
        <polyline points="15 18 9 12 15 6" />
      </svg>
      <span>Back</span>
    </button>

    <div class="header-meta">
      <h1 class="anime-heading">{card?.title ?? "Loading anime…"}</h1>
      <div class="ep-subtitle">
        <span class="ep-pill">E{episode}</span>
        <span class="ep-name">{currentEpEntry?.title || `Episode ${episode}`}</span>
        <span class="sub-badge">Sub | Dub</span>
        {#if seasonCount > 1}
          <span class="season-badge-header">Season {currentSeason}</span>
        {/if}
      </div>
    </div>

    <div class="header-right-actions">
      {#if hasNextSeason}
        <button
          class="next-season-nav-btn"
          onclick={goToNextSeason}
          title={`Move to Season ${currentSeason + 1}`}
        >
          <svg viewBox="0 0 24 24" fill="currentColor" width="14" height="14">
            <polygon points="5 4 15 12 5 20 5 4" />
            <line x1="19" y1="5" x2="19" y2="19" stroke="currentColor" stroke-width="2.5" />
          </svg>
          <span>Next Season (S{currentSeason + 1})</span>
        </button>
      {/if}

      <button
        class="ep-drawer-toggle-btn"
        class:active={showEpisodeDrawer}
        onclick={() => (showEpisodeDrawer = !showEpisodeDrawer)}
        title="Episode List"
      >
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" width="16" height="16">
          <line x1="8" y1="6" x2="21" y2="6" />
          <line x1="8" y1="12" x2="21" y2="12" />
          <line x1="8" y1="18" x2="21" y2="18" />
          <line x1="3" y1="6" x2="3.01" y2="6" />
          <line x1="3" y1="12" x2="3.01" y2="12" />
          <line x1="3" y1="18" x2="3.01" y2="18" />
        </svg>
        <span>Episodes ({episodes.length})</span>
      </button>

      {#if serverName && !loading && !error}
        <button
          class="server-pill"
          onclick={switchServer}
          disabled={switching}
          title="Switch to the next available server"
        >
          <span class="server-dot"></span>
          <span>{switching ? "Switching…" : serverName}</span>
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="13" height="13" stroke-linecap="round" stroke-linejoin="round">
            <path d="M21 12a9 9 0 1 1-2.64-6.36" />
            <polyline points="21 3 21 9 15 9" />
          </svg>
        </button>
      {/if}
    </div>
  </header>

  <!-- Video Stage -->
  <main class="player-stage">
    {#if loading}
      <div class="player-loader">
        <div class="cr-spinner"></div>
        <p>Loading episode stream…</p>
      </div>
    {:else if error}
      <div class="player-error-card">
        <div class="error-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="42" height="42" stroke-linecap="round" stroke-linejoin="round"><path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z" /><path d="M12 9v4" /><path d="M12 17h.01" /></svg></div>
        <h3>Playback Unavailable</h3>
        <p>{error}</p>
        <button class="cr-btn" onclick={() => router.navigate({ name: "details", id })}>
          Return to Anime Overview
        </button>
      </div>
    {:else if streamKind === "embed"}
      <div class="embed-stage">
        <iframe
          src={streamUrl}
          title="Anime Stream"
          allow="autoplay; encrypted-media; fullscreen"
          allowfullscreen
        ></iframe>
      </div>
    {:else}
      <div
        class="video-canvas-wrapper"
        onclick={togglePlay}
        role="button"
        tabindex="0"
        onkeydown={(e) => (e.key === " " || e.key === "k") && togglePlay()}
      >
        <video
          bind:this={videoEl}
          autoplay
          onplay={() => (isPlaying = true)}
          onpause={() => (isPlaying = false)}
          onloadedmetadata={onLoadedMetadata}
          ontimeupdate={onTimeUpdate}
          onended={onEnded}
          onerror={onVideoError}
        ></video>

        <!-- Skip Intro Floating Button -->
        {#if currentTime < 120 && duration > 180}
          <button
            class="skip-intro-button"
            onclick={(e) => {
              e.stopPropagation();
              skipIntro();
            }}
          >
            <span>Skip Intro</span>
            <svg viewBox="0 0 24 24" fill="currentColor" width="14" height="14">
              <polygon points="5 4 15 12 5 20 5 4" />
              <line x1="19" y1="5" x2="19" y2="19" stroke="currentColor" stroke-width="2.5" />
            </svg>
          </button>
        {/if}

        <!-- Up Next Overlay Card -->
        {#if showUpNextPrompt && hasNext}
          <div
            class="up-next-overlay"
            onclick={(e) => e.stopPropagation()}
            role="dialog"
            aria-label="Next episode prompt"
            tabindex="-1"
            onkeydown={(e) => e.key === "Escape" && (showUpNextPrompt = false)}
          >
            <div class="up-next-content">
              <span class="up-next-tag">Up Next in {upNextCountdown}s</span>
              <h4>Episode {episode + 1}</h4>
              <div class="up-next-btns">
                <button class="up-next-play-btn" onclick={nextEp}>
                  Play Now
                </button>
                <button
                  class="up-next-dismiss-btn"
                  onclick={() => {
                    showUpNextPrompt = false;
                    if (countdownTimer) clearInterval(countdownTimer);
                  }}
                >
                  Dismiss
                </button>
              </div>
            </div>
          </div>
        {/if}
      </div>

      <!-- Crunchyroll Custom Controls Bar -->
      <div class="player-controls-overlay" class:hidden={!showControls && isPlaying}>
        <!-- Scrubber Bar -->
        <div
          class="timeline-track-wrap"
          onclick={seek}
          role="slider"
          tabindex="0"
          aria-valuemin="0"
          aria-valuemax={duration}
          aria-valuenow={currentTime}
          aria-label="Seek timeline"
          onkeydown={(e) => {
            if (!videoEl) return;
            if (e.key === "ArrowRight") videoEl.currentTime = Math.min(duration, videoEl.currentTime + 10);
            if (e.key === "ArrowLeft") videoEl.currentTime = Math.max(0, videoEl.currentTime - 10);
          }}
        >
          <div class="timeline-buffered" style="width: {bufferedPercent}%"></div>
          <div
            class="timeline-played"
            style="width: {duration ? (currentTime / duration) * 100 : 0}%"
          >
            <div class="timeline-scrubber-handle"></div>
          </div>
        </div>

        <!-- Controls Row -->
        <div class="controls-bottom-bar">
          <div class="controls-left">
            <button class="ctrl-btn play-btn" onclick={togglePlay} aria-label={isPlaying ? "Pause" : "Play"}>
              {#if isPlaying}
                <svg viewBox="0 0 24 24" fill="currentColor" width="20" height="20">
                  <rect x="6" y="4" width="4" height="16" rx="1" />
                  <rect x="14" y="4" width="4" height="16" rx="1" />
                </svg>
              {:else}
                <svg viewBox="0 0 24 24" fill="currentColor" width="20" height="20">
                  <polygon points="5 3 19 12 5 21 5 3" />
                </svg>
              {/if}
            </button>

            <button class="ctrl-btn" onclick={() => skipTime(-10)} title="Rewind 10s">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" width="18" height="18">
                <path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" />
                <path d="M3 3v5h5" />
              </svg>
              <span class="ctrl-subtext">10</span>
            </button>

            <button class="ctrl-btn" onclick={() => skipTime(10)} title="Forward 10s">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" width="18" height="18">
                <path d="M21 12a9 9 0 1 1-9-9 9.75 9.75 0 0 1 6.74 2.74L21 8" />
                <path d="M21 3v5h-5" />
              </svg>
              <span class="ctrl-subtext">10</span>
            </button>

            <button
              class="ctrl-btn"
              disabled={episode <= 1}
              onclick={prevEp}
              title={episode > 1 ? `Previous: Episode ${episode - 1}` : "First episode"}
            >
              <svg viewBox="0 0 24 24" fill="currentColor" width="18" height="18">
                <polygon points="19 20 9 12 19 4 19 20" />
                <line x1="5" y1="19" x2="5" y2="5" stroke="currentColor" stroke-width="2.5" />
              </svg>
            </button>

            <button
              class="ctrl-btn"
              disabled={!hasNext}
              onclick={nextEp}
              title={hasNext ? `Next: Episode ${episode + 1}` : "No more episodes"}
            >
              <svg viewBox="0 0 24 24" fill="currentColor" width="18" height="18">
                <polygon points="5 4 15 12 5 20 5 4" />
                <line x1="19" y1="5" x2="19" y2="19" stroke="currentColor" stroke-width="2.5" />
              </svg>
            </button>

            <div class="volume-container">
              <button class="ctrl-btn" onclick={toggleMute} aria-label={isMuted ? "Unmute" : "Mute"}>
                {#if isMuted || volume === 0}
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" width="18" height="18">
                    <line x1="1" y1="1" x2="23" y2="23" />
                    <path d="M9 9v3a3 3 0 0 0 5.12 2.12M15 9.34V4a3 3 0 0 0-5.94-.6" />
                    <path d="M17 16.95A7 7 0 0 1 5 12v-2m14 0v2a7 7 0 0 1-.11 1.23" />
                  </svg>
                {:else}
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" width="18" height="18">
                    <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5" />
                    <path d="M19.07 4.93a10 10 0 0 1 0 14.14M15.54 8.46a5 5 0 0 1 0 7.07" />
                  </svg>
                {/if}
              </button>
              <input
                type="range"
                min="0"
                max="1"
                step="0.05"
                value={volume}
                class="vol-slider"
                oninput={(e) => setVolume(parseFloat((e.currentTarget as HTMLInputElement).value))}
              />
            </div>

            <div class="time-readout">
              <span class="current-time">{formatTime(currentTime)}</span>
              <span class="time-sep">/</span>
              <span class="duration-time">{formatTime(duration)}</span>
            </div>
          </div>

          <div class="controls-right">
            <!-- Audio & Subtitles Menu -->
            <div class="relative-menu-wrap">
              <button
                class="ctrl-btn label-btn"
                onclick={() => {
                  showAudioSubMenu = !showAudioSubMenu;
                  showSpeedMenu = false;
                }}
              >
                <span>Audio/Sub</span>
              </button>

              {#if showAudioSubMenu}
                <div class="popup-menu">
                  <div class="menu-section-title">Audio</div>
                  <button
                    class="menu-item"
                    class:active={selectedAudio === "ja"}
                    onclick={() => {
                      selectedAudio = "ja";
                      showAudioSubMenu = false;
                    }}
                  >
                    Japanese (Original)
                  </button>
                  <button
                    class="menu-item"
                    class:active={selectedAudio === "en"}
                    onclick={() => {
                      selectedAudio = "en";
                      showAudioSubMenu = false;
                    }}
                  >
                    English (Dub)
                  </button>

                  <div class="menu-section-title">Subtitles</div>
                  <button
                    class="menu-item"
                    class:active={selectedSubtitle === "en"}
                    onclick={() => {
                      selectedSubtitle = "en";
                      showAudioSubMenu = false;
                    }}
                  >
                    English (Full)
                  </button>
                  <button
                    class="menu-item"
                    class:active={selectedSubtitle === "off"}
                    onclick={() => {
                      selectedSubtitle = "off";
                      showAudioSubMenu = false;
                    }}
                  >
                    Off
                  </button>
                </div>
              {/if}
            </div>

            <!-- Playback Speed Menu -->
            <div class="relative-menu-wrap">
              <button
                class="ctrl-btn label-btn"
                onclick={() => {
                  showSpeedMenu = !showSpeedMenu;
                  showAudioSubMenu = false;
                }}
              >
                <span>{playbackSpeed}x</span>
              </button>

              {#if showSpeedMenu}
                <div class="popup-menu">
                  <div class="menu-section-title">Speed</div>
                  {#each [0.5, 0.75, 1, 1.25, 1.5, 2] as s}
                    <button
                      class="menu-item"
                      class:active={playbackSpeed === s}
                      onclick={() => setSpeed(s)}
                    >
                      {s}x
                    </button>
                  {/each}
                </div>
              {/if}
            </div>

            <!-- Fullscreen Button -->
            <button class="ctrl-btn" onclick={toggleFullscreen} aria-label="Toggle fullscreen">
              {#if isFullscreen}
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" width="18" height="18">
                  <path d="M8 3v3a2 2 0 0 1-2 2H3m18 0h-3a2 2 0 0 1-2-2V3m0 18v-3a2 2 0 0 1 2-2h3M3 16h3a2 2 0 0 1 2 2v3" />
                </svg>
              {:else}
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" width="18" height="18">
                  <path d="M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3" />
                </svg>
              {/if}
            </button>
          </div>
        </div>
      </div>
    {/if}
  </main>

  <!-- Side Episode Selector Drawer (Authentic Crunchyroll Episode Selector) -->
  {#if showEpisodeDrawer}
    <aside class="episode-drawer">
      <div class="drawer-header">
        <div class="drawer-title-group">
          <h3>Select Episode</h3>
          <span class="drawer-count">{episodes.length} Episodes</span>
        </div>
        <button
          class="drawer-close-btn"
          onclick={() => (showEpisodeDrawer = false)}
          aria-label="Close episode selector"
          title="Close (ESC)"
        >
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" width="16" height="16">
            <line x1="18" y1="6" x2="6" y2="18" />
            <line x1="6" y1="6" x2="18" y2="18" />
          </svg>
        </button>
      </div>

      {#if seasonCount > 1}
        <div class="drawer-season-tabs">
          {#each Array.from({ length: seasonCount }) as _, idx}
            {@const sNum = idx + 1}
            <button
              class="drawer-season-pill"
              class:active={currentSeason === sNum}
              onclick={() => {
                const firstEpOfSeason = (sNum - 1) * 12 + 1;
                const target = episodes.find((e) => e.number === firstEpOfSeason) || episodes[0];
                window.location.hash = `#/watch/${id}/${target.number}`;
              }}
            >
              Season {sNum}
            </button>
          {/each}
        </div>
      {/if}

      <div class="drawer-episodes-list">
        {#each episodes as ep (ep.number)}
          <div
            class="drawer-ep-card"
            class:active={ep.number === episode}
            onclick={() => {
              if (ep.number !== episode) {
                window.location.hash = `#/watch/${id}/${ep.number}`;
              }
            }}
            role="button"
            tabindex="0"
            onkeydown={(e) => e.key === "Enter" && (window.location.hash = `#/watch/${id}/${ep.number}`)}
          >
            <div class="drawer-ep-thumb">
              {#if card?.banner || card?.cover}
                <img src={card?.banner || card?.cover} alt={ep.title} loading="lazy" />
              {:else}
                <div class="drawer-noimg">E{ep.number}</div>
              {/if}
              {#if ep.number === episode}
                <div class="now-playing-badge">
                  <span>NOW PLAYING</span>
                </div>
              {/if}
              <div class="drawer-duration">24m</div>
            </div>

            <div class="drawer-ep-info">
              <div class="drawer-ep-tags">
                <span class="ep-num-pill">E{ep.number}</span>
                <span class="ep-sub-tag">Sub | Dub</span>
              </div>
              <h4 class="drawer-ep-title">{ep.title}</h4>
            </div>
          </div>
        {/each}
      </div>
    </aside>
  {/if}
</div>

<style>
  .watch-page-container {
    position: relative;
    width: 100vw;
    height: 100vh;
    background: #000000;
    color: #ffffff;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    font-family: var(--font);
    user-select: none;
  }

  /* Header Bar */
  .watch-header {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 64px;
    background: linear-gradient(to bottom, rgba(0, 0, 0, 0.85) 0%, transparent 100%);
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 28px;
    z-index: 20;
    transition: opacity 0.25s ease, transform 0.25s ease;
  }

  .watch-header.hidden {
    opacity: 0;
    pointer-events: none;
    transform: translateY(-10px);
  }

  .icon-back-btn {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 8px 16px;
    background: rgba(20, 22, 28, 0.7);
    border: 1px solid rgba(255, 255, 255, 0.15);
    border-radius: 999px;
    color: #ffffff;
    font-size: 13.5px;
    font-weight: 600;
    cursor: pointer;
    backdrop-filter: blur(8px);
    transition: all 0.15s ease;
  }

  .icon-back-btn:hover {
    background: rgba(255, 255, 255, 0.2);
    border-color: rgba(255, 255, 255, 0.3);
  }

  .header-meta {
    text-align: center;
    flex: 1;
    min-width: 0;
    padding: 0 20px;
  }

  .anime-heading {
    font-size: 16px;
    font-weight: 800;
    letter-spacing: -0.01em;
    margin: 0;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .ep-subtitle {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    font-size: 12.5px;
    color: #a0a6b5;
    margin-top: 2px;
  }

  .ep-pill {
    font-weight: 800;
    color: var(--accent);
  }

  .sub-badge {
    background: rgba(255, 255, 255, 0.12);
    padding: 2px 6px;
    border-radius: 4px;
    font-size: 10px;
    font-weight: 700;
    text-transform: uppercase;
  }

  .season-badge-header {
    background: var(--accent-dim);
    color: var(--accent);
    border: 1px solid var(--accent);
    padding: 1px 6px;
    border-radius: 4px;
    font-size: 10px;
    font-weight: 800;
    text-transform: uppercase;
  }

  .header-right-actions {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .next-season-nav-btn {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 8px 14px;
    background: var(--accent);
    color: var(--accent-contrast, #000000);
    border: 1px solid var(--accent);
    border-radius: var(--radius-sm, 4px);
    font-size: 12.5px;
    font-weight: 800;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .next-season-nav-btn:hover {
    background: var(--accent-hover);
    transform: translateY(-1px);
  }

  .ep-drawer-toggle-btn {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 8px 16px;
    background: rgba(20, 22, 28, 0.7);
    border: 1px solid rgba(255, 255, 255, 0.15);
    border-radius: var(--radius-sm, 4px);
    color: #ffffff;
    font-size: 13px;
    font-weight: 600;
    cursor: pointer;
    backdrop-filter: blur(8px);
    transition: all 0.15s ease;
  }

  .ep-drawer-toggle-btn:hover,
  .ep-drawer-toggle-btn.active {
    background: var(--accent);
    color: #000000;
    border-color: var(--accent);
  }

  /* Player Stage */
  .player-stage {
    flex: 1;
    position: relative;
    width: 100%;
    height: 100%;
    background: #000000;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .video-canvas-wrapper {
    position: relative;
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    background: #000000;
    cursor: pointer;
  }

  video {
    width: 100%;
    height: 100%;
    max-height: 100vh;
    object-fit: contain;
  }

  .skip-intro-button {
    position: absolute;
    bottom: 90px;
    right: 36px;
    z-index: 25;
    display: inline-flex;
    align-items: center;
    gap: 8px;
    background: rgba(15, 17, 22, 0.85);
    color: #ffffff;
    border: 1px solid rgba(255, 255, 255, 0.25);
    padding: 10px 20px;
    border-radius: 8px;
    font-size: 14px;
    font-weight: 700;
    cursor: pointer;
    backdrop-filter: blur(10px);
    transition: all 0.15s ease;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.5);
  }

  .skip-intro-button:hover {
    background: var(--accent);
    color: #000000;
    border-color: var(--accent);
    transform: scale(1.04);
  }

  /* Up Next Overlay */
  .up-next-overlay {
    position: absolute;
    bottom: 90px;
    left: 36px;
    z-index: 25;
    background: rgba(15, 17, 22, 0.92);
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 10px;
    padding: 18px 22px;
    backdrop-filter: blur(12px);
    box-shadow: 0 12px 36px rgba(0, 0, 0, 0.7);
    animation: fadeIn 0.2s ease;
  }

  .up-next-tag {
    font-size: 11px;
    font-weight: 800;
    text-transform: uppercase;
    color: var(--accent);
    letter-spacing: 0.05em;
  }

  .up-next-content h4 {
    margin: 4px 0 12px;
    font-size: 15px;
    font-weight: 700;
  }

  .up-next-btns {
    display: flex;
    gap: 10px;
  }

  .up-next-play-btn {
    padding: 6px 14px;
    border-radius: 6px;
    background: var(--accent);
    color: #000000;
    font-size: 12.5px;
    font-weight: 700;
    border: none;
    cursor: pointer;
  }

  .up-next-dismiss-btn {
    padding: 6px 12px;
    border-radius: 6px;
    background: rgba(255, 255, 255, 0.12);
    color: #ffffff;
    font-size: 12.5px;
    font-weight: 600;
    border: none;
    cursor: pointer;
  }

  /* Controls Overlay */
  .player-controls-overlay {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    background: linear-gradient(to top, rgba(0, 0, 0, 0.9) 0%, rgba(0, 0, 0, 0.4) 60%, transparent 100%);
    padding: 24px 28px 20px;
    z-index: 20;
    transition: opacity 0.25s ease, transform 0.25s ease;
  }

  .player-controls-overlay.hidden {
    opacity: 0;
    pointer-events: none;
    transform: translateY(10px);
  }

  /* Timeline Scrubber */
  .timeline-track-wrap {
    position: relative;
    width: 100%;
    height: 6px;
    background: rgba(255, 255, 255, 0.2);
    border-radius: 3px;
    cursor: pointer;
    margin-bottom: 14px;
    transition: height 0.15s ease;
  }

  .timeline-track-wrap:hover {
    height: 9px;
  }

  .timeline-buffered {
    position: absolute;
    left: 0;
    top: 0;
    bottom: 0;
    background: rgba(255, 255, 255, 0.35);
    border-radius: 3px;
    pointer-events: none;
  }

  .timeline-played {
    position: absolute;
    left: 0;
    top: 0;
    bottom: 0;
    background: var(--accent);
    border-radius: 3px;
    pointer-events: none;
  }

  .timeline-scrubber-handle {
    position: absolute;
    right: -6px;
    top: 50%;
    transform: translateY(-50%) scale(0);
    width: 14px;
    height: 14px;
    border-radius: 50%;
    background: #ffffff;
    box-shadow: 0 0 6px rgba(0, 0, 0, 0.8);
    transition: transform 0.15s ease;
  }

  .timeline-track-wrap:hover .timeline-scrubber-handle {
    transform: translateY(-50%) scale(1);
  }

  /* Controls Row */
  .controls-bottom-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .controls-left,
  .controls-right {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .ctrl-btn {
    position: relative;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: none;
    color: #ffffff;
    cursor: pointer;
    padding: 6px;
    border-radius: 6px;
    transition: all 0.14s ease;
  }

  .ctrl-btn:hover:not(:disabled) {
    color: var(--accent);
    transform: scale(1.08);
  }

  .ctrl-btn:disabled {
    opacity: 0.3;
    cursor: not-allowed;
  }

  .ctrl-subtext {
    position: absolute;
    font-size: 8px;
    font-weight: 800;
    top: 52%;
    left: 50%;
    transform: translate(-50%, -50%);
  }

  .label-btn {
    font-size: 13px;
    font-weight: 700;
    padding: 6px 10px;
    background: rgba(255, 255, 255, 0.12);
    border-radius: 6px;
  }

  .volume-container {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .vol-slider {
    width: 70px;
    height: 4px;
    accent-color: var(--accent);
    cursor: pointer;
  }

  .time-readout {
    font-size: 13px;
    font-weight: 600;
    color: #a0a6b5;
    margin-left: 8px;
    font-variant-numeric: tabular-nums;
  }

  .current-time {
    color: #ffffff;
  }

  .time-sep {
    margin: 0 3px;
  }

  /* Popups */
  .relative-menu-wrap {
    position: relative;
  }

  .popup-menu {
    position: absolute;
    bottom: 44px;
    right: 0;
    background: #151820;
    border: 1px solid rgba(255, 255, 255, 0.18);
    border-radius: 8px;
    padding: 8px;
    min-width: 160px;
    box-shadow: 0 12px 32px rgba(0, 0, 0, 0.8);
    z-index: 30;
  }

  .menu-section-title {
    font-size: 10px;
    font-weight: 800;
    text-transform: uppercase;
    color: #717789;
    padding: 6px 10px 2px;
    letter-spacing: 0.06em;
  }

  .menu-item {
    display: block;
    width: 100%;
    text-align: left;
    background: transparent;
    border: none;
    color: #ffffff;
    font-size: 13px;
    font-weight: 600;
    padding: 8px 10px;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.12s ease;
  }

  .menu-item:hover {
    background: rgba(255, 255, 255, 0.1);
  }

  .menu-item.active {
    background: var(--accent);
    color: #000000;
    font-weight: 700;
  }

  /* Episode Drawer */
  .episode-drawer {
    position: absolute;
    top: 0;
    right: 0;
    bottom: 0;
    width: 380px;
    max-width: 90vw;
    background: #111318;
    border-left: 1px solid rgba(255, 255, 255, 0.12);
    z-index: 40;
    display: flex;
    flex-direction: column;
    box-shadow: -10px 0 40px rgba(0, 0, 0, 0.8);
    animation: slideLeft 0.22s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .drawer-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 20px 24px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  }

  .drawer-title-group h3 {
    margin: 0;
    font-size: 16px;
    font-weight: 800;
  }

  .drawer-count {
    font-size: 12px;
    color: #717789;
  }

  .drawer-close-btn {
    width: 32px;
    height: 32px;
    aspect-ratio: 1 / 1;
    border-radius: 50%;
    background: transparent;
    border: none;
    color: #a0a6b5;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0;
    cursor: pointer;
    transition: all 0.12s ease;
  }

  .drawer-close-btn:hover {
    color: #ffffff;
    background: rgba(255, 255, 255, 0.15);
  }

  .drawer-season-tabs {
    display: flex;
    gap: 8px;
    padding: 12px 20px 0;
    border-bottom: 1px solid rgba(255, 255, 255, 0.06);
    overflow-x: auto;
  }

  .drawer-season-pill {
    padding: 6px 12px;
    background: rgba(255, 255, 255, 0.06);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: var(--radius-sm, 4px);
    color: #a0a6b5;
    font-size: 12px;
    font-weight: 700;
    cursor: pointer;
    white-space: nowrap;
    transition: all 0.12s ease;
  }

  .drawer-season-pill:hover {
    background: rgba(255, 255, 255, 0.12);
    color: #ffffff;
  }

  .drawer-season-pill.active {
    background: var(--accent);
    color: #000000;
    border-color: var(--accent);
    font-weight: 800;
  }

  .drawer-episodes-list {
    flex: 1;
    overflow-y: auto;
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .drawer-ep-card {
    display: flex;
    gap: 12px;
    padding: 10px;
    border-radius: 8px;
    background: #181b22;
    border: 1px solid rgba(255, 255, 255, 0.06);
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .drawer-ep-card:hover {
    background: #222630;
    border-color: rgba(255, 255, 255, 0.2);
    transform: translateX(-2px);
  }

  .drawer-ep-card.active {
    background: #252a36;
    border-color: var(--accent);
  }

  .drawer-ep-thumb {
    position: relative;
    width: 100px;
    aspect-ratio: 16 / 9;
    border-radius: 4px;
    overflow: hidden;
    background: #090a0d;
    flex-shrink: 0;
  }

  .drawer-ep-thumb img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .now-playing-badge {
    position: absolute;
    inset: 0;
    background: var(--accent);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 9px;
    font-weight: 800;
    color: var(--accent-contrast, #000000);
    letter-spacing: 0.05em;
  }

  .drawer-duration {
    position: absolute;
    bottom: 3px;
    right: 4px;
    font-size: 9px;
    font-weight: 700;
    background: rgba(0, 0, 0, 0.75);
    padding: 1px 4px;
    border-radius: 2px;
  }

  .drawer-ep-info {
    flex: 1;
    min-width: 0;
  }

  .drawer-ep-tags {
    display: flex;
    align-items: center;
    gap: 6px;
    margin-bottom: 4px;
  }

  .ep-num-pill {
    font-size: 11px;
    font-weight: 800;
    color: var(--accent);
  }

  .ep-sub-tag {
    font-size: 10px;
    color: #717789;
  }

  .drawer-ep-title {
    margin: 0;
    font-size: 13px;
    font-weight: 600;
    line-height: 1.35;
    color: #ffffff;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  /* Loader & Error States */
  .player-loader,
  .player-error-card {
    text-align: center;
    padding: 40px;
  }

  .cr-spinner {
    width: 44px;
    height: 44px;
    border: 3px solid rgba(255, 255, 255, 0.15);
    border-top-color: var(--accent);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
    margin: 0 auto 16px;
  }

  .player-error-card h3 {
    margin: 12px 0 6px;
    font-size: 18px;
  }

  .player-error-card p {
    color: #a0a6b5;
    margin-bottom: 20px;
  }

  .cr-btn {
    padding: 10px 20px;
    border-radius: 999px;
    background: var(--accent);
    color: #000000;
    font-weight: 700;
    border: none;
    cursor: pointer;
  }

  .server-pill {
    display: inline-flex;
    align-items: center;
    gap: 7px;
    padding: 6px 14px;
    border-radius: 999px;
    background: rgba(255, 255, 255, 0.08);
    border: 1px solid rgba(255, 255, 255, 0.14);
    color: #e8e8ee;
    font-size: 12px;
    font-weight: 700;
    cursor: pointer;
    transition: background 0.12s ease, border-color 0.12s ease;
  }

  .server-pill:hover {
    background: rgba(255, 255, 255, 0.14);
    border-color: var(--accent);
  }

  .server-pill:disabled {
    opacity: 0.6;
    cursor: wait;
  }

  .server-dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: var(--green, #10b981);
    box-shadow: 0 0 6px rgba(16, 185, 129, 0.5);
  }

  .embed-stage {
    width: 100%;
    height: 100%;
  }

  .embed-stage iframe {
    width: 100%;
    height: 100%;
    border: none;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(6px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  @keyframes slideLeft {
    from {
      transform: translateX(100%);
    }
    to {
      transform: translateX(0);
    }
  }
</style>
