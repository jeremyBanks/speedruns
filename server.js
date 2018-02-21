const express = require('express');
const app = express();

app.use(express.static('s'));

app.get('/https://www.speedrun.com/api/', (req, res) => {
  // forward request to https://www.speedrun.com/api/, and cache locally!
});

app.use((req, res) => {
  res.sendFile(__dirname + '/s/index.html');
})

const listener = app.listen(process.env.PORT, () => {
  console.log(`Your app is listening on port ${listener.address().port}`);
});
