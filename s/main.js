import HTML from '/html.js';

({set _(_){_._=(async _=>(await _)(_._))(_)}})._ = async defer => {
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
      const [gold, silver, bronze] = ['trophy-1st', 'trophy-2nd', 'trophy-3rd'].map(s => gameInfo.assets[s].uri);

      renderHTML`
        <h2>${gameName}</h2>

        <h3><img src="${icon}"> Full Game <img src="${icon}"></h3>

        <table>
          <thead>
            <tr>
              <th>Category</th>
              <th>World Record</th>
              <th>${playerName}'s Best</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <th>All Campaigns</th>
              <td>
                -
              </td>
              <td>-</td>
            </tr>
            <tr>
              <th>Orc Campaign</th>
              <td>
                <img src="${gold}"> 4h 2m 30s <br>
                by John Smith
              </td>
              <td>
                <img src="${silver}"> 6h 22m 13s
              </td>
            </tr>
            <tr>
              <th>Human Campaign</th>
              <td>
                <img src="${gold}"> 10m 13s <br>
                by John Smith
              </td>
              <td>-</td>
            </tr>
          </tbody>
        </table>

        <h3><img src="${icon}"> Individual Levels <img src="${icon}"></h3>

        <p>foo</p>

        <pre>${JSON.stringify(gameInfo, null, 2)}</pre>
        <pre>${JSON.stringify(playerInfo, null, 2)}</pre>
        <pre>${JSON.stringify(runsInfo, null, 2)}</pre>
      `;
    }
    
    bodyRendered = true;
  }

  if (!bodyRendered) handle404: {
    document.location.replace('/wc2+wc2btdp@banks');
  }

  renderHTML`
    <footer>
      This site displays data from <a href="https://www.speedrun.com/about">speedrun.com</a>.
      It is used under <a href="https://creativecommons.org/licenses/by-nc/4.0/">the CC BY-NC license</a> and
      loaded from <a href="https://github.com/speedruncomorg/api/tree/master/version1">their API</a>.
    </footer>
  `;
};
