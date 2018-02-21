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

  addHeading: {
    const heading = HTML.element`<h1><a href="/">${title}</a></h1>`;
    if (projectName) {
      heading.appendChild(HTML.element`
          <span class="links"> <a href="${`https://glitch.com/edit/#!/${projectName}`}">view source</a></span>`);
    }
    out(heading);
  }
  

  if (path.length === 1) displayGameBests: {
    const [gameId, playerId] = path[0].split('@');
    if (!gameId || !playerId) break displayGameBests;
    
    const 
    const response = (await fetch(`${apiRoot}/games/${gameId}`)).json();
    const info = await response.json();

    // https://www.speedrun.com/api/v1/games/o1yry26q/records
    // https://www.speedrun.com/api/v1/users/18qyezox/personal-bests?embed=game%2Ccategory
    // https://www.speedrun.com/api/v1/runs?user=18qyezox&game=o1yry26q
    
    const name = info.data.names.international;
    const icon = info.data.assets.icon.uri;
    const trophies = [
      'trophy-1st', 'trophy-2nd', 'trophy-3rd', 'trophy-4th'
    ].map(s => info.data.assets.icon[s]).map(o => o ? o.url : null);
    
    out(HTML.element`<h2><img src="${icon}"> ${name}</h2>`);
    
    out(HTML.element`<pre>${JSON.stringify(info, null, 2)}</pre>`);
    return;
  }

  handle404: {
    document.location.replace('/wc2@banks');
    return;
  }
});
