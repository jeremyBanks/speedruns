import HTML from './lib/html.js';

({set _(_){_._=(async _=>(await _)(_._))(_)}})._ = async result => {
  (async () => {
    const loadingMessage = document.querySelector('#loading-message');
    try {
      await result;
      loadingMessage.remove();
    } catch (error) {
      loadingMessage.innerHTML =
        HTML.string`<b>${error}</b>\n\n${error.stack}`;
      throw error;
    }
  })();

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
      gameSlug => apiFetch(`games/${gameSlug}?embed=levels,categories,players`));
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
      const placement = n => {
        const suffix =
            (n % 10 == 1 && n % 100 != 11) ? 'st' :
            (n % 10 == 2 && n % 100 != 12) ? 'nd' :
            (n % 10 == 3 && n % 100 != 13) ? 'rd' :
            'th';

        let asset =
            (n == 1) ? gameInfo.assets['trophy-1st'] :
            (n == 2) ? gameInfo.assets['trophy-2nd'] :
            (n == 3) ? gameInfo.assets['trophy-3rd'] :
            (n == 4) ? gameInfo.assets['trophy-4th'] :
            null;

        if (asset) {
          return HTML`<img class="placement" src="${asset.uri}" alt="${n}${suffix}">`;
        } else {
          return HTML`<span class="placement">${n}<sup>${suffix}</sup></span>`;
        }
      };

      renderHTML`
        <section>
          <h2>${icon} ${gameName} ${icon}</h2>

          <h3>${icon} <a href="${gameInfo.weblink}/full_game">Full Game</a> ${icon}</h3>

          <table>
            <thead>
              <tr>
                <th>Category</th>
                <th>World Record</th>
                <th><a href="${playerInfo.weblink}">${playerName}</a>'s Best</th>
              </tr>
            </thead>
            <tbody>
              ${gameInfo.categories.data.map(c => {
                if (c.type === 'per-game') return HTML`
                  <tr class="">
                    <th>${c.name}</th>
                    <td>-</td>
                    <td>-</td>
                  </tr>
                `
              })}
            </tbody>
          </table>

          <h3>${icon} <a href="${gameInfo.weblink}/individual_levels">Individual Levels</a> ${icon}</h3>

          <hr>
          <pre>gameInfo.categories === ${JSON.stringify(gameInfo.categories, null, 2).slice(0, 256)}</pre>
          <hr>
          <pre>gameInfo.levels === ${JSON.stringify(gameInfo.levels, null, 2).slice(0, 256)}</pre>
          <hr>
          <pre>runsInfo === ${JSON.stringify(runsInfo, null, 2).slice(0, 256)}</pre>
          <hr>
        </section>
      `;
    }
    
    renderHTML`
        <pre>${JSON.stringify(playerInfo, null, 2)}</pre>
    `;
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
