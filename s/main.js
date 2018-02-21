import HTML from '/html.js';

((f, p) => p = (async _=> (await f)(p))())(async defer => {
  defer.then(success => {
    document.querySelector('#loading-message').remove();
  }, error => {
    document.querySelector('#loading-message').textContent = String(error.stack);
    throw error;
  });

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

  const hostname = document.location.host;
  const path = document.location.pathname.split(/\//g).slice(1);
  const projectName = hostname.match(/^[a-z0-9\-]+\.glitch\.me$/) ? hostname.split('.')[0] : null;
  const defaultName = "bests";
  const title = `${projectName || defaultName}.glitch.me`;

  document.title = (path.length) ? `${defaultName}…/${path.join('/')}` : title;

  const output = document.querySelector('#main');
  const renderHTML = (...args) => output.appendChild(HTML.fragment(...args));

  renderHTML`
    <header>
      <h1><span>
        <img src="${document.querySelector('link[rel=icon]').href}">
        <a href="/">${title}</a>
      <span></h1>

      ${projectName && HTML`
        <nav class="links"><a href="${`https://glitch.com/edit/#!/${projectName}?path=s/main.js`}">view/edit source</a></nav>
      `}
    </header>
  `;

  let bodyRendered = false;
  renderBody: {
    if (path.length === 1) {
      const [gameSlug, playerSlug] = path[0].split('@');
      if (!gameSlug || !playerSlug) break renderBody;

      const [
        gameInfo,
        playerInfo,
      ] = await Promise.all([
        apiFetch(`games/${gameSlug}?embed=levels,categories`),
        apiFetch(`users/${playerSlug}`),
      ]);

      const gameId = gameInfo.id;
      const playerId = gameInfo.id;

      const gameName = gameInfo.names.international;
      const playerName = playerInfo.names.international;

      const runsInfo = await apiFetch(`runs?user=${playerId}&game=${gameId}`);

      const icon = gameInfo.assets.icon.uri;
      const trophies = ['trophy-1st', 'trophy-2nd', 'trophy-3rd'].map(s => gameInfo.assets[s].uri);

      renderHTML`<h2><img src="${icon}"> ${gameName}</h2>`;

      renderHTML`<pre>${JSON.stringify(gameInfo, null, 2)}</pre>`;

      renderHTML`<h2><img src="${trophies[0]}"> ${playerName}</h2>`;

      renderHTML`<pre>${JSON.stringify(playerInfo, null, 2)}</pre>`;

      renderHTML`<h2><img src="${trophies[1]}"> Runs</h2>`;

      renderHTML`<pre>${JSON.stringify(runsInfo, null, 2)}</pre>`;
    }
    
    bodyRendered = true;
  }

  if (!bodyRendered) handle404: {
    document.location.replace('/wc2@banks');
  }

  renderHTML`
    <footer>
      This site displays data from <a href="https://www.speedrun.com/about">speedrun.com</a>.
      It is used under <a href="https://creativecommons.org/licenses/by-nc/4.0/">the CC BY-NC license</a> and
      loaded from <a href="https://github.com/speedruncomorg/api/tree/master/version1">their API</a>.
    </footer>
  `;
});
