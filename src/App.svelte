<script lang="ts">
  import { router, type Route } from "./lib/router";
  import { userStore } from "./lib/userStore.svelte";
  import Navbar from "./components/Navbar.svelte";
  import Footer from "./components/Footer.svelte";
  import SearchModal from "./components/SearchModal.svelte";
  import ProfileModal from "./components/ProfileModal.svelte";
  import ChangelogModal from "./components/ChangelogModal.svelte";
  import ContextMenu from "./components/ContextMenu.svelte";
  import HomePage from "./pages/HomePage.svelte";
  import BrowsePage from "./pages/BrowsePage.svelte";
  import SeasonPage from "./pages/SeasonPage.svelte";
  import DetailsPage from "./pages/DetailsPage.svelte";
  import LibraryPage from "./pages/LibraryPage.svelte";
  import SettingsPage from "./pages/SettingsPage.svelte";
  import WatchPage from "./pages/WatchPage.svelte";

  let route: Route = $state(router.current());
  router.subscribe((r) => (route = r));

  const isPlayer = $derived(route.name === "watch");
</script>

{#if isPlayer && route.name === "watch"}
  <WatchPage id={route.id} episode={route.episode} />
{:else}
  <div class="shell">
    <Navbar {route} />
    <div class="content">
      {#if route.name === "home"}
        <HomePage />
      {:else if route.name === "browse"}
        <BrowsePage />
      {:else if route.name === "season"}
        <SeasonPage />
      {:else if route.name === "details"}
        <DetailsPage id={route.id} />
      {:else if route.name === "list" || route.name === "library"}
        <LibraryPage />
      {:else if route.name === "settings"}
        <SettingsPage />
      {/if}

      {#if route.name !== "settings"}
        <Footer />
      {/if}
    </div>
  </div>
{/if}

<!-- Modals & Overlays -->
{#if userStore.showSearchModal}
  <SearchModal />
{/if}

{#if userStore.showProfileModal}
  <ProfileModal />
{/if}

{#if userStore.showChangelogModal}
  <ChangelogModal />
{/if}

{#if userStore.showSettingsModal && route.name !== "settings"}
  <SettingsPage />
{/if}

<!-- Global Desktop Context Menu -->
<ContextMenu />

<style>
  .shell {
    position: relative;
    height: 100vh;
    overflow: hidden;
    background: var(--bg);
  }

  .content {
    height: 100vh;
    overflow-y: auto;
    overflow-x: hidden;
    scroll-behavior: smooth;
  }
</style>
