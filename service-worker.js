importScripts('/assets/sw-toolbox/sw-toolbox.js');
// This isn't smart or robust, but should at least allow you to
// re-load a page you were just viewing before going offline.
toolbox.router.get('/(.*)', toolbox.networkFirst, {
  cache: {
    name: 'offline-fallback',
    maxEntries: 256
  }
});
