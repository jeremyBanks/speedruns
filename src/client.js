import HTML from '/src/html.js';
import {zip, devAwaitDeep, compareAll, compareDefault} from '/src/utils.js';

import * as speedrun from '/src/speedrun.js';


const defaultPath = '/wc2+wc2btdp';


const getBests = (gameSlugs, runnerSlug) => {
  return HTML`<pre class="bestsOutput">${async function*() {  
    const line = (content = '') => HTML`<div class="content">${content || ' '}</div>`;

    const gamesSlug = gameSlugs.join('+');
    
    const games = await Promise.all(gameSlugs.map(s => speedrun.Game.get(s)));
    
    yield line(HTML`World record progressions over time${
               runnerSlug ? HTML`, with <a href="/${gamesSlug}/${runnerSlug}">${runnerSlug}</a>'s personal bests for comparison` :
                 `. Click on a runner to compare their personal bests`}.`);

    yield line();
    yield line("Scales and ranges are not consistent across categories/levels. A consistent linear scale is only used for duration differences between runs within a given category/level.");
    yield line();
    for (const game of games) yield async function*() {
        yield line(HTML`      <a class="game" id="${game.slug}" href="/${game.slug}">${game.nick}</a>`);
        yield line();

        const runnables = await game.categoryLevelPairs();

        for (const level of runnables) yield async function*() {
          yield line(HTML`          <a class="level" id="${level.slug}" href="#${level.slug}">${level.nick}</a>`);

          const compareRuns = compareAll(
            (a, b) => compareDefault(a.date, b.date),
            (a, b) => compareDefault(a.dateTimeSubmitted, b.dateTimeSubmitted),
          );
          
          const runs = await level.runs();
          runs.sort(compareRuns);

          const worldRecords = [];
          let wr = null;
          for (const run of runs) {
            if (!wr || run.durationSeconds <= wr.durationSeconds) {
              worldRecords.push(run);
              wr = run;
            }
          }

          const personalRecords = [];
          
          if (runnerSlug) {
            let pr = null;
            for (const run of runs) {
              if (run.runner.nick !== runnerSlug) continue;

              if (!pr || run.durationSeconds < pr.durationSeconds) {
                personalRecords.push(run);
                pr = run;
              }
            }
          }

          const maxRecord = Math.max(...worldRecords.map(r => r.durationSeconds), ...personalRecords.map(r => r.durationSeconds));
          const minRecord = Math.min(...worldRecords.map(r => r.durationSeconds), ...personalRecords.map(r => r.durationSeconds));

          const magnitudeFudge = Math.ceil((Math.log(minRecord) - Math.log(16)) / Math.log(2));

          const maxnitudeFudge = Math.floor(Math.min(maxRecord, 60 * 30) / (2 * 60) + (Math.max(0, Math.log(maxRecord) - Math.log(60*60)))/Math.log(1.5));
                                            
          const records = [...new Set([...personalRecords, ...worldRecords])].sort(compareRuns);

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
                lastWrIndicators = '█' + ''.padEnd(Math.ceil(outstandingProgress * (16 - magnitudeFudge + maxnitudeFudge) + magnitudeFudge)).replace(/./g, '█');
              }
              if (personalRecords.includes(record)) {
                lastPr = record;
                lastPrIndicators = '█' + ''.padEnd(Math.ceil(outstandingProgress * (16 - magnitudeFudge + maxnitudeFudge) + magnitudeFudge)).replace(/./g, '▐');
              }

              const indicators = zip(
                Array.from(lastWrIndicators),
                Array.from(lastPrIndicators)).map(([a, b]) => a ? a : b).join('');

              const isBanks = personalRecords.includes(record);
              const isBoth = isBanks && worldRecords.includes(record);

              const indicatorHTML = HTML(`<span class="${isBanks ? 'both' : 'best'}">` + indicators.replace(/(.)(▐)/, `$1</span><span class="banks ${isBanks ? 'current' : ''}">$2`) + `</span>`)

              const runner = await record.runner;
              yield line(HTML`<a href="${record.url}">${record.durationText.padStart(9)} ${record.date}</a> <a href="/${gamesSlug}/${runner.nick}">${runner.nick.padEnd(14)}</a> ${indicatorHTML}`);
            }
          }
          yield line();
        }
        yield line();
        yield line();
      }
  } }</pre>`;
};




const main = async () => {
  (async () => {
    document.body.classList.remove('unloaded');
    document.body.classList.add('loading');
    const errorMessage = document.querySelector('#error-message');
    try {
      await main.done;
      document.body.classList.remove('loading');
      document.body.classList.add('loaded');
    } catch (error) {
      document.body.classList.remove('loading');
      document.body.classList.add('errored');
      errorMessage.textContent = `${error}\n\n${error.stack}`;
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

  const docTitle = (path.length) ? `${defaultName}/${path.join('/')}` : title
  document.title = docTitle;

  // navigates to an internal URL and recursively re-invokes main to re-render the page.
  const navigateInternal = async url => {
    window.history.pushState(null, docTitle, url);
    return await main();
  };

  const mainContainer = document.querySelector('#main');
  mainContainer.textContent = '';
  
  const output = await HTML.element`<div></div>`; 
  mainContainer.appendChild(output);

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
    return await navigateInternal(defaultPath);
  } else if (path.length <= 2) {
    const [gamesSlug, runnerSlug] = path;
    if (!gamesSlug) throw new Error("no game(s) in URL");

    const gameSlugs = gamesSlug.split(/\+/g).filter(Boolean);
    if (gameSlugs.length == 0) throw new Error("no game(s) in URL");

    const content = getBests(gameSlugs, runnerSlug);
    
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

  output.addEventListener('click', event => {
    console.log(event.target.href);
    if (event.target.host === document.location.host) {
      console.debug("Handling click internally.");
      event.preventDefault();
      event.stopPropagation();
      navigateInternal(event.target.href);
    }
  });

  await Promise.all(blockers);
  console.info("Rendered successfully! 😁");
  document.body.classList.remove('loading');
  document.body.classList.add('loaded');
  
  const hash = window.location.hash;
  if (document.scrollingElement.scrollTop === 0 && hash > '#') {
    const target = document.querySelector(hash);
    if (target) {
      target.classList.add('target');
    }
    window.location.hash = '';
    window.location.hash = hash;
    if (target) {
      target.blur();
    }
  }
};
main.done = main();
