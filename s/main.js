import HTML from '/html.js';

(main => {const done = Promise.resolve().then(_ => main(done));})(async done => {
  done.then(
    _ => document.querySelector('#loading-message').remove(),
    error => {
      document.querySelector('#loading-message').textContent = String(error.stack);
      throw error;
    });

  const hostname = document.location.host;
  const path = document.location.pathname.split(/\//g).slice(1);
  const projectName = hostname.match(/^[a-z0-9\-]+\.glitch\.me$/) ? hostname.split('.')[0] : null;
  const defaultName = "bests";
  const title = `${projectName || defaultName}.glitch.me`;
  
  const apiRoot = '/https://www.speedrun.com/api/v1/';
  const apiFetch = async path => {
    const url = apiRoot + path;
    const response = await fetch(url);
    const body = await response.json();
    if (body.status) {
      throw new Error(`${body.status}: ${body.message}`); 
    } else {
      return body.data;
    }
  }

  document.title = (path.length) ? `${defaultName}…/${path.join('/')}` : title;

  const output = document.querySelector('#main');
  const renderHTML = (...args) => output.appendChild(HTML.fragment(...args));

  addHeader: {
    writeHtml`<header>
      <h1><a href="/">${title}</a></h1>

      ${projectName && HTML`

      `}
    </header>`;
    if (projectName) {
      header.appendChild(HTML.fragment`
          <nav class="links"><a href="${`https://glitch.com/edit/#!/${projectName}`}">view source</a></nav>`);
    }
    out(header);
  }

  let bodyRendered = false;
  renderBody: {
    if (path.length === 1) {
      const [gameSlug, playerSlug] = path[0].split('@');
      if (!gameSlug || !playerSlug) break renderBody;

      const [gameInfo, playerInfo] = await Promise.all([
        apiFetch(`games/${gameSlug}`),
        apiFetch(`users/${playerSlug}`),
      ]);

      const gameId = gameInfo.id;
      const playerId = gameInfo.id;

      const gameName = gameInfo.names.international;
      const playerName = playerInfo.names.international;

      const runsInfo = await apiFetch(`runs?user=${playerId}&game=${gameId}`);

      const icon = gameInfo.assets.icon.uri;
      const trophies = [
        'trophy-1st', 'trophy-2nd', 'trophy-3rd', 'trophy-4th'
      ].map(s => gameInfo.assets[s]).map(o => o ? o.uri : null);

      out(HTML.element`<h2><img src="${icon}"> ${gameName}</h2>`);

      out(HTML.element`<pre>${JSON.stringify(gameInfo, null, 2)}</pre>`);

      out(HTML.element`<h2><img src="${trophies[0]}"> ${playerName}</h2>`);

      out(HTML.element`<pre>${JSON.stringify(playerInfo, null, 2)}</pre>`);

      out(HTML.element`<h2><img src="${trophies[1]}"> Runs</h2>`);

      out(HTML.element`<pre>${JSON.stringify(runsInfo, null, 2)}</pre>`);
    }
    
    bodyRendered = true;
  }

  if (!bodyRendered) handle404: {
    document.location.replace('/wc2@banks');
  }
    
  addFooter: {
    out(HTML.fragment`<footer>
      This site displays data from <a href="https://www.speedrun.com/about">speedrun.com</a>.
      It is used under <a href="https://creativecommons.org/licenses/by-nc/4.0/">the CC BY-NC license</a> and
      loaded from <a href="https://github.com/speedruncomorg/api/tree/master/version1">their API</a>.
    </footer>`);
  }
});
