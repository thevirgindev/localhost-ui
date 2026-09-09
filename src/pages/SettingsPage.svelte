<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "../lib/ipc";

  let cacheMinutes = $state(30);
  let saving = $state(false);
  let savedNote = $state("");

  onMount(async () => {
    try {
      // data dir command is not exposed; use a setting proxy for cache ttl.
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
      savedNote = "Saved.";
      setTimeout(() => (savedNote = ""), 1800);
    } finally {
      saving = false;
    }
  }
</script>

<div class="page">
  <h1 class="page-title">Settings</h1>
  <p class="page-sub">localhost stores everything on this machine. There are no accounts, no sync, no telemetry.</p>

  <section class="panel">
    <h3>Playback</h3>
    <div class="row">
      <div>
        <div class="row-title">Metadata cache lifetime</div>
        <div class="row-desc">How long AniList/Jikan responses stay cached (minutes). Lower = fresher data, more requests.</div>
      </div>
      <input
        type="number"
        min="5"
        max="1440"
        bind:value={cacheMinutes}
        style="width: 90px"
      />
    </div>
    <div class="row-actions">
      <button class="btn primary" onclick={saveCache} disabled={saving}>Save</button>
      {#if savedNote}<span class="saved">{savedNote}</span>{/if}
    </div>
  </section>

  <section class="panel">
    <h3>Privacy</h3>
    <div class="privacy-grid">
      <div class="p-item">
        <span class="p-dot green"></span>
        <div>
          <div class="row-title">No accounts</div>
          <div class="row-desc">Your list and progress live in a local SQLite database.</div>
        </div>
      </div>
      <div class="p-item">
        <span class="p-dot green"></span>
        <div>
          <div class="row-title">No tracking</div>
          <div class="row-desc">No analytics, no crash reporting, no third-party scripts.</div>
        </div>
      </div>
      <div class="p-item">
        <span class="p-dot green"></span>
        <div>
          <div class="row-title">Local library is offline</div>
          <div class="row-desc">Local files play through the OS asset protocol — never uploaded anywhere.</div>
        </div>
      </div>
      <div class="p-item">
        <span class="p-dot amber"></span>
        <div>
          <div class="row-title">Metadata fetches</div>
          <div class="row-desc">Only two read-only public APIs are contacted: graphql.anilist.co and api.jikan.moe.</div>
        </div>
      </div>
    </div>
  </section>

  <section class="panel">
    <h3>Data</h3>
    <div class="row">
      <div>
        <div class="row-title">Where is my data?</div>
        <div class="row-desc">
          Windows: <code>%APPDATA%\localhost\localhost.db</code> — watchlist, progress, settings, library folders.
          Delete that file to reset the app.
        </div>
      </div>
    </div>
  </section>
</div>

<style>
  .panel {
    background: var(--surface);
    border: 1px solid var(--border-soft);
    border-radius: var(--radius);
    padding: 20px 22px;
    margin-bottom: 16px;
    max-width: 760px;
  }

  .panel h3 {
    font-size: 15px;
    margin-bottom: 14px;
  }

  .row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 20px;
    margin-bottom: 12px;
  }

  .row-title {
    font-weight: 600;
    font-size: 13.5px;
    margin-bottom: 3px;
  }

  .row-desc {
    font-size: 12.5px;
    color: var(--text-dim);
    line-height: 1.5;
  }

  code {
    background: var(--surface-2);
    padding: 1px 6px;
    border-radius: 4px;
    font-size: 12px;
  }

  .row-actions {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .saved {
    color: var(--green);
    font-size: 12.5px;
    font-weight: 600;
  }

  .privacy-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 14px;
  }

  .p-item {
    display: flex;
    gap: 10px;
    align-items: flex-start;
  }

  .p-dot {
    width: 8px;
    height: 8px;
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
</style>
