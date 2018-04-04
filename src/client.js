import HTML from '/src/html.js';
import {zip, devAwaitDeep, compareAll, compareDefault} from '/src/utils.js';

import * as speedrun from '/src/speedrun.js';


const getBestsModel = async () => {
  const NOT_IMPLEMENTED = '🚧 NOT IMPLEMENTED 🚧';
  
  const hostname = document.location.host;
  const glitchProjectName =
        hostname.match(/^[a-z0-9\-]+\.glitch\.me$/) ? hostname.split('.')[0] : null;
  
  const runner = await speedrun.Runner.get('18qyezox' || 'Banks');
  
  const game = await speedrun.Game.get('o1yry26q' || 'wc2');
  const runnables = await game.categoryLevelPairs();
  
  const level = runnables[3];
  
  const prints = [];
  const print = (line = '') => prints.push(String(line));

  print(`Level: ${level.nick}`);
  print();

  const runs = await level.runs();
  runs.sort(compareAll(
    (a, b) => compareDefault(a.date, b.date),
    (a, b) => compareDefault(a.dateSubmitted, b.dateSubmitted),
  ));

  const worldRecords = [];
  let wr = null;
  for (const run of runs) {
    if (!wr || run.durationSeconds < wr.durationSeconds) {
      wr = run;
      worldRecords.push(wr);
    }
  }
  
  const personalRecords = [];
  let pr = null;
  for (const run of runs) {
    if (run.runner.nick !== runner.nick) continue; 
    if (!pr || run.durationSeconds < pr.durationSeconds) {
      pr = run;
      personalRecords.push(pr);
    }
  }

  const maxRecord = Math.max(...worldRecords.map(r => r.durationSeconds), ...personalRecords.map(r => r.durationSeconds));
  const minRecord = Math.min(...worldRecords.map(r => r.durationSeconds), ...personalRecords.map(r => r.durationSeconds));

  print(`World Record Over Time:`);
  for (const record of worldRecords) {
    const outstandingProgress = (record.durationSeconds - minRecord) / (maxRecord - minRecord);
    const indicators = '*' + ''.padEnd(outstandingProgress * 31).replace(/./g, '*');
    print(`  ${record.date} - ${record.durationText.padStart(6)} - ${(await record.runner).nick.padEnd(12)} ${indicators}`);
  
  }
  print();

  print(`Personal Record Over Time:`);
  for (const record of personalRecords) {
    const outstandingProgress = (record.durationSeconds - minRecord) / (maxRecord - minRecord);
    const indicators = '*' + ''.padEnd(outstandingProgress * 31).replace(/./g, '*');
    print(`  ${record.date} - ${record.durationText.padStart(6)} - ${(await record.runner).nick.padEnd(12)} ${indicators}`);
  
  }
  print();
  
  return {
    '': prints.map(l => '    ' + l.padEnd(76)),
    glitchProjectName,
    runner,
    game,
  };
};


const getBestsView = async function*(model) {
  const runnerLink = runnerReq.then(runner => HTML`<a href="${runner.weblink}">${runner.names.international}</a>`);

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
              <th>${runnerLink}'s Best</th>
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
              <th>${runnerLink}'s Best</th>
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
                            ${run.runners.map(p => p.name || p.id)}
                          </a>
                        </div>
                      `) || HTML`<span class="none">none</span>`
                  }</td>
                  <td>${runnerReq.then(runner => records
                      .filter(r => r.run.runners.some(p => p.id === runner.id))
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




({set _(_){_._=(async _=>(await _)(_._))(_)}})._ = async main => {
  (async () => {
    const loadingMessage = document.querySelector('#loading-message');
    try {
      await main;
      loadingMessage.remove();
    } catch (error) {
      loadingMessage.textContent = `${error}\n\n${error.stack}`;
      throw error;
    }
  })();

  const hostname = document.location.host;
  const d = hostname.match(/^[a-z0-9\-]+\.glitch\.me$/) ? hostname.split('.')[0] : null;

  // force HTTPS if running on Glitch, where we know it's available.
  if (d && document.location.protocol === 'http:') {
    document.location.protocol = 'https:';
  }

  const path = document.location.pathname.slice(1).split(/\//g).filter(Boolean);
  let jsonRedirect = false;
  while (path[path.length - 1] === 'json') {
    path.pop();
    jsonRedirect = true;
  }
  
  const defaultName = "bests";
  const title = `${d || defaultName}.glitch.me`;

  document.title = (path.length) ? `${defaultName}…/${path.join('/')}` : title;

  const output = await HTML.element`<div></div>`; 
  document.querySelector('#main').appendChild(output);

  output.appendChild(HTML.fragment`
    <header>
      <h1><span>
        <img src="${document.querySelector('link[rel=icon]').href}">
        <a href="/">${title}</a>
      <span></h1>

      ${d && HTML`
        <nav class="links">
          <a href="${`https://glitch.com/edit/#!/${d}?path=client.js`}">edit source code</a><br />
          <a href="/${path.concat('json').join('/')}">show view model</a>
        </nav>
      `}
    </header>
  `);

  const blockers = [];

  const model = getBestsModel();
  if (jsonRedirect || "ONLY JSON FOR YOU FOR NOW") {
    const message = await HTML.element`
      <p class="in-your-face-dev-message">
        Loading all view model data. Please wait or <button>force timeout</button>.
      </p>
    `;

    const forcedTimeout = new Promise(resolve => {
      message.querySelector('button').addEventListener('click', resolve);
    });

    output.appendChild(message);

    // we let the standard render continue below while we wait for the redirect.
    (async () => {
      const syncModel = await devAwaitDeep(model, forcedTimeout);
      const json = JSON.stringify(syncModel, null, 2);
      document.open();
      document.write(HTML.string`<!doctype html><pre style="word-wrap: break-word; white-space: pre-wrap;">${json}`)
      document.close();
      // document.location.assign(URL.createObjectURL(new Blob([], {type: 'application/json'})));
    })();
  }
  const view = getBestsView(model);

  const [fragment, done] = HTML.from(view).fragmentAndDone();
  output.appendChild(fragment);
  blockers.push(done);

  output.appendChild(HTML.fragment`
    <footer>
      This site displays data from <a href="https://www.speedrun.com/about">speedrun.com</a>,
      used under <a href="https://creativecommons.org/licenses/by-nc/4.0/">the CC BY-NC license</a> and
      loaded from <a href="https://github.com/speedruncomorg/api/blob/master/version1/README.md#readme">their API</a>.
    </footer>
  `);
  
  await Promise.all(blockers);
};
