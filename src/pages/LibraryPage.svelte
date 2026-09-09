<script lang="ts">
  import { onMount } from "svelte";
  import { api } from "../lib/api";
  import type { LocalItem } from "../lib/types";

  let folders: string[] = $state([]);
  let items: LocalItem[] = $state([]);
  let loading = $state(true);
  let error = $state("");
  let busy = $state(false);

  onMount(load);

  async function load() {
    loading = true;
    error = "";
    try {
      folders = await api.listLibraryFolders();
      items = await api.scanLibrary();
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function pickFolder() {
    try {
      const { open } = await import("@tauri-apps/plugin-dialog");
      const selected = await open({ directory: true, multiple: false });
      if (typeof selected === "string" && selected) {
        busy = true;
        await api.addLibraryFolder(selected);
        await load();
      }
    } catch (e) {
      error = String(e);
    } finally {
      busy = false;
    }
  }

  async function removeFolder(path: string) {
    await api.removeLibraryFolder(path);
    await load();
  }
</script>

<div class="page">
  <div class="head">
    <div>
      <h1 class="page-title">Library</h1>
      <p class="page-sub">Point localhost at your local anime folders. Files never leave your machine.</p>
    </div>
    <button class="btn primary" onclick={pickFolder} disabled={busy}>
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="15" height="15"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z" /></svg>
      Add folder
    </button>
  </div>

  {#if loading}
    <div class="spinner"></div>
  {:else if error}
    <div class="error-box"><span>{error}</span><button class="btn" onclick={load}>Retry</button></div>
  {:else}
    {#if folders.length > 0}
      <section>
        <div class="section-head">
          <h3>Watched folders</h3>
          <button class="link" onclick={load}>Rescan</button>
        </div>
        <div class="folder-list">
          {#each folders as f (f)}
            <div class="folder-row">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="f-icon"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z" /></svg>
              <span class="f-path" title={f}>{f}</span>
              <button class="btn danger" onclick={() => removeFolder(f)}>Remove</button>
            </div>
          {/each}
        </div>
      </section>
    {/if}

    {#if items.length > 0}
      <section>
        <div class="section-head">
          <h3>Shows found ({items.length})</h3>
        </div>
        <div class="show-grid">
          {#each items as item (item.title)}
            <a class="show-card" href={`#/anime/local/${encodeURIComponent(item.title)}`}>
              <div class="show-icon">
                <svg viewBox="0 0 24 24" fill="currentColor"><path d="M8 5v14l11-7z" /></svg>
              </div>
              <div class="show-title" title={item.title}>{item.title}</div>
              <div class="show-count">{item.episodeCount} episode{item.episodeCount === 1 ? "" : "s"}</div>
            </a>
          {/each}
        </div>
      </section>
    {:else if folders.length > 0}
      <div class="empty-state">
        <div class="big">📁</div>
        <p>No video files found in those folders yet.</p>
        <p class="hint">Each subfolder counts as one show; video files inside become episodes.</p>
      </div>
    {:else}
      <div class="empty-state">
        <div class="big">📁</div>
        <p>No folders added.</p>
        <p class="hint">Local shows take priority over streaming sources when titles match.</p>
      </div>
    {/if}
  {/if}
</div>

<style>
  .head {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: 16px;
  }

  .folder-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
    max-width: 860px;
  }

  .folder-row {
    display: flex;
    align-items: center;
    gap: 12px;
    background: var(--surface);
    border: 1px solid var(--border-soft);
    border-radius: var(--radius-sm);
    padding: 10px 14px;
  }

  .f-icon {
    width: 16px;
    height: 16px;
    color: var(--accent);
    flex-shrink: 0;
  }

  .f-path {
    flex: 1;
    font-size: 13px;
    color: var(--text-dim);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    direction: rtl;
    text-align: left;
  }

  .show-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(210px, 1fr));
    gap: 12px;
  }

  .show-card {
    background: var(--surface);
    border: 1px solid var(--border-soft);
    border-radius: var(--radius);
    padding: 18px 16px;
    color: var(--text);
    transition: border-color 0.12s ease;
  }

  .show-card:hover {
    border-color: var(--accent);
  }

  .show-icon {
    width: 38px;
    height: 38px;
    border-radius: 9px;
    background: var(--accent-dim);
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--accent);
    margin-bottom: 12px;
  }

  .show-icon svg {
    width: 18px;
    height: 18px;
  }

  .show-title {
    font-weight: 600;
    font-size: 13.5px;
    margin-bottom: 3px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .show-count {
    font-size: 12px;
    color: var(--text-faint);
  }
</style>
