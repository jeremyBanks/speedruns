const express = require('express');
const compression = require('compression');
const rp = require('request-promise-native');


const app = express();

// enable etags
app.enable('etag') 

// enable compression
app.use(compression());

// disable json pretty print
app.set('json spaces', null);

// Serve this entire project directoy.
app.use(express.static('./', {
  dotfiles: 'ignore',
  index: ['index.html', '.js']
}));

// Crudely mirror and cache speedrun.com/api.
// We never expire/evict values here; we assume the
// process won't live long enough for it to matter.
const apiCache = new Map();
app.get(/^\/(https:\/\/(www\.)?speedrun\.com\/api\/(.*))/, async (req, res) => {
  const url = req.url.slice(1);
  const cached = apiCache.get(url);

  // We already have server-side caching and in-session
  // client-side caching. We don't need browser caching,
  // and disabling the cache may improve request paralallizability
  // in some cases. Our client also applies the header to requests.
  // res.set('Cache-Control', 'no-store');
  
  if (cached) {
    return res.send(await cached);
  }

  console.log("GETting", url);
  const result = rp.get(url, {simple: false}).then(JSON.parse);
  apiCache.set(url, result);

  return res.json(await result);
});

// Serve index for unknown URLs so it can route them client-side.
app.use((req, res) => {
  res.sendFile(__dirname + '/index.html');
});

const listener = app.listen(process.env.PORT, () => {
  console.log(`Your app is listening on port ${listener.address().port}`);
});
