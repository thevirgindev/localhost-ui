// Evaluate a JS expression in the running app's webview via CDP.
// Usage: node scripts/devtools-eval.mjs "expression" [port]
const port = process.argv[3] || "9223";
const expr = process.argv[2] || "document.title";

async function main() {
  const list = await (await fetch(`http://127.0.0.1:${port}/json`)).json();
  const page = list.find((t) => t.type === "page");
  if (!page) {
    console.log("NO_PAGE_TARGETS", JSON.stringify(list.map((t) => [t.type, t.title, t.url])));
    return;
  }
  const ws = new WebSocket(page.webSocketDebuggerUrl);
  await new Promise((res, rej) => {
    ws.onopen = res;
    ws.onerror = rej;
  });
  const result = await new Promise((res) => {
    ws.onmessage = (ev) => {
      const msg = JSON.parse(ev.data);
      if (msg.id === 1) res(msg.result ?? msg);
    };
    ws.send(
      JSON.stringify({
        id: 1,
        method: "Runtime.evaluate",
        params: { expression: expr, returnByValue: true, awaitPromise: true },
      }),
    );
    setTimeout(() => res({ timeout: true }), 8000);
  });
  console.log(JSON.stringify(result, null, 2));
  ws.close();
}

main().catch((e) => {
  console.error("EVAL_FAILED:", e.message);
  process.exit(1);
});
