import { mount } from "svelte";
import "./app.css";
import App from "./App.svelte";
import { reportError } from "./lib/ipc";

window.addEventListener("error", (e) => reportError("window.onerror", e.error ?? e.message));
window.addEventListener("unhandledrejection", (e) => reportError("unhandledrejection", e.reason));

// Temporary boot diagnostics surfaced via native window title (dev verification).
(async () => {
  try {
    const { getCurrentWindow } = await import("@tauri-apps/api/window");
    await getCurrentWindow().setTitle("localhost: js-running");
    const { invoke } = await import("@tauri-apps/api/core");
    await invoke("log_frontend_error", { message: "boot-ping" });
    await getCurrentWindow().setTitle("localhost: ipc-ok");
  } catch (e) {
    try {
      const { getCurrentWindow } = await import("@tauri-apps/api/window");
      await getCurrentWindow().setTitle(`localhost: fail ${String(e).slice(0, 80)}`);
    } catch {
      // window API unavailable — leave title as-is
    }
  }
})();

const app = mount(App, { target: document.getElementById("app")! });

export default app;
