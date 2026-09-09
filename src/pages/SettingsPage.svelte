<script lang="ts">
  import { onMount } from "svelte";
  import { userStore, BUILT_IN_PRESETS, type CustomThemePreset, type AppPreferences } from "../lib/userStore.svelte";

  let activeTab = $state<
    | "account"
    | "profiles"
    | "appearance"
    | "playback"
    | "subtitles"
    | "notifications"
    | "keybinds"
    | "data"
    | "about"
  >("appearance");

  let usernameInput = $state(userStore.activeProfile.name);
  let statusMessage = $state("");

  // Appearance reactive bindings
  let currentTheme = $state<AppPreferences["theme"]>(userStore.preferences.theme || "dark");
  let accentColor = $state(userStore.preferences.accentColor || "#a855f7");
  let radius = $state(userStore.preferences.radius || "4");
  let fontFamily = $state(userStore.preferences.fontFamily || "Overpass");
  let cardStyle = $state<AppPreferences["cardStyle"]>(userStore.preferences.cardStyle || "bordered");
  let density = $state<AppPreferences["density"]>(userStore.preferences.density || "comfortable");

  // Playback & preferences bindings
  let videoQuality = $state(userStore.preferences.videoQuality);
  let autoplay = $state(userStore.preferences.autoplay);
  let autoSkipIntro = $state(userStore.preferences.autoSkipIntro);
  let audioLanguage = $state(userStore.preferences.audioLanguage);
  let subtitleLanguage = $state(userStore.preferences.subtitleLanguage);
  let subtitleColor = $state(userStore.preferences.subtitleColor);
  let subtitleSize = $state(userStore.preferences.subtitleSize);
  let subtitleBg = $state(userStore.preferences.subtitleBg);
  let airingAlerts = $state(userStore.preferences.airingAlerts);
  let cacheMinutes = $state(userStore.preferences.cacheMinutes);

  // Custom preset creation input
  let newPresetName = $state("");
  let showCreatePreset = $state(false);

  // Check if any settings have changed from store
  const hasUnsavedChanges = $derived.by(() => {
    const p = userStore.preferences;
    return (
      currentTheme !== p.theme ||
      accentColor !== p.accentColor ||
      radius !== p.radius ||
      fontFamily !== p.fontFamily ||
      cardStyle !== p.cardStyle ||
      density !== p.density ||
      videoQuality !== p.videoQuality ||
      autoplay !== p.autoplay ||
      autoSkipIntro !== p.autoSkipIntro ||
      audioLanguage !== p.audioLanguage ||
      subtitleLanguage !== p.subtitleLanguage ||
      subtitleColor !== p.subtitleColor ||
      subtitleSize !== p.subtitleSize ||
      subtitleBg !== p.subtitleBg ||
      airingAlerts !== p.airingAlerts ||
      cacheMinutes !== p.cacheMinutes
    );
  });

  onMount(() => {
    function onKeyDown(e: KeyboardEvent) {
      if (e.key === "Escape") {
        closeSettings();
      }
    }
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  });

  function closeSettings() {
    userStore.showSettingsModal = false;
    if (window.location.hash === "#/settings") {
      history.back();
    }
  }

  function saveProfileName() {
    if (!usernameInput.trim()) return;
    userStore.updateProfile(userStore.activeProfile.id, { name: usernameInput.trim() });
    showNotice("Profile username updated.");
  }

  // Live preview & change handlers
  function setAppTheme(t: AppPreferences["theme"]) {
    currentTheme = t;
    userStore.updatePreferences({ theme: t });
  }

  function setAccentColor(hex: string) {
    accentColor = hex;
    userStore.updatePreferences({ accentColor: hex });
  }

  function setCornerRadius(r: string) {
    radius = r;
    userStore.updatePreferences({ radius: r });
  }

  function setFont(f: string) {
    fontFamily = f;
    userStore.updatePreferences({ fontFamily: f });
  }

  function setCardStyle(cs: AppPreferences["cardStyle"]) {
    cardStyle = cs;
    userStore.updatePreferences({ cardStyle: cs });
  }

  function setDensity(d: AppPreferences["density"]) {
    density = d;
    userStore.updatePreferences({ density: d });
  }

  function applyPreset(preset: CustomThemePreset) {
    currentTheme = preset.theme;
    accentColor = preset.accentColor;
    radius = preset.radius;
    fontFamily = preset.fontFamily;
    cardStyle = preset.cardStyle;
    userStore.applyPreset(preset);
    showNotice(`Applied preset: ${preset.name}`);
  }

  function handleSavePreset() {
    if (!newPresetName.trim()) return;
    userStore.saveCustomPreset(newPresetName.trim());
    newPresetName = "";
    showCreatePreset = false;
    showNotice("Custom preset saved!");
  }

  function saveAllSettings() {
    userStore.updatePreferences({
      theme: currentTheme,
      accentColor,
      radius,
      fontFamily,
      cardStyle,
      density,
      videoQuality,
      autoplay,
      autoSkipIntro,
      audioLanguage,
      subtitleLanguage,
      subtitleColor,
      subtitleSize,
      subtitleBg,
      airingAlerts,
      cacheMinutes,
    });
    showNotice("All settings saved successfully!");
  }

  function resetAllSettings() {
    const p = userStore.preferences;
    currentTheme = p.theme;
    accentColor = p.accentColor;
    radius = p.radius;
    fontFamily = p.fontFamily;
    cardStyle = p.cardStyle;
    density = p.density;
    videoQuality = p.videoQuality;
    autoplay = p.autoplay;
    autoSkipIntro = p.autoSkipIntro;
    audioLanguage = p.audioLanguage;
    subtitleLanguage = p.subtitleLanguage;
    subtitleColor = p.subtitleColor;
    subtitleSize = p.subtitleSize;
    subtitleBg = p.subtitleBg;
    airingAlerts = p.airingAlerts;
    cacheMinutes = p.cacheMinutes;
    showNotice("Settings reset to current saved values.");
  }

  function showNotice(msg: string) {
    statusMessage = msg;
    setTimeout(() => {
      if (statusMessage === msg) statusMessage = "";
    }, 2500);
  }

  async function exportWatchHistory() {
    try {
      const historyData = localStorage.getItem("anime_progress_map") || "{}";
      const blob = new Blob([historyData], { type: "application/json" });
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = `luci_watch_history_${userStore.activeProfile.name.toLowerCase()}.json`;
      a.click();
      URL.revokeObjectURL(url);
      showNotice("Watch history exported successfully.");
    } catch {
      showNotice("Export failed.");
    }
  }

  function clearLocalCache() {
    localStorage.removeItem("luci_anime_cache");
    showNotice("Local anime cache flushed and refreshed.");
  }
</script>

<div
  class="discord-settings-backdrop"
  onclick={closeSettings}
  role="button"
  tabindex="0"
  onkeydown={(e) => e.key === "Escape" && closeSettings()}
>
  <!-- Centered Big Discord Card -->
  <div
    class="discord-settings-card"
    onclick={(e) => e.stopPropagation()}
    role="dialog"
    aria-label="User Settings"
    tabindex="-1"
  >
    <!-- Left Sidebar -->
    <aside class="discord-sidebar">
      <div class="sidebar-scroller">
        <!-- Section: User Settings -->
        <div class="nav-section-title">User Settings</div>
        <button
          class="nav-tab"
          class:active={activeTab === "account"}
          onclick={() => (activeTab = "account")}
        >
          <span class="tab-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="15" height="15" stroke-linecap="round" stroke-linejoin="round"><path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" /><circle cx="12" cy="7" r="4" /></svg></span>
          <span>My Account</span>
        </button>
        <button
          class="nav-tab"
          class:active={activeTab === "profiles"}
          onclick={() => (activeTab = "profiles")}
        >
          <span class="tab-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="15" height="15" stroke-linecap="round" stroke-linejoin="round"><path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" /><circle cx="9" cy="7" r="4" /><path d="M23 21v-2a4 4 0 0 0-3-3.87" /><path d="M16 3.13a4 4 0 0 1 0 7.75" /></svg></span>
          <span>Profiles</span>
          <span class="active-badge">{userStore.profiles.length}</span>
        </button>

        <div class="nav-separator"></div>

        <!-- Section: App Settings -->
        <div class="nav-section-title">App Settings</div>
        <button
          class="nav-tab"
          class:active={activeTab === "appearance"}
          onclick={() => (activeTab = "appearance")}
        >
          <span class="tab-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="15" height="15" stroke-linecap="round" stroke-linejoin="round"><circle cx="13.5" cy="6.5" r=".5" /><circle cx="17.5" cy="10.5" r=".5" /><circle cx="8.5" cy="7.5" r=".5" /><circle cx="6.5" cy="12.5" r=".5" /><path d="M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10c.926 0 1.648-.746 1.648-1.688 0-.437-.18-.835-.437-1.125-.29-.289-.438-.652-.438-1.125a1.64 1.64 0 0 1 1.668-1.668h1.996c3.051 0 5.555-2.503 5.555-5.554C21.965 6.012 17.461 2 12 2z" /></svg></span>
          <span>Appearance & Theme</span>
        </button>
        <button
          class="nav-tab"
          class:active={activeTab === "playback"}
          onclick={() => (activeTab = "playback")}
        >
          <span class="tab-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="15" height="15" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10" /><polygon points="10 8 16 12 10 16 10 8" /></svg></span>
          <span>Video & Playback</span>
        </button>
        <button
          class="nav-tab"
          class:active={activeTab === "subtitles"}
          onclick={() => (activeTab = "subtitles")}
        >
          <span class="tab-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="15" height="15" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" /></svg></span>
          <span>Audio & Subtitles</span>
        </button>
        <button
          class="nav-tab"
          class:active={activeTab === "notifications"}
          onclick={() => (activeTab = "notifications")}
        >
          <span class="tab-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="15" height="15" stroke-linecap="round" stroke-linejoin="round"><path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9" /><path d="M13.73 21a2 2 0 0 1-3.46 0" /></svg></span>
          <span>Airing Alerts</span>
          {#if userStore.unreadNotificationsCount > 0}
            <span class="unread-pill">{userStore.unreadNotificationsCount}</span>
          {/if}
        </button>
        <button
          class="nav-tab"
          class:active={activeTab === "keybinds"}
          onclick={() => (activeTab = "keybinds")}
        >
          <span class="tab-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="15" height="15" stroke-linecap="round" stroke-linejoin="round"><rect x="2" y="4" width="20" height="16" rx="2" /><path d="M6 8h.01M10 8h.01M14 8h.01M18 8h.01M6 12h.01M10 12h.01M14 12h.01M18 12h.01M8 16h8" /></svg></span>
          <span>Keybinds</span>
        </button>

        <div class="nav-separator"></div>

        <!-- Section: Data Management -->
        <div class="nav-section-title">Data & Storage</div>
        <button
          class="nav-tab"
          class:active={activeTab === "data"}
          onclick={() => (activeTab = "data")}
        >
          <span class="tab-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="15" height="15" stroke-linecap="round" stroke-linejoin="round"><path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z" /><polyline points="17 21 17 13 7 13 7 21" /><polyline points="7 3 7 8 15 8" /></svg></span>
          <span>Storage & Cache</span>
        </button>
        <button
          class="nav-tab"
          class:active={activeTab === "about"}
          onclick={() => (activeTab = "about")}
        >
          <span class="tab-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="15" height="15" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10" /><path d="M12 16v-4M12 8h.01" /></svg></span>
          <span>About Luci</span>
        </button>

        <div class="nav-separator"></div>

        <button
          class="nav-tab logout"
          onclick={() => {
            userStore.showProfileModal = true;
            closeSettings();
          }}
        >
          <span class="tab-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="15" height="15" stroke-linecap="round" stroke-linejoin="round"><path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" /><polyline points="16 17 21 12 16 7" /><line x1="21" y1="12" x2="9" y2="12" /></svg></span>
          <span>Log Out</span>
        </button>
      </div>
    </aside>

    <!-- Right Main Content Area -->
    <main class="discord-content">
      <!-- Discord Close Button at Top Right with 1:1 circular highlight -->
      <div class="discord-close-col">
        <button class="discord-esc-btn" onclick={closeSettings} title="Close Settings (ESC)">
          <div class="esc-circle"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" width="14" height="14" stroke-linecap="round"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg></div>
          <span class="esc-label">ESC</span>
        </button>
      </div>

      <div class="content-scroll">
        {#if statusMessage}
          <div class="status-banner">
            <span><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" width="13" height="13" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"/></svg> {statusMessage}</span>
          </div>
        {/if}

        <!-- TAB 1: MY ACCOUNT -->
        {#if activeTab === "account"}
          <div class="tab-pane">
            <h2 class="pane-title">My Account</h2>
            <p class="pane-desc">Manage your local profile credentials, display name, and avatar settings.</p>

            <div class="account-card">
              <div class="account-banner" style="background: {userStore.activeProfile.color}"></div>
              <div class="account-card-body">
                <div class="avatar-row">
                  <div class="account-avatar-box">
                    <img src={userStore.activeProfile.avatar} alt={userStore.activeProfile.name} />
                    <div class="online-status"></div>
                  </div>
                  <button class="btn secondary sm" onclick={() => (activeTab = "profiles")}>
                    Edit Profiles
                  </button>
                </div>

                <div class="account-info-box">
                  <div class="user-display-name">{userStore.activeProfile.name}</div>
                  <div class="user-subtext">Luci Anime Enthusiast • #{userStore.activeProfile.id.slice(-4)}</div>
                </div>

                <div class="account-fields-list">
                  <div class="field-item">
                    <div>
                      <div class="field-label">Display Name</div>
                      <div class="field-value">{userStore.activeProfile.name}</div>
                    </div>
                    <div class="field-action">
                      <input class="inline-input" bind:value={usernameInput} placeholder="Enter name" />
                      <button class="btn secondary sm" onclick={saveProfileName}>Save</button>
                    </div>
                  </div>

                  <div class="field-item">
                    <div>
                      <div class="field-label">Account Type</div>
                      <div class="field-value">Local Offline Account (Zero telemetry, stored securely on your client)</div>
                    </div>
                    <span class="field-tag">LOCAL</span>
                  </div>

                  <div class="field-item">
                    <div>
                      <div class="field-label">Storage Engine</div>
                      <div class="field-value">Svelte 5 Runes & Local Browser Persistence</div>
                    </div>
                    <span class="field-tag verified">VERIFIED</span>
                  </div>
                </div>
              </div>
            </div>
          </div>

        <!-- TAB 2: PROFILES -->
        {:else if activeTab === "profiles"}
          <div class="tab-pane">
            <div class="pane-header-row">
              <h2 class="pane-title">Manage Profiles</h2>
              <button
                class="btn primary sm"
                onclick={() => {
                  userStore.showProfileModal = true;
                  closeSettings();
                }}
              >
                + Add / Switch Profile
              </button>
            </div>
            <p class="pane-desc">
              Profiles keep your watchlist, episode progress, and video history separate for everyone using this client.
            </p>

            <div class="profiles-list-card">
              {#each userStore.profiles as prof (prof.id)}
                {@const isCurrent = prof.id === userStore.activeProfile.id}
                <div class="profile-row" class:current={isCurrent}>
                  <div class="prof-left">
                    <img src={prof.avatar} alt={prof.name} class="prof-img" />
                    <div>
                      <div class="prof-name">
                        {prof.name}
                        {#if isCurrent}
                          <span class="active-tag">CURRENT</span>
                        {/if}
                        {#if prof.isKid}
                          <span class="kid-tag">KIDS</span>
                        {/if}
                      </div>
                      <div class="prof-id">ID: {prof.id} • Created {prof.createdAt}</div>
                    </div>
                  </div>

                  <div class="prof-actions">
                    {#if !isCurrent}
                      <button
                        class="btn secondary sm"
                        onclick={() => {
                          userStore.setActiveProfile(prof);
                          showNotice(`Switched to ${prof.name}`);
                        }}
                      >
                        Switch
                      </button>
                    {/if}
                    {#if userStore.profiles.length > 1 && !isCurrent}
                      <button
                        class="btn danger sm"
                        onclick={() => {
                          userStore.deleteProfile(prof.id);
                          showNotice("Profile removed");
                        }}
                      >
                        Delete
                      </button>
                    {/if}
                  </div>
                </div>
              {/each}
            </div>
          </div>

        <!-- TAB 3: APPEARANCE (COMPREHENSIVE DEEP CUSTOMIZATION) -->
        {:else if activeTab === "appearance"}
          <div class="tab-pane">
            <div class="pane-header-row">
              <div>
                <h2 class="pane-title">Appearance & Theme Customization</h2>
                <p class="pane-desc">
                  Customize colors, fonts, corner radius, and design styles. Changes are applied immediately and saved.
                </p>
              </div>
              <button class="btn secondary sm" onclick={() => (showCreatePreset = !showCreatePreset)}>
                {showCreatePreset ? "Cancel" : "+ Save as Custom Preset"}
              </button>
            </div>

            <!-- Create Preset Form -->
            {#if showCreatePreset}
              <div class="preset-create-card">
                <div class="preset-form-title">Save Current Configuration as Preset</div>
                <div class="preset-form-row">
                  <input
                    type="text"
                    bind:value={newPresetName}
                    placeholder="e.g., My Sharp Cyber Dark"
                    class="preset-input"
                  />
                  <button class="btn primary sm" onclick={handleSavePreset}>Save Preset</button>
                </div>
              </div>
            {/if}

            <!-- 1. Built-in & Custom Presets Section -->
            <div class="settings-group">
              <div class="group-title">Theme Presets</div>
              <div class="presets-grid">
                {#each BUILT_IN_PRESETS as p}
                  <button
                    class="preset-card"
                    class:active={currentTheme === p.theme && accentColor.toLowerCase() === p.accentColor.toLowerCase()}
                    onclick={() => applyPreset(p)}
                  >
                    <div class="preset-color-dot" style="background: {p.accentColor}"></div>
                    <div class="preset-info">
                      <div class="preset-name">{p.name}</div>
                      <div class="preset-details">{p.theme} • {p.radius}px • {p.fontFamily}</div>
                    </div>
                  </button>
                {/each}

                {#each userStore.customPresets as cp (cp.id)}
                  <div class="preset-card custom-preset-card">
                    <button class="preset-click-area" onclick={() => applyPreset(cp)}>
                      <div class="preset-color-dot" style="background: {cp.accentColor}"></div>
                      <div class="preset-info">
                        <div class="preset-name">{cp.name} <span class="custom-badge">CUSTOM</span></div>
                        <div class="preset-details">{cp.theme} • {cp.radius}px • {cp.fontFamily}</div>
                      </div>
                    </button>
                    <button
                      class="delete-preset-btn"
                      title="Delete custom preset"
                      onclick={() => userStore.deleteCustomPreset(cp.id)}
                    >
                      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" width="14" height="14" stroke-linecap="round"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
                    </button>
                  </div>
                {/each}
              </div>
            </div>

            <!-- 2. Application Theme / Background -->
            <div class="settings-group">
              <div class="group-title">Application Theme & Canvas Background</div>
              <div class="radio-cards-grid">
                <button
                  class="radio-card"
                  class:active={currentTheme === "dark"}
                  onclick={() => setAppTheme("dark")}
                >
                  <div class="theme-preview-box dark-box">
                    <div class="preview-bar"></div>
                    <div class="preview-dot" style="background: {accentColor}"></div>
                  </div>
                  <span class="card-label">Dark Theme</span>
                  <span class="card-sub">High contrast dark canvas (#0c0d10)</span>
                </button>

                <button
                  class="radio-card"
                  class:active={currentTheme === "oled"}
                  onclick={() => setAppTheme("oled")}
                >
                  <div class="theme-preview-box oled-box">
                    <div class="preview-bar oled-bar"></div>
                    <div class="preview-dot" style="background: {accentColor}"></div>
                  </div>
                  <span class="card-label">OLED Pitch Black</span>
                  <span class="card-sub">Pure black (#000000) infinite contrast</span>
                </button>

                <button
                  class="radio-card"
                  class:active={currentTheme === "charcoal"}
                  onclick={() => setAppTheme("charcoal")}
                >
                  <div class="theme-preview-box charcoal-box">
                    <div class="preview-bar charcoal-bar"></div>
                    <div class="preview-dot" style="background: {accentColor}"></div>
                  </div>
                  <span class="card-label">Charcoal Slate</span>
                  <span class="card-sub">Refined modern dark slate (#12161c)</span>
                </button>

                <button
                  class="radio-card"
                  class:active={currentTheme === "navy"}
                  onclick={() => setAppTheme("navy")}
                >
                  <div class="theme-preview-box navy-box">
                    <div class="preview-bar navy-bar"></div>
                    <div class="preview-dot" style="background: {accentColor}"></div>
                  </div>
                  <span class="card-label">Midnight Navy</span>
                  <span class="card-sub">Deep nautical blue (#0b101b)</span>
                </button>

                <button
                  class="radio-card"
                  class:active={currentTheme === "cyber"}
                  onclick={() => setAppTheme("cyber")}
                >
                  <div class="theme-preview-box cyber-box">
                    <div class="preview-bar cyber-bar"></div>
                    <div class="preview-dot" style="background: {accentColor}"></div>
                  </div>
                  <span class="card-label">Cyberpunk Matrix</span>
                  <span class="card-sub">High-tech cyan-infused dark (#07090e)</span>
                </button>

                <button
                  class="radio-card"
                  class:active={currentTheme === "light"}
                  onclick={() => setAppTheme("light")}
                >
                  <div class="theme-preview-box light-box">
                    <div class="preview-bar light-bar"></div>
                    <div class="preview-dot" style="background: {accentColor}"></div>
                  </div>
                  <span class="card-label">Light (White) Theme</span>
                  <span class="card-sub">Crisp paper white canvas (#f5f6fa)</span>
                </button>
              </div>
            </div>

            <!-- 3. Accent Colors with Custom Color Picker -->
            <div class="settings-group">
              <div class="group-title">Accent Color</div>
              <div class="color-picker-grid">
                {#each [
                  { label: "Luci Orange", hex: "#a855f7" },
                  { label: "Electric Cyan", hex: "#00d2ff" },
                  { label: "Emerald Jade", hex: "#10b981" },
                  { label: "Neon Violet", hex: "#8b5cf6" },
                  { label: "Crimson Red", hex: "#ef4444" },
                  { label: "Sunset Amber", hex: "#f59e0b" },
                  { label: "Sakura Pink", hex: "#ec4899" },
                  { label: "Royal Blue", hex: "#3b82f6" },
                ] as col}
                  <button
                    class="color-btn"
                    class:active={accentColor.toLowerCase() === col.hex.toLowerCase()}
                    onclick={() => setAccentColor(col.hex)}
                  >
                    <span class="color-swatch" style="background: {col.hex}"></span>
                    <span class="color-name">{col.label}</span>
                  </button>
                {/each}
              </div>

              <!-- Custom Color Input -->
              <div class="custom-color-row">
                <span class="custom-color-label">Custom Hex Accent:</span>
                <input
                  type="color"
                  value={accentColor}
                  oninput={(e) => setAccentColor((e.currentTarget as HTMLInputElement).value)}
                  class="color-input-native"
                />
                <input
                  type="text"
                  bind:value={accentColor}
                  onchange={() => setAccentColor(accentColor)}
                  class="color-hex-text"
                  placeholder="#a855f7"
                />
              </div>
            </div>

            <!-- 4. Corner Radius (Sharp Square to Smooth Rounded) -->
            <div class="settings-group">
              <div class="group-title">Corner Radius & Shape</div>
              <p class="group-subtext">Control edge rounding across buttons, cards, tags, and dialogs.</p>
              <div class="radius-selector-grid">
                {#each [
                  { label: "Sharp / Square", val: "0", desc: "0px • Hard rectangular edges" },
                  { label: "Minimal", val: "2", desc: "2px • Subtle micro bevel" },
                  { label: "Subtle", val: "4", desc: "4px • Modern refined corner" },
                  { label: "Standard", val: "6", desc: "6px • Balanced geometry" },
                  { label: "Smooth", val: "8", desc: "8px • Soft curve" },
                  { label: "Rounded", val: "12", desc: "12px • Gentle curvature" },
                  { label: "Capsule", val: "16", desc: "16px • Maximum rounded" },
                ] as rad}
                  <button
                    class="radius-option-card"
                    class:active={radius === rad.val}
                    onclick={() => setCornerRadius(rad.val)}
                  >
                    <div class="radius-sample-box" style="border-radius: {rad.val}px;"></div>
                    <div class="radius-label">{rad.label}</div>
                    <div class="radius-desc">{rad.desc}</div>
                  </button>
                {/each}
              </div>
            </div>

            <!-- 5. Typography & Font Family -->
            <div class="settings-group">
              <div class="group-title">Typography & Font Family</div>
              <div class="fonts-grid">
                {#each [
                  { name: "Overpass", desc: "Luci Display Sans", sample: "Overpass Sans" },
                  { name: "Inter", desc: "Modern UI Neutral", sample: "Inter Clean" },
                  { name: "Poppins", desc: "Geometric Display", sample: "Poppins Pro" },
                  { name: "Plus Jakarta Sans", desc: "Refined Contemporary", sample: "Plus Jakarta" },
                  { name: "Outfit", desc: "Futuristic Clean", sample: "Outfit Neo" },
                  { name: "Roboto Mono", desc: "Technical Monospace", sample: "Roboto Mono" },
                  { name: "Fira Code", desc: "Code Monospace", sample: "Fira Code" },
                  { name: "System Sans", desc: "Native OS System", sample: "System Font" },
                ] as f}
                  <button
                    class="font-option-card"
                    class:active={fontFamily === f.name}
                    onclick={() => setFont(f.name)}
                  >
                    <div class="font-sample" style="font-family: '{f.name}', sans-serif;">Aa</div>
                    <div class="font-title">{f.name}</div>
                    <div class="font-desc">{f.desc}</div>
                  </button>
                {/each}
              </div>
            </div>

            <!-- 6. Card Style & Interface Density -->
            <div class="settings-group">
              <div class="group-title">Card Style & Layout Density</div>
              <div class="card-style-grid">
                <button
                  class="card-opt-btn"
                  class:active={cardStyle === "bordered"}
                  onclick={() => setCardStyle("bordered")}
                >
                  <span class="card-opt-title">Bordered (Crisp Outline)</span>
                  <span class="card-opt-sub">Clean 1px borders with subtle hover highlight</span>
                </button>
                <button
                  class="card-opt-btn"
                  class:active={cardStyle === "flat"}
                  onclick={() => setCardStyle("flat")}
                >
                  <span class="card-opt-title">Flat Minimal</span>
                  <span class="card-opt-sub">Solid tone surfaces without border emphasis</span>
                </button>
                <button
                  class="card-opt-btn"
                  class:active={cardStyle === "elevated"}
                  onclick={() => setCardStyle("elevated")}
                >
                  <span class="card-opt-title">Elevated Shadow</span>
                  <span class="card-opt-sub">Gentle drop shadows for depth</span>
                </button>
                <button
                  class="card-opt-btn"
                  class:active={cardStyle === "glass"}
                  onclick={() => setCardStyle("glass")}
                >
                  <span class="card-opt-title">Frosted Glass</span>
                  <span class="card-opt-sub">Backdrop blur and semi-transparency</span>
                </button>
              </div>

              <div class="group-title" style="margin-top: 24px;">Layout Spacing Density</div>
              <div class="card-style-grid">
                <button
                  class="card-opt-btn"
                  class:active={density === "compact"}
                  onclick={() => setDensity("compact")}
                >
                  <span class="card-opt-title">Compact</span>
                  <span class="card-opt-sub">Tighter grids and smaller padding</span>
                </button>
                <button
                  class="card-opt-btn"
                  class:active={density === "comfortable"}
                  onclick={() => setDensity("comfortable")}
                >
                  <span class="card-opt-title">Comfortable</span>
                  <span class="card-opt-sub">Standard balanced spacing & sizing</span>
                </button>
                <button
                  class="card-opt-btn"
                  class:active={density === "spacious"}
                  onclick={() => setDensity("spacious")}
                >
                  <span class="card-opt-title">Spacious</span>
                  <span class="card-opt-sub">Larger visual breathing room</span>
                </button>
              </div>
            </div>
          </div>

        <!-- TAB 4: PLAYBACK -->
        {:else if activeTab === "playback"}
          <div class="tab-pane">
            <h2 class="pane-title">Video & Playback Settings</h2>
            <p class="pane-desc">Configure default resolution, autoplay triggers, and opening/ending skip preferences.</p>

            <div class="settings-group">
              <div class="group-title">Default Video Quality</div>
              <div class="quality-grid">
                {#each ["auto", "1080p", "720p", "4k"] as q}
                  <button
                    class="quality-card"
                    class:active={videoQuality === q}
                    onclick={() => (videoQuality = q as any)}
                  >
                    <span class="q-title">{q.toUpperCase()}</span>
                    <span class="q-sub">{q === "auto" ? "Dynamic bitrate selection" : `Locked to ${q}`}</span>
                  </button>
                {/each}
              </div>
            </div>

            <div class="settings-group">
              <div class="group-title">Playback Automation</div>
              <div class="toggle-list">
                <label class="toggle-row">
                  <div class="toggle-text">
                    <span class="toggle-name">Autoplay Next Episode</span>
                    <span class="toggle-desc">Automatically advance to the subsequent episode when the current one finishes.</span>
                  </div>
                  <input type="checkbox" bind:checked={autoplay} class="toggle-switch" />
                </label>

                <label class="toggle-row">
                  <div class="toggle-text">
                    <span class="toggle-name">Auto Skip Opening & Ending Themes</span>
                    <span class="toggle-desc">Automatically fast-forward verified anime intros.</span>
                  </div>
                  <input type="checkbox" bind:checked={autoSkipIntro} class="toggle-switch" />
                </label>
              </div>
            </div>
          </div>

        <!-- TAB 5: AUDIO & SUBTITLES -->
        {:else if activeTab === "subtitles"}
          <div class="tab-pane">
            <h2 class="pane-title">Audio & Subtitles</h2>
            <p class="pane-desc">Customize default spoken language, subtitles formatting, and rendering colors.</p>

            <div class="settings-group">
              <div class="group-title">Preferred Audio Language</div>
              <div class="lang-selector-row">
                {#each [
                  { code: "ja", label: "Japanese (Original)" },
                  { code: "en", label: "English (Dub)" },
                  { code: "es", label: "Spanish (Dub)" }
                ] as lang}
                  <button
                    class="lang-pill"
                    class:active={audioLanguage === lang.code}
                    onclick={() => (audioLanguage = lang.code as any)}
                  >
                    {lang.label}
                  </button>
                {/each}
              </div>
            </div>

            <div class="settings-group">
              <div class="group-title">Subtitle Styling</div>
              <div class="subtitle-customizer-grid">
                <div>
                  <label class="input-label">Subtitle Color</label>
                  <div class="color-options-row">
                    {#each ["#ffffff", "#ffe600", "#00ffff"] as c}
                      <button
                        class="sub-col-dot"
                        class:active={subtitleColor === c}
                        style="background: {c}"
                        onclick={() => (subtitleColor = c as any)}
                      ></button>
                    {/each}
                  </div>
                </div>

                <div>
                  <label class="input-label">Subtitle Size</label>
                  <select bind:value={subtitleSize} class="settings-select">
                    <option value="small">Small (18px)</option>
                    <option value="medium">Medium (24px)</option>
                    <option value="large">Large (30px)</option>
                  </select>
                </div>
              </div>
            </div>
          </div>

        <!-- TAB 6: NOTIFICATIONS -->
        {:else if activeTab === "notifications"}
          <div class="tab-pane">
            <h2 class="pane-title">Airing Alerts & Notifications</h2>
            <p class="pane-desc">Receive instant notifications when new weekly episodes of tracked anime air.</p>

            <div class="toggle-list">
              <label class="toggle-row">
                <div class="toggle-text">
                  <span class="toggle-name">Weekly Airing Broadcast Alerts</span>
                  <span class="toggle-desc">Show badge notifications when followed anime broadcast new episodes.</span>
                </div>
                <input type="checkbox" bind:checked={airingAlerts} class="toggle-switch" />
              </label>
            </div>
          </div>

        <!-- TAB 7: KEYBINDS -->
        {:else if activeTab === "keybinds"}
          <div class="tab-pane">
            <h2 class="pane-title">Keyboard Shortcuts</h2>
            <p class="pane-desc">Quick navigation and video playback hotkeys.</p>

            <div class="keybinds-table">
              <div class="keybind-row"><span class="key-name">Universal Search</span><kbd>Ctrl + K</kbd></div>
              <div class="keybind-row"><span class="key-name">Play / Pause</span><kbd>Space / K</kbd></div>
              <div class="keybind-row"><span class="key-name">Seek Forward 10s</span><kbd>→</kbd></div>
              <div class="keybind-row"><span class="key-name">Seek Backward 10s</span><kbd>←</kbd></div>
              <div class="keybind-row"><span class="key-name">Toggle Fullscreen</span><kbd>F</kbd></div>
              <div class="keybind-row"><span class="key-name">Toggle Mute</span><kbd>M</kbd></div>
              <div class="keybind-row"><span class="key-name">Close Modal / Settings</span><kbd>ESC</kbd></div>
            </div>
          </div>

        <!-- TAB 8: DATA & STORAGE -->
        {:else if activeTab === "data"}
          <div class="tab-pane">
            <h2 class="pane-title">Data Management & Cache</h2>
            <p class="pane-desc">Export your progress, backup local library collections, and manage cache files.</p>

            <div class="data-actions-grid">
              <div class="data-card">
                <div class="data-card-title">Export Watch History</div>
                <p class="data-card-desc">Download a JSON file containing all watched anime, episode timestamps, and progress.</p>
                <button class="btn secondary sm" onclick={exportWatchHistory}>Export History</button>
              </div>

              <div class="data-card">
                <div class="data-card-title">Flush Application Cache</div>
                <p class="data-card-desc">Clear cached anime metadata, banner images, and catalog indexes.</p>
                <button class="btn secondary sm" onclick={clearLocalCache}>Flush Cache</button>
              </div>
            </div>
          </div>

        <!-- TAB 9: ABOUT -->
        {:else if activeTab === "about"}
          <div class="tab-pane">
            <div class="about-hero-box">
              <div class="about-logo-row">
                <span class="about-logo-text">Luci</span>
                <span class="about-ver-tag">v2.5.0 Production Engine</span>
              </div>
              <p class="about-lead">
                The ultimate anime streaming client and personal library manager, powered by Svelte 5 runes, responsive layout design, and zero telemetry.
              </p>
            </div>

            <div class="about-specs-grid">
              <div class="spec-card">
                <span class="spec-label">Framework</span>
                <span class="spec-value">Svelte 5 + TypeScript</span>
              </div>
              <div class="spec-card">
                <span class="spec-label">Styling Engine</span>
                <span class="spec-value">Custom CSS Variables + Dynamic Themes</span>
              </div>
              <div class="spec-card">
                <span class="spec-label">Data Privacy</span>
                <span class="spec-value">100% Client-Side Local Storage</span>
              </div>
              <div class="spec-card">
                <span class="spec-label">Catalog Source</span>
                <span class="spec-value">AniList GraphQL + Crunchyroll Feeds</span>
              </div>
            </div>
          </div>
        {/if}
      </div>

      <!-- Floating Unsaved Changes / Save Notification Bar -->
      {#if hasUnsavedChanges}
        <div class="floating-save-bar">
          <div class="save-bar-inner">
            <div class="save-bar-text">
              <span class="save-bar-alert">Careful — you have unsaved changes!</span>
            </div>
            <div class="save-bar-actions">
              <button class="btn ghost sm" onclick={resetAllSettings}>Reset</button>
              <button class="btn primary sm save-btn" onclick={saveAllSettings}>Save Changes</button>
            </div>
          </div>
        </div>
      {/if}
    </main>
  </div>
</div>

<style>
  .discord-settings-backdrop {
    position: fixed;
    inset: 0;
    z-index: 2000;
    background: rgba(0, 0, 0, 0.85);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 24px;
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

  .discord-settings-card {
    width: 100%;
    max-width: 1180px;
    height: 88vh;
    max-height: 820px;
    background: var(--surface, #141519);
    border: 1px solid var(--border, #282a33);
    border-radius: var(--radius, 6px);
    display: flex;
    overflow: hidden;
    box-shadow: 0 24px 60px rgba(0, 0, 0, 0.95);
    position: relative;
  }

  /* Left Sidebar */
  .discord-sidebar {
    width: 260px;
    background: var(--surface-2, #101115);
    border-right: 1px solid var(--border, #1c1d25);
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
  }

  .sidebar-scroller {
    flex: 1;
    overflow-y: auto;
    padding: 24px 12px 24px 18px;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .nav-section-title {
    font-size: 11px;
    font-weight: 800;
    color: var(--text-faint, #6f717a);
    text-transform: uppercase;
    letter-spacing: 0.06em;
    padding: 10px 10px 4px;
  }

  .nav-tab {
    display: flex;
    align-items: center;
    gap: 10px;
    width: 100%;
    padding: 8px 12px;
    background: transparent;
    border: none;
    border-radius: var(--radius-sm, 4px);
    color: var(--text-dim, #92939c);
    font-size: 13.5px;
    font-weight: 700;
    text-align: left;
    cursor: pointer;
    transition: all 0.12s ease;
    font-family: inherit;
  }

  .nav-tab:hover {
    background: var(--surface-3, #191a21);
    color: var(--text, #ffffff);
  }

  .nav-tab.active {
    background: var(--surface-3, #23252f);
    color: var(--text, #ffffff);
    border-left: 3px solid var(--accent, #a855f7);
  }

  .nav-tab.logout {
    color: #ef4444;
  }

  .nav-tab.logout:hover {
    background: rgba(239, 68, 68, 0.12);
    color: #f87171;
  }

  .tab-icon {
    font-size: 14px;
  }

  .active-badge,
  .unread-pill {
    margin-left: auto;
    font-size: 10px;
    font-weight: 900;
    padding: 1px 6px;
    border-radius: 10px;
  }

  .active-badge {
    background: #282a36;
    color: #a0a0a8;
  }

  .unread-pill {
    background: var(--accent, #a855f7);
    color: #000000;
  }

  .nav-separator {
    height: 1px;
    background: var(--border-soft, #1c1d25);
    margin: 8px 6px;
  }

  /* Right Content Pane */
  .discord-content {
    position: relative;
    flex: 1;
    display: flex;
    flex-direction: column;
    background: var(--surface, #141519);
    overflow: hidden;
  }

  .content-scroll {
    flex: 1;
    overflow-y: auto;
    padding: 32px 40px 64px;
  }

  /* Discord ESC Button (Perfect 1:1 circle, no oval distortion) */
  .discord-close-col {
    position: absolute;
    top: 24px;
    right: 24px;
    z-index: 20;
  }

  .discord-esc-btn {
    display: flex;
    flex-direction: column;
    align-items: center;
    background: transparent;
    border: none;
    cursor: pointer;
    gap: 4px;
    padding: 0;
  }

  .esc-circle {
    width: 36px;
    height: 36px;
    aspect-ratio: 1 / 1;
    border-radius: 50%;
    border: 2px solid #555762;
    color: #a0a0a8;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 14px;
    font-weight: 800;
    line-height: 1;
    transition: all 0.12s ease;
  }

  .discord-esc-btn:hover .esc-circle {
    border-color: #ffffff;
    color: #ffffff;
    background: rgba(255, 255, 255, 0.12);
  }

  .esc-label {
    font-size: 10.5px;
    font-weight: 800;
    color: #72737c;
    letter-spacing: 0.05em;
  }

  .discord-esc-btn:hover .esc-label {
    color: #ffffff;
  }

  /* Status Banner */
  .status-banner {
    background: rgba(46, 204, 113, 0.15);
    border: 1px solid rgba(46, 204, 113, 0.4);
    color: #2ecc71;
    padding: 10px 16px;
    border-radius: var(--radius-sm, 4px);
    font-size: 13.5px;
    font-weight: 700;
    margin-bottom: 24px;
  }

  /* Pane Typography */
  .pane-title {
    font-size: 20px;
    font-weight: 900;
    color: var(--text, #ffffff);
    margin: 0 0 6px;
    letter-spacing: -0.02em;
  }

  .pane-desc {
    font-size: 13.5px;
    color: var(--text-dim, #8e909c);
    margin: 0 0 28px;
    line-height: 1.5;
  }

  .pane-header-row {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: 16px;
  }

  .settings-group {
    margin-bottom: 32px;
  }

  .group-title {
    font-size: 12px;
    font-weight: 800;
    color: var(--text-dim, #a0a2b0);
    text-transform: uppercase;
    letter-spacing: 0.06em;
    margin-bottom: 12px;
  }

  .group-subtext {
    font-size: 12.5px;
    color: var(--text-dim, #7e808e);
    margin: -6px 0 12px;
  }

  /* Preset creation card */
  .preset-create-card {
    background: var(--surface-2, #181920);
    border: 1px solid var(--border, #282a36);
    border-radius: var(--radius, 6px);
    padding: 16px;
    margin-bottom: 24px;
  }

  .preset-form-title {
    font-size: 13px;
    font-weight: 800;
    color: var(--text, #ffffff);
    margin-bottom: 10px;
  }

  .preset-form-row {
    display: flex;
    gap: 10px;
  }

  .preset-input {
    flex: 1;
    background: var(--surface-3, #22242e);
    border: 1px solid var(--border, #2e313d);
    color: var(--text, #ffffff);
    padding: 8px 12px;
    border-radius: var(--radius-sm, 4px);
    font-size: 13.5px;
    outline: none;
  }

  .preset-input:focus {
    border-color: var(--accent, #a855f7);
  }

  /* Presets Grid */
  .presets-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
    gap: 12px;
  }

  .preset-card {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 14px;
    background: var(--surface-2, #181a22);
    border: 1px solid var(--border, #262833);
    border-radius: var(--radius, 4px);
    cursor: pointer;
    text-align: left;
    transition: all 0.12s ease;
    font-family: inherit;
    color: inherit;
  }

  .preset-card:hover {
    background: var(--surface-3, #20232e);
    border-color: var(--accent, #a855f7);
  }

  .preset-card.active {
    background: var(--surface-3, #222634);
    border-color: var(--accent, #a855f7);
    box-shadow: 0 0 0 1px var(--accent, #a855f7);
  }

  .custom-preset-card {
    padding: 0;
    display: flex;
    align-items: center;
  }

  .preset-click-area {
    flex: 1;
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 14px;
    background: transparent;
    border: none;
    text-align: left;
    cursor: pointer;
    color: inherit;
    font-family: inherit;
  }

  .delete-preset-btn {
    background: transparent;
    border: none;
    color: #ef4444;
    padding: 12px;
    cursor: pointer;
    font-size: 12px;
  }

  .delete-preset-btn:hover {
    color: #ff7777;
    background: rgba(239, 68, 68, 0.1);
  }

  .preset-color-dot {
    width: 14px;
    height: 14px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .preset-name {
    font-size: 13.5px;
    font-weight: 800;
    color: var(--text, #ffffff);
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .custom-badge {
    font-size: 9px;
    background: var(--accent, #a855f7);
    color: #000000;
    font-weight: 900;
    padding: 1px 4px;
    border-radius: 2px;
  }

  .preset-details {
    font-size: 11px;
    color: var(--text-dim, #7e808d);
    text-transform: capitalize;
  }

  /* Themes Radio Cards Grid */
  .radio-cards-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 14px;
  }

  .radio-card {
    background: var(--surface-2, #181920);
    border: 1px solid var(--border, #282a33);
    border-radius: var(--radius, 6px);
    padding: 14px;
    cursor: pointer;
    text-align: left;
    display: flex;
    flex-direction: column;
    gap: 8px;
    transition: all 0.12s ease;
    font-family: inherit;
  }

  .radio-card:hover {
    background: var(--surface-3, #21232d);
    border-color: var(--accent, #a855f7);
  }

  .radio-card.active {
    background: var(--surface-3, #222530);
    border-color: var(--accent, #a855f7);
    box-shadow: 0 0 0 1px var(--accent, #a855f7);
  }

  .theme-preview-box {
    width: 100%;
    height: 48px;
    border-radius: var(--radius-sm, 3px);
    position: relative;
    padding: 8px;
    display: flex;
    align-items: flex-end;
    justify-content: flex-end;
  }

  .dark-box {
    background: #0c0d10;
    border: 1px solid #22242e;
  }

  .oled-box {
    background: #000000;
    border: 1px solid #1a1a1a;
  }

  .charcoal-box {
    background: #12161c;
    border: 1px solid #222c38;
  }

  .navy-box {
    background: #0b101b;
    border: 1px solid #1c263c;
  }

  .cyber-box {
    background: #07090e;
    border: 1px solid #1a2336;
  }

  .light-box {
    background: #f5f6fa;
    border: 1px solid #d5d9e4;
  }

  .preview-bar {
    position: absolute;
    top: 8px;
    left: 8px;
    right: 32px;
    height: 6px;
    background: rgba(255, 255, 255, 0.2);
    border-radius: 2px;
  }

  .preview-bar.light-bar {
    background: rgba(0, 0, 0, 0.2);
  }

  .preview-dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
  }

  .card-label {
    font-size: 13.5px;
    font-weight: 800;
    color: var(--text, #ffffff);
  }

  .card-sub {
    font-size: 11.5px;
    color: var(--text-dim, #7e808e);
    line-height: 1.35;
  }

  /* Color Picker Grid */
  .color-picker-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(170px, 1fr));
    gap: 10px;
    margin-bottom: 14px;
  }

  .color-btn {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 14px;
    background: var(--surface-2, #181920);
    border: 1px solid var(--border, #282a33);
    border-radius: var(--radius-sm, 4px);
    cursor: pointer;
    font-family: inherit;
    transition: all 0.12s ease;
  }

  .color-btn:hover {
    background: var(--surface-3, #22242e);
    border-color: var(--accent, #a855f7);
  }

  .color-btn.active {
    background: var(--surface-3, #22242e);
    border-color: var(--accent, #a855f7);
    box-shadow: 0 0 0 1px var(--accent, #a855f7);
  }

  .color-swatch {
    width: 16px;
    height: 16px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .color-name {
    font-size: 13px;
    font-weight: 700;
    color: var(--text, #ffffff);
  }

  .custom-color-row {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 16px;
    background: var(--surface-2, #181920);
    border: 1px solid var(--border, #282a33);
    border-radius: var(--radius-sm, 4px);
  }

  .custom-color-label {
    font-size: 13px;
    font-weight: 700;
    color: var(--text, #ffffff);
  }

  .color-input-native {
    width: 32px;
    height: 32px;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    background: transparent;
  }

  .color-hex-text {
    width: 110px;
    background: var(--surface-3, #22242e);
    border: 1px solid var(--border, #2e313d);
    color: var(--text, #ffffff);
    padding: 6px 10px;
    border-radius: var(--radius-sm, 4px);
    font-size: 13px;
    font-family: monospace;
  }

  /* Corner Radius Selector Grid */
  .radius-selector-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(135px, 1fr));
    gap: 10px;
  }

  .radius-option-card {
    background: var(--surface-2, #181920);
    border: 1px solid var(--border, #282a33);
    border-radius: var(--radius, 4px);
    padding: 14px 10px;
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    gap: 6px;
    cursor: pointer;
    font-family: inherit;
    transition: all 0.12s ease;
  }

  .radius-option-card:hover {
    background: var(--surface-3, #22242e);
    border-color: var(--accent, #a855f7);
  }

  .radius-option-card.active {
    background: var(--surface-3, #22242e);
    border-color: var(--accent, #a855f7);
    box-shadow: 0 0 0 1px var(--accent, #a855f7);
  }

  .radius-sample-box {
    width: 38px;
    height: 38px;
    border: 2px solid var(--accent, #a855f7);
    background: rgba(168, 85, 247, 0.12);
    margin-bottom: 4px;
  }

  .radius-label {
    font-size: 12.5px;
    font-weight: 800;
    color: var(--text, #ffffff);
  }

  .radius-desc {
    font-size: 10.5px;
    color: var(--text-dim, #7e808e);
  }

  /* Fonts Grid */
  .fonts-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
    gap: 10px;
  }

  .font-option-card {
    background: var(--surface-2, #181920);
    border: 1px solid var(--border, #282a33);
    border-radius: var(--radius, 4px);
    padding: 14px 12px;
    text-align: left;
    cursor: pointer;
    font-family: inherit;
    transition: all 0.12s ease;
  }

  .font-option-card:hover {
    background: var(--surface-3, #22242e);
    border-color: var(--accent, #a855f7);
  }

  .font-option-card.active {
    background: var(--surface-3, #22242e);
    border-color: var(--accent, #a855f7);
    box-shadow: 0 0 0 1px var(--accent, #a855f7);
  }

  .font-sample {
    font-size: 24px;
    font-weight: 800;
    color: var(--accent, #a855f7);
    margin-bottom: 6px;
  }

  .font-title {
    font-size: 13.5px;
    font-weight: 800;
    color: var(--text, #ffffff);
  }

  .font-desc {
    font-size: 11px;
    color: var(--text-dim, #7e808e);
  }

  /* Card Style Grid */
  .card-style-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
    gap: 10px;
  }

  .card-opt-btn {
    background: var(--surface-2, #181920);
    border: 1px solid var(--border, #282a33);
    border-radius: var(--radius, 4px);
    padding: 14px;
    text-align: left;
    cursor: pointer;
    display: flex;
    flex-direction: column;
    gap: 4px;
    font-family: inherit;
    transition: all 0.12s ease;
  }

  .card-opt-btn:hover {
    background: var(--surface-3, #22242e);
    border-color: var(--accent, #a855f7);
  }

  .card-opt-btn.active {
    background: var(--surface-3, #22242e);
    border-color: var(--accent, #a855f7);
    box-shadow: 0 0 0 1px var(--accent, #a855f7);
  }

  .card-opt-title {
    font-size: 13.5px;
    font-weight: 800;
    color: var(--text, #ffffff);
  }

  .card-opt-sub {
    font-size: 11.5px;
    color: var(--text-dim, #7e808e);
  }

  /* Account Card (Discord Style) */
  .account-card {
    background: var(--surface-2, #181920);
    border: 1px solid var(--border, #282a33);
    border-radius: var(--radius, 6px);
    overflow: hidden;
  }

  .account-banner {
    height: 90px;
  }

  .account-card-body {
    padding: 0 24px 24px;
    position: relative;
  }

  .avatar-row {
    display: flex;
    justify-content: space-between;
    align-items: flex-end;
    margin-top: -40px;
    margin-bottom: 16px;
  }

  .account-avatar-box {
    position: relative;
    width: 80px;
    height: 80px;
    border-radius: var(--radius-sm, 6px);
    border: 4px solid var(--surface-2, #181920);
    overflow: hidden;
    background: #090a0d;
  }

  .account-avatar-box img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .online-status {
    position: absolute;
    bottom: 2px;
    right: 2px;
    width: 12px;
    height: 12px;
    border-radius: 50%;
    background: #10b981;
    border: 2px solid var(--surface-2, #181920);
  }

  .account-info-box {
    margin-bottom: 20px;
  }

  .user-display-name {
    font-size: 18px;
    font-weight: 800;
    color: var(--text, #ffffff);
  }

  .user-subtext {
    font-size: 12.5px;
    color: var(--text-dim, #7e808e);
  }

  .account-fields-list {
    display: flex;
    flex-direction: column;
    gap: 12px;
    background: var(--surface-3, #1e2029);
    padding: 16px;
    border-radius: var(--radius-sm, 4px);
  }

  .field-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-bottom: 10px;
    border-bottom: 1px solid var(--border-soft, #262833);
  }

  .field-item:last-child {
    border-bottom: none;
    padding-bottom: 0;
  }

  .field-label {
    font-size: 11px;
    font-weight: 800;
    color: var(--text-dim, #7e808e);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    margin-bottom: 2px;
  }

  .field-value {
    font-size: 13.5px;
    font-weight: 700;
    color: var(--text, #ffffff);
  }

  .field-action {
    display: flex;
    gap: 8px;
  }

  .inline-input {
    background: var(--surface-2, #15161b);
    border: 1px solid var(--border, #2e303d);
    color: var(--text, #ffffff);
    padding: 6px 10px;
    border-radius: var(--radius-sm, 4px);
    font-size: 13px;
    outline: none;
  }

  .field-tag {
    font-size: 10px;
    font-weight: 800;
    padding: 2px 8px;
    border-radius: 4px;
    background: var(--surface-2, #181920);
    color: var(--text-dim, #9094a6);
    border: 1px solid var(--border, #2b2d3a);
  }

  .field-tag.verified {
    background: rgba(16, 185, 129, 0.15);
    color: #10b981;
    border-color: rgba(16, 185, 129, 0.4);
  }

  /* Profiles List */
  .profiles-list-card {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .profile-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 14px 18px;
    background: var(--surface-2, #181920);
    border: 1px solid var(--border, #282a33);
    border-radius: var(--radius, 4px);
  }

  .profile-row.current {
    border-color: var(--accent, #a855f7);
    background: var(--surface-3, #1e212b);
  }

  .prof-left {
    display: flex;
    align-items: center;
    gap: 14px;
  }

  .prof-img {
    width: 44px;
    height: 44px;
    border-radius: var(--radius-sm, 4px);
    object-fit: cover;
  }

  .prof-name {
    font-size: 14.5px;
    font-weight: 800;
    color: var(--text, #ffffff);
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .active-tag {
    font-size: 9.5px;
    font-weight: 900;
    background: var(--accent, #a855f7);
    color: #000000;
    padding: 2px 6px;
    border-radius: 2px;
  }

  .kid-tag {
    font-size: 9.5px;
    font-weight: 900;
    background: #3b82f6;
    color: #ffffff;
    padding: 2px 6px;
    border-radius: 2px;
  }

  .prof-id {
    font-size: 11.5px;
    color: var(--text-dim, #7e808e);
  }

  .prof-actions {
    display: flex;
    gap: 8px;
  }

  /* Toggle Switches */
  .toggle-list {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .toggle-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 20px;
    background: var(--surface-2, #181920);
    border: 1px solid var(--border, #282a33);
    border-radius: var(--radius, 4px);
    cursor: pointer;
  }

  .toggle-text {
    display: flex;
    flex-direction: column;
    gap: 3px;
  }

  .toggle-name {
    font-size: 14px;
    font-weight: 800;
    color: var(--text, #ffffff);
  }

  .toggle-desc {
    font-size: 12.5px;
    color: var(--text-dim, #7e808e);
  }

  .toggle-switch {
    width: 20px;
    height: 20px;
    accent-color: var(--accent, #a855f7);
    cursor: pointer;
  }

  /* Keybinds Table */
  .keybinds-table {
    display: flex;
    flex-direction: column;
    background: var(--surface-2, #181920);
    border: 1px solid var(--border, #282a33);
    border-radius: var(--radius, 4px);
    overflow: hidden;
  }

  .keybind-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 18px;
    border-bottom: 1px solid var(--border-soft, #262833);
  }

  .keybind-row:last-child {
    border-bottom: none;
  }

  .key-name {
    font-size: 13.5px;
    font-weight: 700;
    color: var(--text, #ffffff);
  }

  kbd {
    background: var(--surface-3, #252834);
    border: 1px solid var(--border, #343746);
    padding: 4px 10px;
    border-radius: var(--radius-sm, 4px);
    font-size: 12px;
    font-weight: 800;
    color: var(--text, #ffffff);
    font-family: inherit;
  }

  /* Data Management Cards */
  .data-actions-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
    gap: 16px;
  }

  .data-card {
    background: var(--surface-2, #181920);
    border: 1px solid var(--border, #282a33);
    border-radius: var(--radius, 4px);
    padding: 20px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .data-card-title {
    font-size: 15px;
    font-weight: 800;
    color: var(--text, #ffffff);
  }

  .data-card-desc {
    font-size: 12.5px;
    color: var(--text-dim, #7e808e);
    line-height: 1.4;
    margin: 0 0 8px;
    flex: 1;
  }

  /* About Hero */
  .about-hero-box {
    background: var(--surface-2, #181920);
    border: 1px solid var(--border, #282a33);
    border-radius: var(--radius, 6px);
    padding: 24px;
    margin-bottom: 24px;
  }

  .about-logo-row {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 12px;
  }

  .about-logo-text {
    font-size: 28px;
    font-weight: 900;
    color: var(--accent, #a855f7);
    letter-spacing: -0.03em;
  }

  .about-ver-tag {
    font-size: 11px;
    font-weight: 800;
    padding: 3px 8px;
    background: var(--surface-3, #22242e);
    border: 1px solid var(--border, #2d303e);
    color: var(--text-dim, #a0a0aa);
    border-radius: var(--radius-sm, 3px);
  }

  .about-lead {
    font-size: 14px;
    line-height: 1.6;
    color: var(--text-dim, #8e909d);
    margin: 0;
  }

  .about-specs-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
    gap: 12px;
  }

  .spec-card {
    background: var(--surface-2, #181920);
    border: 1px solid var(--border, #282a33);
    border-radius: var(--radius, 4px);
    padding: 14px;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .spec-label {
    font-size: 11px;
    font-weight: 800;
    color: var(--text-dim, #7e808e);
    text-transform: uppercase;
  }

  .spec-value {
    font-size: 13.5px;
    font-weight: 800;
    color: var(--text, #ffffff);
  }

  /* Floating Save Changes Notice Bar (Discord Style) */
  .floating-save-bar {
    position: absolute;
    bottom: 20px;
    left: 40px;
    right: 40px;
    background: #111216;
    border: 1px solid #282a33;
    border-radius: var(--radius, 6px);
    box-shadow: 0 12px 36px rgba(0, 0, 0, 0.9);
    padding: 12px 20px;
    z-index: 100;
    animation: slideUp 0.18s ease-out;
  }

  @keyframes slideUp {
    from {
      transform: translateY(20px);
      opacity: 0;
    }
    to {
      transform: translateY(0);
      opacity: 1;
    }
  }

  .save-bar-inner {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 16px;
  }

  .save-bar-alert {
    font-size: 13.5px;
    font-weight: 800;
    color: #ffffff;
  }

  .save-bar-actions {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .save-btn {
    background: var(--accent, #a855f7) !important;
    color: #000000 !important;
  }

  /* Quality & Subtitle helper components */
  .quality-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(130px, 1fr));
    gap: 10px;
  }

  .quality-card {
    background: var(--surface-2, #181920);
    border: 1px solid var(--border, #282a33);
    border-radius: var(--radius, 4px);
    padding: 14px;
    text-align: center;
    cursor: pointer;
    font-family: inherit;
    transition: all 0.12s ease;
  }

  .quality-card:hover {
    background: var(--surface-3, #22242e);
    border-color: var(--accent, #a855f7);
  }

  .quality-card.active {
    background: var(--surface-3, #22242e);
    border-color: var(--accent, #a855f7);
    box-shadow: 0 0 0 1px var(--accent, #a855f7);
  }

  .q-title {
    font-size: 14px;
    font-weight: 900;
    color: var(--text, #ffffff);
    display: block;
  }

  .q-sub {
    font-size: 10.5px;
    color: var(--text-dim, #7e808e);
    display: block;
    margin-top: 2px;
  }

  .lang-selector-row {
    display: flex;
    gap: 10px;
    flex-wrap: wrap;
  }

  .lang-pill {
    padding: 8px 16px;
    background: var(--surface-2, #181920);
    border: 1px solid var(--border, #282a33);
    border-radius: var(--radius-sm, 4px);
    color: var(--text-dim, #9094a6);
    font-size: 13px;
    font-weight: 700;
    cursor: pointer;
    font-family: inherit;
    transition: all 0.12s ease;
  }

  .lang-pill:hover {
    background: var(--surface-3, #22242e);
    color: var(--text, #ffffff);
  }

  .lang-pill.active {
    background: var(--surface-3, #22242e);
    color: var(--text, #ffffff);
    border-color: var(--accent, #a855f7);
  }

  .subtitle-customizer-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 16px;
  }

  .input-label {
    display: block;
    font-size: 12px;
    font-weight: 700;
    color: var(--text-dim, #7e808e);
    margin-bottom: 6px;
  }

  .color-options-row {
    display: flex;
    gap: 10px;
  }

  .sub-col-dot {
    width: 28px;
    height: 28px;
    border-radius: 50%;
    border: 2px solid transparent;
    cursor: pointer;
  }

  .sub-col-dot.active {
    border-color: #ffffff;
    transform: scale(1.1);
  }

  .settings-select {
    width: 100%;
    background: var(--surface-2, #181920);
    border: 1px solid var(--border, #282a33);
    color: var(--text, #ffffff);
    padding: 8px 12px;
    border-radius: var(--radius-sm, 4px);
    font-size: 13px;
    font-family: inherit;
    outline: none;
  }
</style>
