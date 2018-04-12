importScripts('/node_modules/sw-toolbox/sw-toolbox.js');

// except for API requests, unless we're offline 
self.toolbox.router.get('/https?:\/\//(.*)', toolbox.networkFirst);


// load everything from the cache
self.toolbox.router.get('/', toolbox.fastest);
