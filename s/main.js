import HTML from '/html.js';

({set _(_){_._=(async _=>(await _)(_._))(_)}})._ = async defer => {
  defer.then(success => {
    document.querySelector('#loading-message').remove();
  }, error => {
    document.querySelector('#loading-message').textContent = `${error}\n${error.stack}S`;
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
  const path = document.location.pathname.slice(1).split(/\//g).filter(Boolean);
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

  if (path.length === 0) {
    document.location.replace('/wc2+wc2btdp+sc1+scbw@banks');
  }

  if (path.length === 1) {
    const [gamesSlug, playerSlug] = path[0].split('@');
    if (!gamesSlug) throw new Error("no game(s) in URL");
    if (!playerSlug) throw new Error("no player in URL");

    const gameSlugs = gamesSlug.split(/\+/g).filter(Boolean);
    if (gameSlugs.length == 0) throw new Error("no game(s) in URL");

    const playerInfoReq = apiFetch(`users/${playerSlug}`);
    const gameInfoReqs = gameSlugs.map(
      gameSlug => apiFetch(`games/${gameSlug}?embed=levels,categories`));
    const gameRunReqs = gameInfoReqs.map(
      gameInfoReq => gameInfoReq.then(async gameInfo => {
        const playerInfo = await playerInfoReq;
        return apiFetch(`runs?user=${playerInfo.id}&game=${gameInfo.id}`);
      }))
    
    const playerInfo = await playerInfoReq;
    const playerName = playerInfo.names.international;

    for (const [gameInfoReq, gameRunReq] of zip(gameInfoReqs, gameRunReqs)) {
      const gameInfo = await gameInfoReq;
      const runsInfo = await gameRunReq;

      const gameName = gameInfo.names.international;

      const icon = HTML`<img src="${gameInfo.assets.icon.uri}" alt="">`;
      const gold = HTML`<img src="${gameInfo.assets['trophy-1st'].uri}" alt="1st">`;
      const silver = HTML`<img src="${gameInfo.assets['trophy-2nd'].uri}" alt="2nd">`;
      const bronze = HTML`<img src="${gameInfo.assets['trophy-3rd'].uri}" alt="3rd">`;
      const medals = [gold, silver, bronze];

      renderHTML`
        <section>
          <h2>${icon} ${gameName} ${icon}</h2>

          <h3>${icon} Full Game ${icon}</h3>

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
                  ${gold} 4h 2m 30s <br>
                  by John Smith
                </td>
                <td>
                  <span class="placement">12<sup>th</sup></span> 6h 22m 13s
                </td>
              </tr>
              <tr class="best-best">
                <th>Human Campaign</th>
                <td>
                  ${gold} 10m 13s <br>
                  by Banks
                </td>
                <td>
                  ${gold} 10m 13s
                </td>
              </tr>
            </tbody>
          </table>

          <h3>${icon} Individual Levels ${icon}</h3>

          <p>foo</p>

          <pre>${JSON.stringify(gameInfo, null, 2).slice(0, 128)}</pre>
          <pre>${JSON.stringify(playerInfo, null, 2).slice(0, 128)}</pre>
          <pre>${JSON.stringify(runsInfo, null, 2).slice(0, 128)}</pre>
        </section>
      `;
    }
  }

  renderHTML`
    <footer>
      This site displays data from <a href="https://www.speedrun.com/about">speedrun.com</a>.
      It is used under <a href="https://creativecommons.org/licenses/by-nc/4.0/">the CC BY-NC license</a> and
      loaded from <a href="https://github.com/speedruncomorg/api/tree/master/version1">their API</a>.
    </footer>
  `;
};

const zip = (...args) => {
  // like Python's itertools.zip_longest
  // from stackoverflow.com/a/10284006
  const longest = args.reduce((a, b) => a.length > b.length ? a : b, []);
  return longest.map((_, i) => args.map(array => array[i]));
};
