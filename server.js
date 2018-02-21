const express = require('express');
const request = require('request-promise-native');

const app = express();

app.use(express.static('s'));

const apiCache = new Map();
app.get(/^\/(https:\/\/www\.speedrun\.com\/api\/(.*))/, async (req, res) => {
  const url = req.url.slice(1);
  const cached = apiCache.get(url);
  
  if (cached) {
    return await cached;
  }

  console.log("GETting", url);
  const result = request.get(url, {simple: false});
  apiCache.set(url, result);

  return res.send(await result);
});

app.use((req, res) => {
  res.sendFile(__dirname + '/s/index.html');
});

const listener = app.listen(process.env.PORT, () => {
  console.log(`Your app is listening on port ${listener.address().port}`);
});
