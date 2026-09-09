<script lang="ts">
  import { router, type Route } from "./lib/router";
  import Navbar from "./components/Navbar.svelte";
  import HomePage from "./pages/HomePage.svelte";
  import BrowsePage from "./pages/BrowsePage.svelte";
  import SeasonPage from "./pages/SeasonPage.svelte";
  import DetailsPage from "./pages/DetailsPage.svelte";
  import ListPage from "./pages/ListPage.svelte";
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
      {:else if route.name === "list"}
        <ListPage />
      {:else if route.name === "library"}
        <LibraryPage />
      {:else if route.name === "settings"}
        <SettingsPage />
      {/if}
    </div>
  </div>
{/if}

<style>
  .shell {
    display: flex;
    flex-direction: column;
    height: 100vh;
  }

  .content {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
  }
</style>
