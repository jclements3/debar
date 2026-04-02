const CACHE_NAME = 'debar-v4';
const ASSETS = [
  '/',
  '/index.html',
  '/genesis1.html',
  '/hebrew_app.html',
  '/arabic.html',
  '/aramaic.html',
  '/gen1_data.js',
  '/narratives.json',
  '/quran_books.js',
  '/aramaic_books.js',
  '/pkg/hebrew.js',
  '/pkg/hebrew_bg.wasm',
];

self.addEventListener('install', e => {
  e.waitUntil(
    caches.open(CACHE_NAME).then(cache => cache.addAll(ASSETS))
  );
  self.skipWaiting();
});

self.addEventListener('activate', e => {
  e.waitUntil(
    caches.keys().then(keys =>
      Promise.all(keys.filter(k => k !== CACHE_NAME).map(k => caches.delete(k)))
    )
  );
  self.clients.claim();
});

self.addEventListener('fetch', e => {
  e.respondWith(
    caches.match(e.request).then(cached => cached || fetch(e.request))
  );
});
