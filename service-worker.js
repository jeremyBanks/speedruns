importScripts('/node_modules/sw-toolbox/sw-toolbox.js');
toolbox.router.get(/^\/https?:\/\//i, toolbox.networkOnly);
toolbox.router.get(/^\/(?!https?:\/\/)/i, toolbox.fastest);
