const express = require('express');
const request = require('request-promise-native');

const app = express();

app.use(express.static('s'));

app.get(/^\/(https:\/\/www\.speedrun\.com\/api\/(.*))/, async (req, res) => {
  const url = req.params[0];
  res.send(await request.get(url));
});

app.use((req, res) => {
  res.sendFile(__dirname + '/s/index.html');
})

const listener = app.listen(process.env.PORT, () => {
  console.log(`Your app is listening on port ${listener.address().port}`);
});
