import HTML from '/src/html.js';
import {zip, devAwaitDeep, compareAll, compareDefault} from '/src/utils.js';

import * as speedrun from '/src/speedrun.js';


const getBests = async () => {
    const prints = [];
  const print = (line = '') => prints.push(String(line));

  const runner = await speedrun.Runner.get('18qyezox' || 'Banks');
  
  print();
  for (const game of [
    await speedrun.Game.get('o1yry26q' || 'wc2'),
    await speedrun.Game.get('wc2btdp')
  ]) {
    print(`        ${game.nick}`);
    print();

    const runnables = await game.categoryLevelPairs();

    for (const level of runnables) {
      print(`            ${level.nick}`);

      const runs = await level.runs();
      runs.sort(compareAll(
        (a, b) => compareDefault(a.date, b.date),
        (a, b) => compareDefault(a.dateSubmitted, b.dateSubmitted),
      ));

      const worldRecords = [];
      let wr = null;
      for (const run of runs) {
        if (!wr || run.durationSeconds <= wr.durationSeconds) {
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

      const magnitudeFudge = (Math.log(minRecord) - Math.log(16)) / Math.log(2);
      
      const records = [...new Set([...personalRecords, ...worldRecords])].sort((a, b) => compareDefault(a.date, b.date))

      if (records.length === 0) {
        print("                        (no runs)");
      } else {
        let lastWr = null, lastWrIndicators = null;
        for (const record of records) {
          let outstandingProgress = (record.durationSeconds - minRecord) / (maxRecord - minRecord);
          if (records.length === 1) {
            outstandingProgress = 1;
          }
          let indicators = '▐' + ''.padEnd(outstandingProgress * (40 - magnitudeFudge) + magnitudeFudge).replace(/./g, '█');
          if (lastWr && personalRecords.includes(record) && !worldRecords.includes(record)) {
            indicators = zip(
              Array.from(lastWrIndicators),
              Array.from(indicators.replace(/./g, '▐'))).map(([a, b]) => a == '█' ? a : b).join('');  
          } else {
            lastWr = record;
            lastWrIndicators = indicators;
          }
          print(`  ${record.durationText.padStart(9)} ${record.date} ${(await record.runner).nick.padEnd(12)} ${indicators}`);
        }
      }
      print();
    }
  }
  
  const f = ''.padEnd(80).replace(/./g, '\'');
  return [f, ...prints.map(l => l.padEnd(80)), f];
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
        </nav>
      `}
    </header>
  `);

  const blockers = [];

  const bests = getBests();

  const message = await HTML.element`
    <p class="in-your-face-dev-message">
      Loading data. Please wait or <button>force timeout</button>.
    </p>
  `;

  const forcedTimeout = new Promise(resolve => {
    message.querySelector('button').addEventListener('click', resolve);
  });

  output.appendChild(message);

  // we let the standard render continue below while we wait for the redirect.
  (async () => {
    const syncModel = await devAwaitDeep(bests, forcedTimeout);
    const json = JSON.stringify(syncModel, null, 2);
    document.open('text/plain');
    if (document.contentType == 'text/plain') {
      document.write(json);
    } else {
      document.write(HTML.string`<!doctype html><pre style="word-wrap: break-word; white-space: pre-wrap;">${json}`)
    }
    document.close();
    // document.location.assign(URL.createObjectURL(new Blob([], {type: 'application/json'})));
  })();

  output.appendChild(HTML.fragment`
    <footer>
      This site displays data from <a href="https://www.speedrun.com/about">speedrun.com</a>,
      used under <a href="https://creativecommons.org/licenses/by-nc/4.0/">the CC BY-NC license</a> and
      loaded from <a href="https://github.com/speedruncomorg/api/blob/master/version1/README.md#readme">their API</a>.
    </footer>
  `);
  
  await Promise.all(blockers);
};
