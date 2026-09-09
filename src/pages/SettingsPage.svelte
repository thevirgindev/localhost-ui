<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "../lib/ipc";

  let cacheMinutes = $state(30);
  let saving = $state(false);
  let savedNote = $state("");

  onMount(async () => {
    try {
      const stored = await invoke<string | null>("get_setting", { key: "cache_minutes" });
      if (stored) cacheMinutes = Number(stored) || 30;
    } catch {
      // defaults are fine
    }
  });

  async function saveCache() {
    saving = true;
    try {
      await invoke("set_setting", { key: "cache_minutes", value: String(cacheMinutes) });
      savedNote = "Preferences saved.";
      setTimeout(() => (savedNote = ""), 2200);
    } finally {
      saving = false;
    }
  }
</script>

<div class="page">
  <h1 class="page-title">Settings</h1>
  <p class="page-sub">localhost stores everything on this machine. There are no accounts, no sync, no telemetry.</p>

  <section class="panel">
    <h3>Playback & Cache</h3>
    <div class="row">
      <div>
        <div class="row-title">Metadata cache lifetime</div>
        <div class="row-desc">
          Duration in minutes that AniList and Jikan responses remain cached locally.
        </div>
      </div>
      <input
        type="number"
        min="5"
        max="1440"
        bind:value={cacheMinutes}
        class="cache-input"
      />
    </div>
    <div class="row-actions">
      <button class="btn primary" onclick={saveCache} disabled={saving}>Save Preferences</button>
      {#if savedNote}
        <span class="saved">{savedNote}</span>
      {/if}
    </div>
  </section>

  <section class="panel">
    <h3>Privacy & Security</h3>
    <div class="privacy-grid">
      <div class="p-item">
        <span class="p-dot green"></span>
        <div>
          <div class="row-title">No user accounts</div>
          <div class="row-desc">Your list, status, and watch progress live strictly in a local SQLite file.</div>
        </div>
      </div>
      <div class="p-item">
        <span class="p-dot green"></span>
        <div>
          <div class="row-title">Zero tracking</div>
          <div class="row-desc">No tracking pixels, crash telemetry, or third-party ad beacons.</div>
        </div>
      </div>
      <div class="p-item">
        <span class="p-dot green"></span>
        <div>
          <div class="row-title">Offline local playback</div>
          <div class="row-desc">Local video directories stream over the secure OS asset protocol.</div>
        </div>
      </div>
      <div class="p-item">
        <span class="p-dot amber"></span>
        <div>
          <div class="row-title">Metadata lookups</div>
          <div class="row-desc">Only public endpoints graphql.anilist.co and api.jikan.moe are queried.</div>
        </div>
      </div>
    </div>
  </section>

  <section class="panel">
    <h3>Data Storage</h3>
    <div class="row">
      <div>
        <div class="row-title">Local database path</div>
        <div class="row-desc">
          Windows: <code>%APPDATA%\localhost\localhost.db</code> — contains watchlist, progress history, and folder pointers.
        </div>
      </div>
    </div>
  </section>
</div>

<style>
  .panel {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    padding: 24px 28px;
    margin-bottom: 20px;
    max-width: 820px;
  }

  .panel h3 {
    font-size: 16px;
    letter-spacing: -0.01em;
    margin-bottom: 16px;
  }

  .row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 24px;
    margin-bottom: 16px;
  }

  .row-title {
    font-weight: 700;
    font-size: 14px;
    margin-bottom: 4px;
    color: var(--text);
  }

  .row-desc {
    font-size: 13px;
    color: var(--text-dim);
    line-height: 1.5;
  }

  .cache-input {
    width: 90px;
    text-align: center;
    border-radius: 999px;
  }

  code {
    background: var(--surface-2);
    padding: 2px 8px;
    border-radius: 4px;
    font-size: 12.5px;
    font-family: var(--mono);
    color: var(--accent);
  }

  .row-actions {
    display: flex;
    align-items: center;
    gap: 14px;
    margin-top: 8px;
  }

  .saved {
    color: var(--green);
    font-size: 13px;
    font-weight: 700;
  }

  .privacy-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 18px;
  }

  .p-item {
    display: flex;
    gap: 12px;
    align-items: flex-start;
  }

  .p-dot {
    width: 9px;
    height: 9px;
    border-radius: 50%;
    margin-top: 5px;
    flex-shrink: 0;
  }

  .p-dot.green {
    background: var(--green);
  }

  .p-dot.amber {
    background: #e8c268;
  }

  @media (max-width: 760px) {
    .privacy-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
