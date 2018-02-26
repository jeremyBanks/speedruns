import HTML from '/lib/html.js';
import {zip} from '/lib/iteration.js';

import {defaultPath} from '/config/client.js';


const getBestsView = async function*(model) {


  const playerLink = playerReq.then(player => HTML`<a href="${player.weblink}">${player.names.international}</a>`);

  for (const [gameReq, gameRunsReq] of zip(gameReqs, gameRunsReqs)) {
    const icon = gameReq.then(game => HTML`<img src="${game.assets.icon.uri}" alt="">`);
    const placement = async (n) => {
      const suffix =
          (n % 10 == 1 && n % 100 != 11) ? 'st' :
          (n % 10 == 2 && n % 100 != 12) ? 'nd' :
          (n % 10 == 3 && n % 100 != 13) ? 'rd' :
          'th';

      const nth = `${n}${suffix}`;

      let asset = (await gameReq).assets[`trophy-${nth}`];

      if (asset) {
        return HTML`<img class="placement" src="${asset.uri}" alt="${nth}">`;
      } else {
        return HTML`<span class="placement">${n}<sup>${suffix}</sup></span>`;
      }
    };

    yield HTML`
      <section>${gameReq.then(game => HTML`
        <h2>${icon} ${game.names.international} ${icon}</h2>

        <h3>${icon} <a href="${game.weblink}/full_game">Full Game</a> ${icon}</h3>

        <table class="game-records">
          <thead>
            <tr>
              <th>Category</th>
              <th>World Record</th>
              <th>${playerLink}'s Best</th>
            </tr>
          </thead>
          <tbody>
            ${gameReq.then(game => game.categories.data.map(c => {
              if (c.type === 'per-game') return HTML`
                <tr class="">
                  <th><a href="${c.weblink}">${c.name}</a></th>
                  <td><span class="none">none</span></td>
                  <td><span class="none">none</span></td>
                </tr>
              `
            }))}
          </tbody>
        </table>

        <h3>${icon} <a href="${game.weblink}/individual_levels">Individual Levels</a> ${icon}</h3>

        <table class="level-records">
          <thead>
            <tr>
              <th>Level</th>
              <th>World Record</th>
              <th>${playerLink}'s Best</th>
            </tr>
          </thead>
          <tbody>
            ${game.levels.data.map(async level => {
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
                  <td>${playerReq.then(player => records
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
                  )}</td>
                </tr>
              `
            })}
          </tbody>
        </table>
      `)}</section>
    `;
  }
};


let api; {
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
  api = async path => {
    if (!apiCache.has(path)) {
      const result = await apiFetch(path);
      apiCache.set(path, result);
      return result;
    } else {
      return apiCache.get(path);
    }
  };
};
