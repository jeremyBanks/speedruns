importScripts('/node_modules/sw-toolbox/sw-toolbox.js');
toolbox.router.get('/(https://.*)', toolbox.networkFirst);
toolbox.router.get('/(.*)', toolbox.networkFirst);
