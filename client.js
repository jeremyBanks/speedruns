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

  const hostname = document.location.host;
  const projectName = hostname.match(/^[a-z0-9\-]+\.glitch\.me$/) ? hostname.split('.')[0] : null;

  // force HTTPS if running on Glitch, where we know it's available.
  if (projectName && document.location.protocol === 'http:') {
    document.location.protocol = 'https:';
  }

  const path = document.location.pathname.slice(1).split(/\//g).filter(Boolean);

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
  };
  const apiCache = new Map();
  const api = async path => {
    if (!apiCache.has(path)) {
      const result = await apiFetch(path);
      apiCache.set(path, result);
      return result;
    } else {
      return apiCache.get(path);
    }
  };
  const defaultName = "bests";
  const title = `${projectName || defaultName}.glitch.me`;

  document.title = (path.length) ? `${defaultName}…/${path.join('/')}` : title;

  const output = HTML.element`<div></div>`; 
  document.querySelector('#main').appendChild(output);

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
    document.location.replace('/wc2+wc2btdp@banks');
  }

  if (path.length === 1) {
    const [gamesSlug, playerSlug] = path[0].split('@');
    if (!gamesSlug) throw new Error("no game(s) in URL");
    if (!playerSlug) throw new Error("no player in URL");

    const gameSlugs = gamesSlug.split(/\+/g).filter(Boolean);
    if (gameSlugs.length == 0) throw new Error("no game(s) in URL");

    const playerReq = api(`users/${playerSlug}`);
    const gameReqs = gameSlugs.map(
      gameSlug => api(`games/${gameSlug}?embed=levels,categories,players`));
    const gameRunsReqs = gameReqs.map(
      gameReq => gameReq.then(async game => {
        const player = await playerReq;
        return api(`runs?user=${player.id}&game=${game.id}`);
      }))
    
    const player = await playerReq;
    const playerName = player.names.international;

    for (const [gameReq, gameRunsReq] of zip(gameReqs, gameRunsReqs)) {
      const game = await gameReq;
      const runs = await gameRunsReq;

      const gameName = game.names.international;

      const icon = HTML`<img src="${game.assets.icon.uri}" alt="">`;
      const placement = n => {
        const suffix =
            (n % 10 == 1 && n % 100 != 11) ? 'st' :
            (n % 10 == 2 && n % 100 != 12) ? 'nd' :
            (n % 10 == 3 && n % 100 != 13) ? 'rd' :
            'th';

        const nth = `${n}${suffix}`;

        let asset = game.assets[`trophy-${nth}`];

        if (asset) {
          return HTML`<img class="placement" src="${asset.uri}" alt="${nth}">`;
        } else {
          return HTML`<span class="placement">${n}<sup>${suffix}</sup></span>`;
        }
      };

      renderHTML`
        <section>
          <h2>${icon} ${gameName} ${icon}</h2>

          <h3>${icon} <a href="${game.weblink}/full_game">Full Game</a> ${icon}</h3>

          <table class="game-records">
            <thead>
              <tr>
                <th>Category</th>
                <th>World Record</th>
                <th><a href="${player.weblink}">${playerName}</a>'s Best</th>
              </tr>
            </thead>
            <tbody>
              ${game.categories.data.map(c => {
                if (c.type === 'per-game') return HTML`
                  <tr class="">
                    <th><a href="${c.weblink}">${c.name}</a></th>
                    <td><span class="none">none</span></td>
                    <td><span class="none">none</span></td>
                  </tr>
                `
              })}
            </tbody>
          </table>

          <h3>${icon} <a href="${game.weblink}/individual_levels">Individual Levels</a> ${icon}</h3>

          <table class="level-records">
            <thead>
              <tr>
                <th>Level</th>
                <th>World Record</th>
                <th><a href="${player.weblink}">${playerName}</a>'s Best</th>
              </tr>
            </thead>
            <tbody>
              ${await Promise.all(game.levels.data.map(async level => {
                const records = (await api(`levels/${level.id}/records?max=200`))[0].runs;
            
                return HTML`
                  <tr class="">
                    <th><a href="${level.weblink}">${level.name}</a></th>
                    <td>${
                      records
                        .filter(r => r.place == 1)
                        .map(r => r.run)
                        .map(run => HTML`
                          <div>
                            <a href="${run.weblink}">
                              <span class="time">${run.times.primary.toLowerCase().slice(2).replace(/\D+/g, s => `${s} `).trim()}</span>
                              ${placement(1)}
                              ${run.players.map(p => p.name || p.id)}
                            </a>
                          </div>
                        `) || HTML`<span class="none">none</span>`
                    }</td>
                    <td>${
                      records
                        .filter(r => r.run.players.some(p => p.id === player.id))
                        .slice(0, 1)
                        .map(record => HTML`
                          <div>
                            <a href="${record.run.weblink}">
                              <span class="time">${record.run.times.primary.toLowerCase().slice(2).replace(/\D+/g, s => `${s} `).trim()}</span>
                              ${placement(record.place)}
                            </a>
                          </div>
                        `) || HTML`<span class="none">none</span>`
                    }</td>
                  </tr>
                `
              }))}
            </tbody>
          </table>
        </section>
      `;
    }
  }

  renderHTML`
    <footer>
      This site displays data from <a href="https://www.speedrun.com/about">speedrun.com</a>,
      used under <a href="https://creativecommons.org/licenses/by-nc/4.0/">the CC BY-NC license</a> and
      loaded from <a href="https://github.com/speedruncomorg/api/blob/master/version1/README.md#readme">their API</a>.
    </footer>
  `;
};

const zip = (...args) => {
  // like Python's itertools.zip_longest
  // from stackoverflow.com/a/10284006
  const longest = args.reduce((a, b) => a.length > b.length ? a : b, []);
  return longest.map((_, i) => args.map(array => array[i]));
};
