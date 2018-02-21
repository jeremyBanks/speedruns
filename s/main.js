import HTML from '/html.js';

(main => {const done = Promise.resolve().then(_ => main(done));})(async done => {
  done.then(
    _ => document.querySelector('#premain-message').remove(),
    error => {
      document.querySelector('#premain-message').textContent = String(error.stack);
      throw error;
    });

  const hostname = document.location.host;
  const path = document.location.pathname.split(/\//g).slice(1);
  const projectName = hostname.match(/^[a-z0-9\-]+\.glitch\.me$/) ? hostname.split('.')[0] : null;
  const defaultName = "bests";
  const title = `${projectName || defaultName}.glitch.me`;
  
  const apiRoot = '/https://www.speedrun.com/api/v1';

  document.title = (path.length) ? `${defaultName}…/${path.join('/')}` : title;

  const output = document.querySelector('#main');
  const out = child => output.appendChild(child);

  addHeader: {
    const header = HTML.element`<header><h1><a href="/">${title}</a></h1></header>`;
    if (projectName) {
      header.appendChild(HTML.fragment`
          <nav class="links"><a href="${`https://glitch.com/edit/#!/${projectName}`}">view source</a></nav>`);
    }
    out(header);
  }

  if (path.length === 1) displayGameBests: {
    const [gameSlug, playerSlug] = path[0].split('@');
    if (!gameSlug || !playerSlug) break displayGameBests;
    
    const [gameInfo, playerInfo] = await Promise.all([
      fetch(`${apiRoot}/games/${gameSlug}`).then(r => r.json()),
      fetch(`${apiRoot}/users/${playerSlug}`).then(r => r.json()),
    ]);
    
    const gameId = gameInfo.data.id;
    const playerId = gameInfo.data.id;

    const gameName = gameInfo.data.names.international;
    const playerName = playerInfo.data.names.international;
    
    const runsInfo = await fetch(`${apiRoot}/runs?user=${playerId}&game=${gameId}`).then(r => r.json());

    // https://www.speedrun.com/api/v1/games/o1yry26q/records
    // https://www.speedrun.com/api/v1/users/18qyezox/personal-bests?embed=game%2Ccategory
    // https://www.speedrun.com/api/v1/
    
    const icon = gameInfo.data.assets.icon.uri;
    const trophies = [
      'trophy-1st', 'trophy-2nd', 'trophy-3rd', 'trophy-4th'
    ].map(s => gameInfo.data.assets[s]).map(o => o ? o.uri : null);
    
    out(HTML.element`<h2><img src="${icon}"> ${gameName}</h2>`);
    
    out(HTML.element`<pre>${JSON.stringify(gameInfo, null, 2)}</pre>`);
    
    out(HTML.element`<h2><img src="${trophies[0]}"> ${playerName}</h2>`);

    out(HTML.element`<pre>${JSON.stringify(playerInfo, null, 2)}</pre>`);

    out(HTML.element`<h2><img src="${trophies[1]}"> Runs</h2>`);

    out(HTML.element`<pre>${JSON.stringify(runsInfo, null, 2)}</pre>`);
  } else handle404: {
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
