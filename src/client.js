import HTML from '/src/html.js';
import {zip, devAwaitDeep, compareAll, compareDefault} from '/src/utils.js';

import * as speedrun from '/src/speedrun.js';


const defaultPath = 'wc2+wc2btdp@banks';


const getBests = (gameSlugs, playerSlug) => {
  return HTML`<pre class="bestsOutput">${async function*() {  
    const line = (content = '') => HTML`<div class="content">${content || ' '}</div>`;

    const runner = await speedrun.Runner.get(playerSlug);

    const games = await Promise.all(gameSlugs.map(s => speedrun.Game.get(s)));

    yield line(HTML`Historical progression of <a href="${runner.url}">${runner.nick}</a>'s personal bests against the world records:`);
    yield line();
    for (const game of games) {
      yield line(HTML`      <a class="game" href="${game.url}">${game.nick}</a>`);
      yield line();

      const runnables = await game.categoryLevelPairs();

      for (const level of runnables) {
        yield line(HTML`          <a class="level" href="${level.url}">${level.nick}</a>`);

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
          yield line(HTML`                      <span class="none">(no runs)</span>`);
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
              lastWrIndicators = '█' + ''.padEnd(outstandingProgress * (40 - magnitudeFudge) + magnitudeFudge).replace(/./g, '█');
            }
            if (personalRecords.includes(record)) {
              lastPr = record;
              lastPrIndicators = '█' + ''.padEnd(outstandingProgress * (40 - magnitudeFudge) + magnitudeFudge).replace(/./g, '▐');
            }

            const indicators = zip(
              Array.from(lastWrIndicators),
              Array.from(lastPrIndicators)).map(([a, b]) => a ? a : b).join('');

            const isBanks = personalRecords.includes(record);
            const isBoth = isBanks && worldRecords.includes(record);

            const indicatorHTML = HTML(`<span class="${isBanks ? 'both' : 'best'}">` + indicators.replace(/(.)(▐)/, `$1</span><span class="banks ${isBanks ? 'current' : ''}">$2`) + `</span>`)

            const runner = await record.runner;
            yield line(HTML`<a href="${record.url}">${record.durationText.padStart(9)} ${record.date}</a> <a href="${runner.url || record.url}">${runner.nick.padEnd(14)}</a> ${indicatorHTML}`);
          }
        }
        yield line();
      }
    }
  }}</pre>`;
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
          <a href="${`https://glitch.com/edit/#!/${d}?path=src/client.js`}">edit source code</a><br />
        </nav>
      `}
    </header>
  `);

  const blockers = [];
  
  if (path.length === 0) {
    document.location.replace(`/${defaultPath}`);
  } else if (path.length === 1) {
    const [gamesSlug, playerSlug] = path[0].split('@');
    if (!gamesSlug) throw new Error("no game(s) in URL");
    if (!playerSlug) throw new Error("no player in URL");

    const gameSlugs = gamesSlug.split(/\+/g).filter(Boolean);
    if (gameSlugs.length == 0) throw new Error("no game(s) in URL");

    const content = getBests(gameSlugs, playerSlug);
    
    const [fragment, done] = HTML.from(content).fragmentAndDone();
    output.appendChild(fragment);
    blockers.push(done);
  } else {
    throw new Error("404/invalid URL");
  }

  output.appendChild(HTML.fragment`
    <footer>
      This site displays data from <a href="https://www.speedrun.com/about">speedrun.com</a>,
      used under <a href="https://creativecommons.org/licenses/by-nc/4.0/">the CC BY-NC license</a> and
      loaded from <a href="https://github.com/speedruncomorg/api/blob/master/version1/README.md#readme">their API</a>.
    </footer>
  `);
  
  await Promise.all(blockers);
};
