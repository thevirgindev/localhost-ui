// Thin wrapper over Tauri invoke.

const isTauri = typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

export async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  if (!isTauri) {
    throw new Error("localhost must run inside the Tauri shell");
  }
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<T>(cmd, args);
}

export const tauriAvailable = isTauri;

// Fire-and-forget local error capture — written to a local log file only.
export function reportError(where: string, err: unknown) {
  if (!isTauri) return;
  const message = `${where}: ${err instanceof Error ? (err.stack ?? err.message) : String(err)}`;
  import("@tauri-apps/api/core")
    .then(({ invoke }) => invoke("log_frontend_error", { message }).catch(() => {}))
    .catch(() => {});
}
