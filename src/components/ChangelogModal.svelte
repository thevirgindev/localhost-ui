<script lang="ts">
  import { userStore } from "../lib/userStore.svelte";

  function close() {
    userStore.showChangelogModal = false;
  }
</script>

<div class="changelog-backdrop" onclick={close} role="button" tabindex="0" onkeydown={(e) => e.key === "Escape" && close()}>
  <div
    class="changelog-card"
    onclick={(e) => e.stopPropagation()}
    role="dialog"
    aria-label="What's New in Crunchyroll Anime"
    tabindex="-1"
  >
    <div class="changelog-header">
      <div class="header-left">
        <span class="version-tag">Version 2.4.0 Update</span>
        <h2 class="title">What's New in Luci Anime</h2>
      </div>
      <button class="close-btn" onclick={close} title="Close"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" width="14" height="14" stroke-linecap="round"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg></button>
    </div>

    <div class="changelog-body">
      <div class="release-item">
        <div class="release-badge new">New</div>
        <div class="release-content">
          <h4>Universal Search Center</h4>
          <p>Click the search icon in the top navigation or press <kbd>Ctrl+K</kbd> to launch the centered command modal with category filters, live results, and trending searches.</p>
        </div>
      </div>

      <div class="release-item">
        <div class="release-badge feature">Feature</div>
        <div class="release-content">
          <h4>Settings & Preferences Center</h4>
          <p>Configure themes (Dark, Light, OLED, Charcoal), video playback parameters (1080p, auto-skip intro), subtitle colors, audio preferences, and local profile management.</p>
        </div>
      </div>

      <div class="release-item">
        <div class="release-badge feature">Feature</div>
        <div class="release-content">
          <h4>Local User Profiles & Switcher</h4>
          <p>Switch between profiles or create customized profiles with unique anime avatars, child content locks, and localized watch history.</p>
        </div>
      </div>

      <div class="release-item">
        <div class="release-badge improve">Improvement</div>
        <div class="release-content">
          <h4>Luci Precision Typography & Episodes Layout</h4>
          <p>Integrated refined Overpass typography, streamlined episode lists, and embedded full metadata tags in the hero header.</p>
        </div>
      </div>

      <div class="release-item">
        <div class="release-badge improve">Improvement</div>
        <div class="release-content">
          <h4>Airing Notifications & Clean Catalog Grids</h4>
          <p>Stay up to date with new episode broadcasts from One Piece, Demon Slayer, and Jujutsu Kaisen, and toggle grid densities in the catalog.</p>
        </div>
      </div>
    </div>

    <div class="changelog-footer">
      <button class="btn primary" onclick={close}>Got it, let's watch</button>
    </div>
  </div>
</div>

<style>
  .changelog-backdrop {
    position: fixed;
    inset: 0;
    z-index: 2500;
    background: rgba(0, 0, 0, 0.82);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 20px;
    animation: fadeIn 0.15s ease-out;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  .changelog-card {
    width: 100%;
    max-width: 620px;
    background: #141519;
    border: 1px solid #282a32;
    border-radius: 6px;
    overflow: hidden;
    box-shadow: 0 24px 64px rgba(0, 0, 0, 0.9);
  }

  .changelog-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    padding: 24px 28px 18px;
    border-bottom: 1px solid #23252b;
    background: #181a20;
  }

  .version-tag {
    font-size: 11px;
    font-weight: 800;
    color: #a855f7;
    letter-spacing: 0.08em;
  }

  .title {
    font-size: 20px;
    font-weight: 800;
    color: #ffffff;
    margin: 4px 0 0;
  }

  .close-btn {
    background: transparent;
    border: none;
    color: #8c8c92;
    font-size: 16px;
    cursor: pointer;
    padding: 4px;
    border-radius: 4px;
  }

  .close-btn:hover {
    color: #ffffff;
  }

  .changelog-body {
    padding: 24px 28px;
    display: flex;
    flex-direction: column;
    gap: 18px;
    max-height: 420px;
    overflow-y: auto;
  }

  .release-item {
    display: flex;
    gap: 14px;
    align-items: flex-start;
  }

  .release-badge {
    font-size: 10px;
    font-weight: 900;
    padding: 2px 6px;
    border-radius: 3px;
    letter-spacing: 0.04em;
    flex-shrink: 0;
    margin-top: 2px;
  }

  .release-badge.new {
    background: #a855f7;
    color: #000000;
  }

  .release-badge.feature {
    background: #3b82f6;
    color: #ffffff;
  }

  .release-badge.improve {
    background: #10b981;
    color: #ffffff;
  }

  .release-content h4 {
    margin: 0 0 4px;
    font-size: 14px;
    font-weight: 700;
    color: #ffffff;
  }

  .release-content p {
    margin: 0;
    font-size: 13px;
    color: #9c9ca3;
    line-height: 1.5;
  }

  kbd {
    background: #23252b;
    border: 1px solid #33363e;
    color: #ffffff;
    padding: 1px 4px;
    border-radius: 3px;
    font-size: 11px;
  }

  .changelog-footer {
    padding: 16px 28px;
    border-top: 1px solid #23252b;
    background: #111215;
    display: flex;
    justify-content: flex-end;
  }
</style>
