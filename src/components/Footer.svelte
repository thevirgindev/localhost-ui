<script lang="ts">
  import { userStore } from "../lib/userStore.svelte";
  import { router } from "../lib/router";

  function scrollToTop() {
    window.scrollTo({ top: 0, behavior: "smooth" });
  }

  function openSearch() {
    userStore.showSearchModal = true;
  }

  function goToSettings() {
    router.navigate({ name: "settings" });
  }
</script>

<footer class="luci-footer" id="main-footer">
  <div class="footer-container">
    <!-- Top Row: Brand, Description, Fast Links -->
    <div class="footer-grid">
      <!-- Brand & Mission Column -->
      <div class="footer-brand-col">
        <div class="brand-row">
          <div class="brand-logo-mark">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" class="logo-icon">
              <polygon points="5 3 19 12 5 21 5 3" />
            </svg>
          </div>
          <span class="brand-name">Luci</span>
          <span class="brand-pill">v2.4</span>
        </div>

        <p class="brand-desc">
          A minimalist, local-first anime streaming and catalog client. Designed for pure focus, fast navigation, and zero telemetry.
        </p>

        <div class="status-indicator">
          <span class="status-dot"></span>
          <span class="status-text">Local Storage Engine Active • All Caches Synced</span>
        </div>
      </div>

      <!-- Navigation Links -->
      <div class="footer-nav-col">
        <h4 class="col-heading">Explore</h4>
        <ul class="link-list">
          <li><a href="#/" class="footer-link">Home</a></li>
          <li><a href="#/browse" class="footer-link">Catalog Browse</a></li>
          <li><a href="#/season" class="footer-link">Seasonal Lineup</a></li>
          <li>
            <button class="footer-link-btn" onclick={openSearch}>
              Quick Search <span class="shortcut-tag">Ctrl+K</span>
            </button>
          </li>
        </ul>
      </div>

      <!-- Library & Personal -->
      <div class="footer-nav-col">
        <h4 class="col-heading">Library</h4>
        <ul class="link-list">
          <li><a href="#/library?tab=watchlist" class="footer-link">My Lists</a></li>
          <li><a href="#/library?tab=continue" class="footer-link">Continue Watching</a></li>
          <li><a href="#/library?tab=local" class="footer-link">Local Folder Scan</a></li>
          <li><a href="#/settings" class="footer-link">Custom Presets</a></li>
        </ul>
      </div>

      <!-- Settings & Controls -->
      <div class="footer-nav-col">
        <h4 class="col-heading">Appearance</h4>
        <div class="theme-quick-info">
          <div class="current-theme-badge">
            <span class="theme-label">Active Theme:</span>
            <span class="theme-name">{userStore.preferences?.theme || "dark"}</span>
          </div>
          <p class="theme-hint">
            Customize typography, corner radius, and accent color in real-time.
          </p>
          <button class="manage-appearance-btn" onclick={goToSettings}>
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="14" height="14">
              <circle cx="12" cy="12" r="3" />
              <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" />
            </svg>
            Appearance Settings
          </button>
        </div>
      </div>
    </div>

    <!-- Middle Divider with Shortcuts -->
    <div class="shortcuts-strip">
      <div class="shortcuts-title">Key Shortcuts:</div>
      <div class="shortcuts-group">
        <span class="shortcut-item"><kbd>Ctrl</kbd> + <kbd>K</kbd> Search</span>
        <span class="shortcut-item"><kbd>Space</kbd> Play/Pause</span>
        <span class="shortcut-item"><kbd>M</kbd> Mute</span>
        <span class="shortcut-item"><kbd>F</kbd> Fullscreen</span>
        <span class="shortcut-item"><kbd>Esc</kbd> Close Overlay</span>
      </div>
      <button class="back-to-top-btn" onclick={scrollToTop} title="Back to top">
        <span>Back to Top</span>
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" width="14" height="14">
          <polyline points="18 15 12 9 6 15" />
        </svg>
      </button>
    </div>

    <!-- Bottom Bar -->
    <div class="footer-bottom-bar">
      <div class="copyright-text">
        © 2026 <strong>Luci</strong>. Local anime streaming & media organizer.
      </div>
      <div class="footer-badges">
        <span class="f-badge">Svelte 5</span>
        <span class="f-badge">Zero Ads</span>
        <span class="f-badge">Privacy First</span>
      </div>
    </div>
  </div>
</footer>

<style>
  .luci-footer {
    width: 100%;
    background: var(--surface, #0d0e12);
    border-top: 1px solid var(--border, #1f222b);
    color: var(--text-dim, #9699a6);
    padding: 56px 0 28px;
    margin-top: 64px;
  }

  .footer-container {
    max-width: 1600px;
    margin: 0 auto;
    padding: 0 48px;
  }

  @media (max-width: 768px) {
    .footer-container {
      padding: 0 20px;
    }
  }

  .footer-grid {
    display: grid;
    grid-template-columns: 2fr 1fr 1fr 1.3fr;
    gap: 40px;
    margin-bottom: 40px;
  }

  @media (max-width: 1024px) {
    .footer-grid {
      grid-template-columns: 1fr 1fr;
      gap: 32px;
    }
  }

  @media (max-width: 640px) {
    .footer-grid {
      grid-template-columns: 1fr;
      gap: 28px;
    }
  }

  /* Brand Column */
  .footer-brand-col {
    display: flex;
    flex-direction: column;
    gap: 14px;
  }

  .brand-row {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .brand-logo-mark {
    width: 30px;
    height: 30px;
    background: var(--accent);
    color: var(--accent-contrast, #000000);
    border-radius: var(--radius-sm, 4px);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .logo-icon {
    width: 16px;
    height: 16px;
    fill: currentColor;
  }

  .brand-name {
    font-size: 20px;
    font-weight: 900;
    letter-spacing: -0.02em;
    color: var(--text, #ffffff);
  }

  .brand-pill {
    font-size: 10px;
    font-weight: 800;
    padding: 2px 6px;
    border-radius: 4px;
    background: var(--surface-2, #181920);
    border: 1px solid var(--border, #282a34);
    color: var(--text-faint, #6f7282);
    letter-spacing: 0.05em;
  }

  .brand-desc {
    font-size: 13.5px;
    line-height: 1.6;
    color: var(--text-dim, #8e91a0);
    margin: 0;
    max-width: 380px;
  }

  .status-indicator {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 12px;
    font-weight: 600;
    color: var(--text-faint, #737686);
    margin-top: 4px;
  }

  .status-dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: #10b981;
    box-shadow: 0 0 8px rgba(16, 185, 129, 0.4);
  }

  /* Nav Columns */
  .footer-nav-col {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .col-heading {
    font-size: 12px;
    font-weight: 800;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: var(--text, #ffffff);
    margin: 0 0 6px;
  }

  .link-list {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .footer-link {
    font-size: 13.5px;
    color: var(--text-dim, #8e91a0);
    text-decoration: none;
    transition: color 0.12s ease;
    display: inline-block;
  }

  .footer-link:hover {
    color: var(--accent);
  }

  .footer-link-btn {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    background: transparent;
    border: none;
    padding: 0;
    color: var(--text-dim, #8e91a0);
    font-size: 13.5px;
    font-family: inherit;
    cursor: pointer;
    transition: color 0.12s ease;
  }

  .footer-link-btn:hover {
    color: var(--accent);
  }

  .shortcut-tag {
    font-size: 10px;
    font-weight: 800;
    background: var(--surface-2, #181920);
    border: 1px solid var(--border, #282a34);
    padding: 1px 5px;
    border-radius: 3px;
    color: var(--text-faint, #6f7282);
  }

  /* Appearance Column */
  .theme-quick-info {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .current-theme-badge {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 12.5px;
  }

  .theme-label {
    color: var(--text-faint, #737686);
    font-weight: 600;
  }

  .theme-name {
    color: var(--accent);
    font-weight: 800;
    text-transform: capitalize;
  }

  .theme-hint {
    font-size: 12px;
    line-height: 1.5;
    color: var(--text-faint, #6f7282);
    margin: 0;
  }

  .manage-appearance-btn {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 7px 12px;
    background: var(--surface-2, #181920);
    border: 1px solid var(--border, #282a34);
    border-radius: var(--radius-sm, 4px);
    color: var(--text, #ffffff);
    font-size: 12px;
    font-weight: 700;
    cursor: pointer;
    font-family: inherit;
    transition: all 0.12s ease;
    width: fit-content;
    margin-top: 4px;
  }

  .manage-appearance-btn:hover {
    background: var(--surface-3, #22242e);
    border-color: var(--accent);
    color: var(--accent);
  }

  /* Shortcuts Strip */
  .shortcuts-strip {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    padding: 16px 20px;
    background: var(--surface-2, #121318);
    border: 1px solid var(--border, #1f222b);
    border-radius: var(--radius, 6px);
    margin-bottom: 28px;
    flex-wrap: wrap;
  }

  .shortcuts-title {
    font-size: 12px;
    font-weight: 800;
    color: var(--text-faint, #6f7282);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .shortcuts-group {
    display: flex;
    align-items: center;
    gap: 16px;
    flex-wrap: wrap;
  }

  .shortcut-item {
    font-size: 12px;
    color: var(--text-dim, #9699a6);
    display: inline-flex;
    align-items: center;
    gap: 4px;
  }

  kbd {
    background: var(--surface-3, #1e2029);
    border: 1px solid var(--border, #2a2d39);
    color: var(--text, #ffffff);
    padding: 2px 5px;
    border-radius: 3px;
    font-size: 10px;
    font-weight: 700;
    font-family: inherit;
  }

  .back-to-top-btn {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    background: transparent;
    border: none;
    color: var(--text-dim, #8e91a0);
    font-size: 12px;
    font-weight: 700;
    cursor: pointer;
    font-family: inherit;
    transition: color 0.12s ease;
    margin-left: auto;
  }

  .back-to-top-btn:hover {
    color: var(--accent);
  }

  /* Bottom Bar */
  .footer-bottom-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    font-size: 12.5px;
    color: var(--text-faint, #626574);
    flex-wrap: wrap;
    padding-top: 8px;
  }

  .footer-bottom-bar strong {
    color: var(--text, #ffffff);
  }

  .footer-badges {
    display: flex;
    gap: 8px;
  }

  .f-badge {
    font-size: 10.5px;
    font-weight: 700;
    padding: 2px 8px;
    background: var(--surface-2, #14151a);
    border: 1px solid var(--border, #22242d);
    border-radius: var(--radius-sm, 4px);
    color: var(--text-faint, #737686);
  }
</style>
