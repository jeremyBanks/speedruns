const express = require('express');
const request = require('request-promise-native');

const app = express();

// Serve static files from s/.
app.use(express.static('s'));

// Crudely mirror and cache speedrun.com/api.
// We never expire/evict values here; we assume
// the process won't live long enough for this to get very stale.
const apiCache = new Map();
app.get(/^\/(https:\/\/www\.speedrun\.com\/api\/(.*))/, async (req, res) => {
  const url = req.url.slice(1);
  const cached = apiCache.get(url);
  
  if (cached) {
    return res.send(await cached);
  }

  console.log("GETting", url);
  const result = request.get(url, {simple: false});
  apiCache.set(url, result);

  return res.send(await result);
});

// Serve index for unknown URLs so it can route them client-side.
app.use((req, res) => {
  res.sendFile(__dirname + '/s/index.html');
});

const listener = app.listen(process.env.PORT, () => {
  console.log(`Your app is listening on port ${listener.address().port}`);
});
