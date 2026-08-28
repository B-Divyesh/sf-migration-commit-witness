const CACHE = 'mcw-shell-v4';
const SHELL = ['/', '/demo/', '/privacy/', '/terms/', '/404.html', '/demo-record.json', '/witness-core.webp', '/share.webp', '/favicon.svg', '/apple-touch-icon.png', '/manifest.webmanifest'];

async function precacheShell() {
  const cache = await caches.open(CACHE);
  await cache.addAll(SHELL);
  const assetUrls = new Set();
  for (const page of ['/', '/demo/', '/privacy/', '/terms/', '/404.html']) {
    const response = await cache.match(page);
    if (!response) continue;
    const html = await response.text();
    for (const match of html.matchAll(/(?:src|href)="(\/assets\/[^\"]+)"/g)) assetUrls.add(match[1]);
  }
  await cache.addAll([...assetUrls]);
}

self.addEventListener('install', (event) => {
  event.waitUntil(precacheShell().then(() => self.skipWaiting()));
});

self.addEventListener('activate', (event) => {
  event.waitUntil(caches.keys().then((keys) => Promise.all(keys.filter((key) => key !== CACHE).map((key) => caches.delete(key)))).then(() => self.clients.claim()));
});

self.addEventListener('fetch', (event) => {
  if (event.request.method !== 'GET' || new URL(event.request.url).origin !== location.origin) return;
  if (event.request.mode === 'navigate') {
    event.respondWith(
      fetch(event.request).then((response) => {
        if (response.ok) caches.open(CACHE).then((cache) => cache.put(event.request, response.clone()));
        return response;
      }).catch(() => caches.match(event.request).then((cached) => cached || caches.match('/404.html'))),
    );
    return;
  }
  event.respondWith(
    caches.match(event.request).then((cached) => cached || fetch(event.request).then((response) => {
      if (response.ok) caches.open(CACHE).then((cache) => cache.put(event.request, response.clone()));
      return response;
    }).catch(() => Response.error())),
  );
});
