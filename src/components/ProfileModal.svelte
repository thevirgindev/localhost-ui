<script lang="ts">
  import { userStore, type UserProfile } from "../lib/userStore.svelte";

  let isAdding = $state(false);
  let newName = $state("");
  let selectedAvatar = $state("https://images.unsplash.com/photo-1578632767115-351597cf2477?w=160&auto=format&fit=crop&q=80");
  let selectedColor = $state("#a855f7");
  let isKid = $state(false);

  const presetAvatars = [
    { url: "https://images.unsplash.com/photo-1578632767115-351597cf2477?w=160&auto=format&fit=crop&q=80", label: "Anime Hero" },
    { url: "https://images.unsplash.com/photo-1534447677768-be436bb09401?w=160&auto=format&fit=crop&q=80", label: "Scout" },
    { url: "https://images.unsplash.com/photo-1618005182384-a83a8bd57fbe?w=160&auto=format&fit=crop&q=80", label: "Sorcerer" },
    { url: "https://images.unsplash.com/photo-1607604276583-eef5d076aa5f?w=160&auto=format&fit=crop&q=80", label: "Pirate King" },
    { url: "https://images.unsplash.com/photo-1579783902614-a3fb3927b675?w=160&auto=format&fit=crop&q=80", label: "Demon Slayer" },
    { url: "https://images.unsplash.com/photo-1550684848-fac1c5b4e853?w=160&auto=format&fit=crop&q=80", label: "Cyber Ninja" },
  ];

  const presetColors = ["#a855f7", "#3b82f6", "#10b981", "#ec4899", "#eab308", "#64748b"];

  function closeModal() {
    userStore.showProfileModal = false;
    isAdding = false;
  }

  function selectProfile(p: UserProfile) {
    userStore.setActiveProfile(p);
    closeModal();
  }

  function handleCreateProfile() {
    if (!newName.trim()) return;
    userStore.addProfile(newName.trim(), selectedAvatar, selectedColor, isKid);
    newName = "";
    isAdding = false;
    closeModal();
  }
</script>

<div class="profile-backdrop" onclick={closeModal} role="button" tabindex="0" onkeydown={(e) => e.key === "Escape" && closeModal()}>
  <div
    class="profile-card"
    onclick={(e) => e.stopPropagation()}
    role="dialog"
    aria-label="Profile Selection"
    tabindex="-1"
    onkeydown={(e) => e.key === "Escape" && closeModal()}
  >
    <div class="profile-card-header">
      <div>
        <h2 class="card-title">{isAdding ? "Create Profile" : "Who's Watching?"}</h2>
        <p class="card-sub">
          {isAdding
            ? "Create a local profile to track your custom watch history, watchlist, and ratings."
            : "Select your active profile to continue watching where you left off."}
        </p>
      </div>
      <button class="close-btn" onclick={closeModal} title="Close"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" width="14" height="14" stroke-linecap="round"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg></button>
    </div>

    {#if !isAdding}
      <!-- Profile Grid -->
      <div class="profiles-grid">
        {#each userStore.profiles as profile (profile.id)}
          {@const isActive = userStore.activeProfile.id === profile.id}
          <div
            class="profile-item"
            class:active={isActive}
            onclick={() => selectProfile(profile)}
            role="button"
            tabindex="0"
            onkeydown={(e) => e.key === "Enter" && selectProfile(profile)}
          >
            <div class="avatar-box" style="border-color: {isActive ? profile.color : 'transparent'}">
              <img src={profile.avatar} alt={profile.name} />
              {#if isActive}
                <span class="active-badge" style="background: {profile.color}"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" width="10" height="10" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"/></svg> ACTIVE</span>
              {/if}
            </div>

            <span class="profile-name">{profile.name}</span>
            {#if profile.isKid}
              <span class="kid-pill">KIDS</span>
            {/if}

            {#if userStore.profiles.length > 1 && !isActive}
              <button
                class="del-btn"
                title="Delete profile"
                onclick={(e) => {
                  e.stopPropagation();
                  userStore.deleteProfile(profile.id);
                }}
              >
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" width="14" height="14" stroke-linecap="round"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
              </button>
            {/if}
          </div>
        {/each}

        <!-- Add Profile Button -->
        <button class="add-profile-item" onclick={() => (isAdding = true)}>
          <div class="add-icon-box">
            <span class="plus-sign">+</span>
          </div>
          <span class="profile-name">Add Profile</span>
        </button>
      </div>
    {:else}
      <!-- Add Profile Form -->
      <div class="add-form">
        <div class="form-group">
          <label class="form-label" for="profile-name-input">Profile Name</label>
          <input
            id="profile-name-input"
            class="form-input"
            bind:value={newName}
            placeholder="e.g. Otaku Sempai"
            maxlength="24"
          />
        </div>

        <div class="form-group">
          <span class="form-label">Choose Avatar</span>
          <div class="avatar-choices">
            {#each presetAvatars as av}
              <button
                class="avatar-choice"
                class:selected={selectedAvatar === av.url}
                onclick={() => (selectedAvatar = av.url)}
              >
                <img src={av.url} alt={av.label} />
              </button>
            {/each}
          </div>
        </div>

        <div class="form-group">
          <span class="form-label">Theme Accent</span>
          <div class="color-choices">
            {#each presetColors as col}
              <button
                class="color-choice"
                class:selected={selectedColor === col}
                style="background: {col}"
                onclick={() => (selectedColor = col)}
                aria-label="Color {col}"
              ></button>
            {/each}
          </div>
        </div>

        <div class="form-group checkbox-group">
          <label class="checkbox-label">
            <input type="checkbox" bind:checked={isKid} />
            <span>Kid profile (Family-friendly content only)</span>
          </label>
        </div>

        <div class="form-actions">
          <button class="btn primary" onclick={handleCreateProfile} disabled={!newName.trim()}>
            Save Profile
          </button>
          <button class="btn secondary" onclick={() => (isAdding = false)}>
            Cancel
          </button>
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  .profile-backdrop {
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

  .profile-card {
    width: 100%;
    max-width: 680px;
    background: #141519;
    border: 1px solid #282a32;
    border-radius: 6px;
    overflow: hidden;
    box-shadow: 0 24px 64px rgba(0, 0, 0, 0.9);
    padding: 28px 32px;
  }

  .profile-card-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 28px;
  }

  .card-title {
    font-size: 22px;
    font-weight: 800;
    color: #ffffff;
    margin: 0 0 6px;
    letter-spacing: -0.02em;
  }

  .card-sub {
    font-size: 13px;
    color: #8c8c92;
    margin: 0;
    line-height: 1.4;
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
    background: #23252e;
  }

  .profiles-grid {
    display: flex;
    gap: 24px;
    flex-wrap: wrap;
    align-items: center;
    justify-content: center;
    padding: 10px 0 20px;
  }

  .profile-item {
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 10px;
    cursor: pointer;
    width: 110px;
    transition: transform 0.15s ease;
  }

  .profile-item:hover {
    transform: translateY(-4px);
  }

  .avatar-box {
    position: relative;
    width: 90px;
    height: 90px;
    border-radius: 6px;
    overflow: hidden;
    border: 3px solid transparent;
    background: #1c1e25;
    transition: border-color 0.15s ease;
  }

  .profile-item:hover .avatar-box {
    border-color: var(--accent);
  }

  .avatar-box img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .active-badge {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    font-size: 9px;
    font-weight: 900;
    color: #000000;
    text-align: center;
    padding: 2px 0;
    letter-spacing: 0.05em;
  }

  .profile-name {
    font-size: 14px;
    font-weight: 700;
    color: #ffffff;
    text-align: center;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 100%;
  }

  .kid-pill {
    font-size: 9px;
    font-weight: 900;
    color: #10b981;
    background: rgba(16, 185, 129, 0.14);
    padding: 1px 6px;
    border-radius: 2px;
  }

  .del-btn {
    position: absolute;
    top: -6px;
    right: 6px;
    width: 20px;
    height: 20px;
    border-radius: 50%;
    background: #e11d48;
    color: #ffffff;
    border: none;
    font-size: 10px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    opacity: 0;
    transition: opacity 0.15s ease;
  }

  .profile-item:hover .del-btn {
    opacity: 1;
  }

  .add-profile-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 10px;
    cursor: pointer;
    width: 110px;
    background: transparent;
    border: none;
    transition: transform 0.15s ease;
  }

  .add-profile-item:hover {
    transform: translateY(-4px);
  }

  .add-icon-box {
    width: 90px;
    height: 90px;
    border-radius: 6px;
    border: 2px dashed #3a3d48;
    display: flex;
    align-items: center;
    justify-content: center;
    background: #191a21;
    transition: border-color 0.15s ease, background 0.15s ease;
  }

  .add-profile-item:hover .add-icon-box {
    border-color: var(--accent);
    background: color-mix(in srgb, var(--accent) 8%, transparent);
  }

  .plus-sign {
    font-size: 32px;
    color: #727278;
    font-weight: 300;
    line-height: 1;
  }

  .add-profile-item:hover .plus-sign {
    color: var(--accent);
  }

  /* Form */
  .add-form {
    display: flex;
    flex-direction: column;
    gap: 18px;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .form-label {
    font-size: 13px;
    font-weight: 700;
    color: #cfcfd4;
  }

  .form-input {
    background: #1d1f27;
    border: 1px solid #2e313c;
    border-radius: 4px;
    padding: 10px 14px;
    color: #ffffff;
    font-size: 14px;
    font-family: inherit;
    outline: none;
  }

  .form-input:focus {
    border-color: var(--accent);
  }

  .avatar-choices {
    display: flex;
    gap: 12px;
    flex-wrap: wrap;
  }

  .avatar-choice {
    width: 54px;
    height: 54px;
    border-radius: 6px;
    overflow: hidden;
    border: 2px solid transparent;
    cursor: pointer;
    background: #1a1c22;
    padding: 0;
    transition: border-color 0.12s ease;
  }

  .avatar-choice img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .avatar-choice.selected {
    border-color: var(--accent);
  }

  .color-choices {
    display: flex;
    gap: 10px;
  }

  .color-choice {
    width: 28px;
    height: 28px;
    border-radius: 50%;
    border: 2px solid transparent;
    cursor: pointer;
    transition: transform 0.12s ease;
  }

  .color-choice.selected {
    border-color: #ffffff;
    transform: scale(1.15);
  }

  .checkbox-label {
    display: flex;
    align-items: center;
    gap: 10px;
    font-size: 13px;
    color: #a0a0a5;
    cursor: pointer;
  }

  .form-actions {
    display: flex;
    gap: 12px;
    margin-top: 10px;
  }
</style>
