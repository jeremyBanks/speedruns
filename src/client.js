import HTML from '/src/html.js';
import {zip, devAwaitDeep, compareAll, compareDefault} from '/src/utils.js';

import * as speedrun from '/src/speedrun.js';


const getBests = async (output) => {
  const print = (line = '') => output.appendChild(HTML.fragment`<div class="line">${line}</div>`);

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
        let lastWr = null, lastWrIndicators = '';
        let lastPr = null, lastPrIndicators = '';        

        for (const record of records) {
          let outstandingProgress = (record.durationSeconds - minRecord) / (maxRecord - minRecord);
          if (records.length === 1) {
            outstandingProgress = 1;
          }
          let indicators = '▐' + ''.padEnd(outstandingProgress * (40 - magnitudeFudge) + magnitudeFudge).replace(/./g, '█');
          
          if (worldRecords.includes(record)) {
            lastWr = lastWr;
            lastWrIndicators = '▐' + ''.padEnd(outstandingProgress * (40 - magnitudeFudge) + magnitudeFudge).replace(/./g, '█');
          }
          if (personalRecords.includes(record)) {
            lastPr = record;
            lastPrIndicators = '▐' + ''.padEnd(outstandingProgress * (40 - magnitudeFudge) + magnitudeFudge).replace(/./g, '▐');
          }

          indicators = zip(
            Array.from(lastWrIndicators),
            Array.from(lastPrIndicators)).map(([a, b]) => (!b || a == '█') ? a : b).join('');
          
          print(`  ${record.durationText.padStart(9)} ${record.date} ${(await record.runner).nick.padEnd(12)} ${indicators}`);
        }
      }
      print();
    }
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

  const bestsOutput = HTML.fragment`<pre class="bestsOutput"></pre>`.firstChild;
  getBests(bestsOutput);
  output.appendChild(bestsOutput);

  output.appendChild(HTML.fragment`
    <footer>
      This site displays data from <a href="https://www.speedrun.com/about">speedrun.com</a>,
      used under <a href="https://creativecommons.org/licenses/by-nc/4.0/">the CC BY-NC license</a> and
      loaded from <a href="https://github.com/speedruncomorg/api/blob/master/version1/README.md#readme">their API</a>.
    </footer>
  `);
  
  await Promise.all(blockers);
};
