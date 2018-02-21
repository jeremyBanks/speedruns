const express = require('express');
const request = require('request-promise-native');

const app = express();

app.use(express.static('s'));

const apiCache = new Map();
app.get(/^\/(https:\/\/www\.speedrun\.com\/api\/(.*))/, async (req, res) => {
  const url = req.url.slice(1);
  if (apiCache.has(url)) {
    return res.send(apiCache.get(url));
  }
  console.log("Loading and caching", url);
  const result = await request.get(url, {simple: false});
  apiCache.set(url, result);
  return res.send(result);
});

app.use((req, res) => {
  res.sendFile(__dirname + '/s/index.html');
});

const listener = app.listen(process.env.PORT, () => {
  console.log(`Your app is listening on port ${listener.address().port}`);
});
