// Frontend router — hash-based so it works inside the Tauri webview without
// any server-side routing.

import { writable } from "svelte/store";

export type Route =
  | { name: "home" }
  | { name: "browse" }
  | { name: "season" }
  | { name: "details"; id: number }
  | { name: "watch"; id: number; episode: number }
  | { name: "list" }
  | { name: "library" }
  | { name: "settings" };

function parseHash(): Route {
  const raw = window.location.hash.replace(/^#\/?/, "") || "";
  const [pathPart, queryPart] = raw.split("?");
  const queryParams = new URLSearchParams(queryPart || "");
  const parts = pathPart.split("/").filter(Boolean);

  const first = (parts[0] || "").toLowerCase();

  switch (first) {
    case "":
    case "home":
      return { name: "home" };
    case "browse":
      return { name: "browse" };
    case "season":
    case "seasons":
      return { name: "season" };
    case "anime":
    case "details": {
      const idFromQuery = Number(queryParams.get("id"));
      const idFromPath = Number(parts[1]);
      return { name: "details", id: idFromQuery || idFromPath || 0 };
    }
    case "watch":
      return { name: "watch", id: Number(parts[1]) || 0, episode: Number(parts[2]) || 1 };
    case "list":
    case "lists":
    case "mylists":
    case "my-lists":
    case "watchlist":
    case "library":
      return { name: "list" };
    case "settings":
      return { name: "settings" };
    default:
      return { name: "home" };
  }
}

function toHash(route: Route): string {
  switch (route.name) {
    case "home":
      return "#/";
    case "browse":
      return "#/browse";
    case "season":
      return "#/season";
    case "details":
      return `#/anime/${route.id}`;
    case "watch":
      return `#/watch/${route.id}/${route.episode}`;
    case "list":
      return "#/list";
    case "library":
      return "#/library";
    case "settings":
      return "#/settings";
  }
}

function createRouter() {
  const route = writable<Route>(parseHash());

  window.addEventListener("hashchange", () => {
    route.set(parseHash());
  });

  return {
    subscribe: route.subscribe,
    current(): Route {
      let value: Route = { name: "home" };
      route.subscribe((r) => (value = r))();
      return value;
    },
    navigate(next: Route, replace = false) {
      const hash = toHash(next);
      if (replace) {
        window.history.replaceState(null, "", hash);
        route.set(next);
      } else {
        window.location.hash = hash;
      }
    },
  };
}

export const router = createRouter();

export function go(path: string) {
  window.location.hash = path;
}
