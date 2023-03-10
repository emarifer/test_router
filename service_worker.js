var cacheName = "test_router";
var filesToCache = [
  "/test_router/",
  "/test_router/index.html",
  "/test_router/test_router.js",
  "/test_router/test_router_bg.wasm",
  "/test_router/styles.css",
  "/test_router/img/list.svg",
  "/test_router/img/x-lg.svg",
  "/test_router/favicon.png",
  "/test_router/manifest.json",
];

/* Start the service worker and cache all of the app's content */
self.addEventListener("install", function (e) {
  self.skipWaiting();

  e.waitUntil(
    caches.open(cacheName).then(function (cache) {
      return cache.addAll(filesToCache);
    })
  );
});

self.addEventListener("activate", (event) => {
  event.waitUntil(clients.claim());
});

// Cache first, falling back to network. VER:
// https://developer.chrome.com/docs/workbox/caching-strategies-overview/#cache-first-falling-back-to-network
self.addEventListener("fetch", (event) => {
  // Check if this is a navigation request
  if (event.request.mode === "navigate") {
    event.respondWith(
      caches.open(cacheName).then((cache) => {
        // Go to the cache first
        return cache.match("/test_router/").then((cachedResponse) => {
          // Return a cached response if we have one
          if (cachedResponse) {
            return cachedResponse;
          }

          // Otherwise, hit the network
          return fetch(event.request).then((fetchedResponse) => {
            // Add the network response to the cache for later visits
            cache.put(event.request, fetchedResponse.clone());

            // Return the network response
            return fetchedResponse;
          });
        });
      })
    );
  } else {
    return;
  }
});

// https://www.google.com/search?q=service+worker+caches+another+route&oq=service+worker+caches+another+r&aqs=chrome.2.69i57j33i10i160l4j33i21j33i22i29i30.24430j0j7&sourceid=chrome&ie=UTF-8
//
// https://christianheilmann.com/2022/01/13/turning-a-github-page-into-a-progressive-web-app/
