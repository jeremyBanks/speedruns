import HTML from '/src/html.js';
import {zip, devAwaitDeep, compareAll, compareDefault} from '/src/utils.js';

import * as speedrun from '/src/speedrun.js';


const getBests = async (output) => {
  const print = (line = '') => output.appendChild(HTML.fragment`<div class="line">${line || ' '}</div>`);

  const runner = await speedrun.Runner.get('18qyezox' || 'Banks');
  
  print();
  for (const game of [
    await speedrun.Game.get('o1yry26q' || 'wc2'),
    await speedrun.Game.get('wc2btdp')
  ]) {
    print(HTML`        <a href="${game.url}">${game.nick}</a>`);
    print();

    const runnables = await game.categoryLevelPairs();

    for (const level of runnables) {
      print(HTML`            <a href="${level.url}">${level.nick}</a>`);

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
          
          if (worldRecords.includes(record)) {
            lastWr = lastWr;
            lastWrIndicators = '▐' + ''.padEnd(outstandingProgress * (40 - magnitudeFudge) + magnitudeFudge).replace(/./g, '█');
          }
          if (personalRecords.includes(record)) {
            lastPr = record;
            lastPrIndicators = '▐' + ''.padEnd(outstandingProgress * (40 - magnitudeFudge) + magnitudeFudge).replace(/./g, '▐');
          }

          const indicators = zip(
            Array.from(lastWrIndicators),
            Array.from(lastPrIndicators)).map(([a, b]) => a ? a : b).join('');

          const isBanks = personalRecords.includes(record);
          const isBoth = isBanks && worldRecords.includes(record);
          
          const indicatorHTML = HTML(`<span class="${isBanks ? 'both' : 'best'}">` + indicators.replace(/(.)(▐)/, `$1</span><span class="banks ${isBanks ? 'current' : ''}">$2`) + `</span>`)
          
          const runner = await record.runner;
          print(HTML`  <a href="${record.url}">${record.durationText.padStart(9)} ${record.date}</a> <a href="${runner.url || record.url}">${runner.nick.padEnd(12)}</a> ${indicatorHTML}`);
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
