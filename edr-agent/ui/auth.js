/* auth.js: sign-in for the agent API, shared by index.html and graph.html.
 *
 * Every API call needs "Authorization: Bearer <EDR_AGENT_TOKEN>". The token is
 * typed into a sign-in field and kept only in this closure: never in
 * localStorage, a cookie or a URL, so a reload asks for it again. EventSource
 * cannot send headers, so streams are read with fetch and parsed here.
 */
(() => {
  "use strict";

  let token = null;

  // Any 401 after sign-in (token rotated, agent restarted) drops the token and
  // reloads to the sign-in field.
  async function apiFetch(path, init = {}) {
    const headers = new Headers(init.headers || {});
    if (token) headers.set("Authorization", `Bearer ${token}`);
    const res = await fetch(path, { ...init, headers });
    if (res.status === 401 && token) {
      token = null;
      location.reload();
    }
    return res;
  }

  // Reads a server-sent event stream with the token. Calls onData(text) for each
  // event's data and reconnects after 3 s, as EventSource would.
  async function stream(path, onData) {
    for (;;) {
      try {
        const res = await apiFetch(path, { headers: { Accept: "text/event-stream" }, cache: "no-store" });
        if (res.status === 401) return;
        if (res.ok && res.body) {
          const reader = res.body.getReader();
          const dec = new TextDecoder();
          let buf = "";
          for (;;) {
            const { value, done } = await reader.read();
            if (done) break;
            buf += dec.decode(value, { stream: true });
            let cut;
            while ((cut = buf.indexOf("\n\n")) >= 0) {
              const block = buf.slice(0, cut);
              buf = buf.slice(cut + 2);
              const data = block.split("\n")
                .filter((l) => l.startsWith("data:"))
                .map((l) => l.slice(5).replace(/^ /, ""))
                .join("\n");
              if (data) onData(data);
            }
          }
        }
      } catch (_) { /* network error, retry below */ }
      await new Promise((r) => setTimeout(r, 3000));
    }
  }

  // Shows the sign-in field and resolves once the agent accepts the token.
  function signIn() {
    return new Promise((resolve) => {
      const wrap = document.createElement("div");
      wrap.style.cssText = "position:fixed;inset:0;z-index:9999;display:flex;align-items:center;justify-content:center;background:rgba(2,6,23,.94);font-family:system-ui,sans-serif";
      const form = document.createElement("form");
      form.style.cssText = "background:#0f172a;color:#e2e8f0;border:1px solid #334155;border-radius:10px;padding:20px;width:340px";
      form.innerHTML =
        '<div style="font-weight:600;margin-bottom:10px">EDR agent sign-in</div>' +
        '<label style="font-size:13px;display:block">Agent API token' +
        '<input type="password" autocomplete="off" style="display:block;width:100%;box-sizing:border-box;margin-top:4px;padding:6px;background:#020617;color:#e2e8f0;border:1px solid #334155;border-radius:6px"></label>' +
        '<p style="font-size:12px;color:#94a3b8">The value of EDR_AGENT_TOKEN on the agent. It is kept in memory only, so a reload asks again.</p>' +
        '<p data-err style="font-size:12px;color:#f87171;min-height:1em"></p>' +
        '<button type="submit" style="width:100%;padding:8px;border:0;border-radius:6px;background:#0284c7;color:#fff">Sign in</button>';
      const input = form.querySelector("input");
      const err = form.querySelector("[data-err]");
      form.onsubmit = async (e) => {
        e.preventDefault();
        const t = input.value.trim();
        if (!t) return;
        try {
          const r = await fetch("/ioc/stats", { headers: { Authorization: `Bearer ${t}` }, cache: "no-store" });
          if (r.ok) {
            token = t;
            input.value = "";
            wrap.remove();
            resolve();
            return;
          }
          err.textContent = r.status === 401 ? "That token was rejected." : `Agent answered HTTP ${r.status}.`;
        } catch (_) {
          err.textContent = "Could not reach the agent.";
        }
      };
      wrap.appendChild(form);
      document.body.appendChild(wrap);
      input.focus();
    });
  }

  window.edrAuth = { ready: signIn(), fetch: apiFetch, stream };
})();
