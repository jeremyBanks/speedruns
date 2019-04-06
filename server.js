import express from 'express';
import compression from 'compression';
import rp from 'request-promise-native';
import serveIndex from 'serve-index';
import commonmark from 'commonmark';

import url from 'url';


// We don't use this yet, but don't want to break it.
import * as speedrun from '/assets/speedrun.js';
import HTML from '/assets/bester/html.js';


const app = express();

// enable strong etags
app.set('etag', 'strong');

// enable compression
app.use(compression());

// disable json pretty print
app.set('json spaces', 2);


// Hard-coded until we have an ES-module-friendly alternative,
// such as https://github.com/tc39/proposal-import-meta
const __dirname = '/app';


// We serve all static files under the public path 'assets/' because that's what speedrun.com
// uses, so by copying that we can avoid clobbering any of their paths we'd like to mirror.

// Serve directory indexes for public/ftp folder (with icons)
app.use('/assets', serveIndex(__dirname + '/src', {
  view: 'details',
  icons: true,
}));
app.use('/assets', express.static(__dirname + '/src', {
  dotfiles: 'ignore',
  index: ['index.html', 'README.md']
}));


app.get('/*.md', async (req, res, next) => {
  const input = await new Promise((resolve, reject) => fs.readFile(__dirname + req.url, 'utf8', (err, data) => { err ? reject(err) : resolve(data); }));
  const reader = new commonmark.Parser();
  const writer = new commonmark.HtmlRenderer({safe: true});
  const result = writer.render(reader.parse(input));
  const projectName = process.env.PROJECT_NAME || 'bests';
  
  res.set('Content-Type', 'text/html');
          
  res.send(await HTML.string`<!doctype html>
<html>
<head>
<title>${req.url}</title>
<meta charset="utf-8" />
<link rel="icon" href="/assets/icon.png">
<style>
body {
  font-family: sans-serif;
  max-width: 640px;
  margin: 32px;
}
.edit-link {
  position: absolute;
  padding: 16px;
  background: #EEE;
  top: 0;
  right: 0;
  border-left: 1px solid #DDD;
  border-bottom: 1px solid #DDD;
}
</style>
</head>
<body>
${projectName && HTML`
  <div class="edit-link"><a href="https://glitch.com/edit/#!/${projectName}?path=${req.url.slice(1)}">edit on Glitch</a></div>
`}
<main>
  ${HTML(result)}
</main>
</body>
</html>`);
});

app.get('/favicon.ico', (req, res) => { res.sendFile(__dirname + '/src/icon.png'); });


import {SqliteStringMap} from './sqlite-string-map.js';

// Crudely mirror and cache speedrun.com/api.
// We never expire/evict values here; we assume the
// process won't live long enough for it to matter.
const apiCache = new SqliteStringMap('api-cache');
// apiCache.clear();

app.get(/^\/api\/(.*)/, async (req, res) => {
  const url = 'https://www.speedrun.com/api/' + req.url.slice(1);
  const cached = await apiCache.get(url);

  if (cached) {
    console.log("CACHE hit", url);
    return res.json(JSON.parse(cached));
  }

  console.log("cache MISS", url);

  const result = rp.get(url, {simple: false});
  
  // We don't await/block on the result of the set.
  apiCache.set(url, result);

  return res.json(JSON.parse(await result));
});

import {BestsRouter} from '/assets/router.js';
import {Header, Footer} from '/assets/common.js';
import fs from 'fs';
import recursiveReaddir from 'recursive-readdir';

let bodyCache = {};
app.use(async (req, res) => {
  const index = await new Promise((resolve, reject) => fs.readFile(__dirname + '/src/index.html', 'utf8', (err, data) => { err ? reject(err) : resolve(data); }));
  
  const [gamesSlug, runnerSlug] = req.url.slice(1).split(/\//g);
  const gameSlugs = gamesSlug.split(/\+/g).filter(Boolean);

  let body = '';
  const state = [];
  if (bodyCache[req.path]) {
    state.push('loaded');
    body = bodyCache[req.path];
  } else {
    try {
      body = await Promise.race([
        (async () => {
          const result = await HTML.string`<div>
            ${BestsRouter.of({url: new url.URL(req.path, `https://${req.hostname}/`)})}
          </div>`;
          bodyCache[req.path] = result;
          state.push('loaded');
          return result;
        })(),
        (async () => {
          // time-out server side render at 1s, but all required fetches will continue in background.
          await new Promise(resolve => setTimeout(resolve, 125));
          const result = await HTML.string`<div>
            ${Header.of({currentHost: req.get('host'), currentProject: process.env.PROJECT_NAME})}
            <p>
              Still loading data from speedrun.com. Please try again in a minute.
            </p>
            ${Footer.of()}
          </div>`;
          state.push('errored');
          return result;
        })(),
      ]);
    } catch (error) {
      state.push('errored');
      body = await HTML.string`<div>
            ${Header.of({currentHost: req.get('host'), currentProject: process.env.PROJECT_NAME})}
            <pre>${error}\n${error.stack}</pre>
            ${Footer.of()}
          </div>`;
    }
  }
  if (state[0] === 'loaded') {
    // res.status(200); // full response
  } else if (state[0] === 'errored') {
    // res.status(500); // maybe-persistent error
  } else {
    // res.status(504); // still 'loading' - gateway timeout
  }
  res.set('Content-Type', 'text/html');
  return res.send(index
                  .replace('unloaded', state[0])
                  .replace(`import '/assets/main.js';`, await recursiveReaddir('src').then(files => files.filter(s => /\.js$/i.test(s) && !s.includes('-node') ).map(s => `import ${JSON.stringify(s.replace(/^src\//, '/assets/'))};`).sort().join('\n')))
                  .replace('</main>', body + '</main>'));
});

// Serve index for unknown URLs so it can route them client-side.
app.use((req, res) => {
  res.sendFile(__dirname + '/src/index.html');
});

const listener = app.listen(process.env.PORT, () => {
  console.log(`Your app is listening on port ${listener.address().port}`);
});
