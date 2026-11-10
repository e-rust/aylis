// Gerado pelo Aylis: guarda a página no navegador para abrir rápido (e sem rede).
const SCOPE = self.registration.scope;
const PREFIX = "aylis:" + SCOPE + ":";
const CACHE = PREFIX + "5e7a6a8e72";
// A página (a pasta, com a barra no fim) e o que ela carrega antes de desenhar.
const PRECACHE = ["./", "./pkg/aylis_demos-5e7a6a8e72.js", "./pkg/aylis_demos_bg-5e7a6a8e72.wasm"];
// Com o hash no nome: o conteúdo nunca muda.
const IMMUTABLE = /-[0-9a-f]{10}\.(js|wasm)$/;

// A resposta sem a marca de redirecionamento: o navegador recusa uma resposta redirecionada
// para uma navegação (o ERR_FAILED).
async function clean(response) {
  if (!response.redirected) return response;
  return new Response(await response.blob(), { status: response.status, statusText: response.statusText, headers: response.headers });
}

// A chave de uma navegação: sem a query e, na página da pasta, sem o `index.html`.
function pageKey(url) {
  const key = new URL(url.href);
  key.search = "";
  key.hash = "";
  if (key.pathname.endsWith("/index.html")) key.pathname = key.pathname.slice(0, -"index.html".length);
  return key.href;
}

self.addEventListener("install", (event) => {
  // Sem rede (ou com um arquivo faltando) a instalação falha e o service worker anterior,
  // com o cache dele, continua valendo.
  event.waitUntil((async () => {
    const cache = await caches.open(CACHE);
    for (const file of PRECACHE) {
      const url = new URL(file, SCOPE).href;
      const response = await fetch(new Request(url, { cache: "reload" }));
      if (!response.ok) throw new Error(url + ": " + response.status);
      await cache.put(url, await clean(response));
    }
    await self.skipWaiting();
  })());
});

self.addEventListener("activate", (event) => {
  event.waitUntil((async () => {
    const cache = await caches.open(CACHE);
    for (const key of await caches.keys()) {
      if (!key.startsWith(PREFIX) || key === CACHE) continue;
      // O que a página baixou depois (fontes, imagens, outras páginas) passa para o cache
      // novo, para continuar abrindo sem rede; o `.js` e o `.wasm` antigos não servem mais.
      const old = await caches.open(key);
      for (const request of await old.keys()) {
        if (IMMUTABLE.test(new URL(request.url).pathname) || (await cache.match(request))) continue;
        const response = await old.match(request);
        if (response) await cache.put(request, response);
      }
      await caches.delete(key);
    }
    await self.clients.claim();
  })());
});

self.addEventListener("fetch", (event) => {
  const request = event.request;
  const url = new URL(request.url);
  if (request.method !== "GET" || url.origin !== location.origin) return;
  const navigate = request.mode === "navigate";
  const key = navigate ? pageKey(url) : request;
  event.respondWith((async () => {
    const cache = await caches.open(CACHE);
    if (IMMUTABLE.test(url.pathname)) {
      const hit = await cache.match(request);
      if (hit) return hit;
    }
    try {
      const response = await fetch(request);
      if (response.ok && response.type === "basic") {
        const copy = await clean(response.clone());
        event.waitUntil(cache.put(key, copy));
      }
      return response;
    } catch (error) {
      // Sem rede: a cópia guardada (deste cache ou, na troca de versão, de outro).
      const hit = (await cache.match(key)) || (await caches.match(key)) || (await caches.match(request, { ignoreSearch: true }));
      if (hit) return hit;
      // `/ide` sem a barra: o servidor redirecionaria para `/ide/`; sem ele, quem redireciona
      // é aqui, se a pasta estiver guardada.
      if (navigate && !url.pathname.endsWith("/") && !url.pathname.split("/").pop().includes(".")) {
        const folder = new URL(url.href);
        folder.pathname += "/";
        if (await caches.match(pageKey(folder))) return Response.redirect(folder.href, 302);
      }
      throw error;
    }
  })());
});

// Web Push (o componente PushNotifications): a mensagem vira uma linha para a página
// ("message\tid\ttítulo\ttexto\tdados JSON"). Com a página na frente, quem decide mostrar é
// ela; senão, a notificação aparece aqui (o navegador exige).
const field = (text) => String(text ?? "").replace(/[\t\r\n]/g, " ");
function pushLine(payload) {
  let p = {};
  try { p = JSON.parse(payload); } catch (_) { p = { body: payload }; }
  if (typeof p !== "object" || p === null) p = { body: String(payload) };
  const n = p.notification || {};
  const data = p.data && typeof p.data === "object" ? p.data : {};
  return [field(p.id), field(p.title ?? n.title), field(p.body ?? n.body), JSON.stringify(data)].join("\t");
}

self.addEventListener("push", (event) => {
  const line = pushLine(event.data ? event.data.text() : "");
  event.waitUntil((async () => {
    const windows = await self.clients.matchAll({ type: "window", includeUncontrolled: true });
    for (const w of windows) w.postMessage({ aylis_push: "message\t" + line });
    if (windows.some((w) => w.focused || w.visibilityState === "visible")) return;
    const [, title, body] = line.split("\t");
    await self.registration.showNotification(title || self.registration.scope, { body, data: { aylis_push: line } });
  })());
});

self.addEventListener("notificationclick", (event) => {
  event.notification.close();
  const line = (event.notification.data && event.notification.data.aylis_push) || "";
  event.waitUntil((async () => {
    const windows = await self.clients.matchAll({ type: "window", includeUncontrolled: true });
    if (windows.length) {
      const w = windows[0];
      w.postMessage({ aylis_push: "opened\t" + line });
      if ("focus" in w) await w.focus();
    } else {
      await self.clients.openWindow(self.registration.scope + "?aylis_push=" + encodeURIComponent(line));
    }
  })());
});
