import HTML from '/assets/bester/html.js';
import {zip, devAwaitDeep, compareAll, compareDefault} from '/assets/bester/utils.js';
import {RootComponent, Component} from '/assets/bester/component.js';

import * as speedrun from '/assets/speedrun.js';


const defaultPath = '/wc2+wc2btdp';

class BestsReport extends RootComponent {
  static render({gameSlugs, runnerSlug, currentHost}) {
    return HTML`<pre>${async function*() {  
      const gamesSlug = gameSlugs.join('+');

      const games = await Promise.all(gameSlugs.map(s => speedrun.Game.get(s)));

      if (runnerSlug) {
        yield HTML`World record and ${runnerSlug}'s personal best [<a href="//${currentHost}/${gamesSlug}">remove</a>] progressions over time.\n`;
      } else {
        yield "World record progressions over time. Click a runner name to compare their bests.\n";
      }

      yield "\n";
      yield "A consistent linear scale is only used for duration differences between runs within a given category/level, not differences between between categories/levels.\n";
      yield "\n";

      for (const game of games) {
        yield new BestsReportGame({game, currentHost, gamesSlug, runnerSlug});
      }
    } }</pre>`;
  }
};

class BestsReportGame extends Component {
  static async *render({game, currentHost, gamesSlug, runnerSlug}) {
    yield HTML`      <a class="game" id="${game.slug}" href="//${currentHost}/${game.slug}${runnerSlug ? `/${runnerSlug}` : ''}">${game.nick}</a>\n`;
    yield "\n";

    const runsByLevel = await game.runsByCategoryLevelPairs();

    for (const [level, runs] of runsByLevel) {
      yield new BestsReportRun({level, runs, runnerSlug, currentHost, gamesSlug});
    }
    yield "\n";
    yield "\n";
  }
}


class BestsReportRun extends Component {
  static async *render({level, runs, runnerSlug, currentHost, gamesSlug}) {
    yield HTML`          <a class="level" id="${level.slug}" href="//${currentHost}/${gamesSlug}${runnerSlug ? `/${runnerSlug}` : ''}#${level.slug}">${level.nick}</a>\n`;

    const compareRuns = compareAll(
      (a, b) => compareDefault(a.date, b.date),
      (a, b) => compareDefault(a.dateTimeSubmitted, b.dateTimeSubmitted),
    );

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
        if (run.runner.nick.toLowerCase() !== runnerSlug.toLowerCase()) continue;

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
      yield HTML`                      <span class="none">(no runs)</span>\n`;
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
        yield HTML`<a href="${record.url}">${record.durationText.padStart(9)} ${record.date}</a> <a href="//${currentHost}/${gamesSlug}/${runner.nick}#${level.slug}">${runner.nick.padEnd(14)} ${indicatorHTML}</a>\n`;
      }
    }
    yield "\n";
  }
}

class LocationProvider {
  constructor() {
    this.hostname = document.location.host.replace(/^bests\.run$/, 'bests.glitch.me');
    this.currentProject = this.hostname.match(/^[a-z0-9\-]+\.glitch\.me$/) ? this.hostname.split('.')[0] : null;
    this.canonicalProject = 'bests';
    this.canonicalHost = 'bests.run';
    this.currentHost = (this.currentProject === this.canonicalProject) ? this.canonicalHost : this.hostname;
    this.path = document.location.pathname.slice(1).split(/\//g).filter(Boolean); 
    this.hasNonDefaultProject = Boolean(this.currentProject && this.currentProject !== this.canonicalProject);
  }
  
  get docTitle() {
    return (this.path.length) ? `${this.hasNonDefaultProject 
      ? this.currentProject : this.canonicalHost}/${this.path.join('/')}` 
      : this.hasNonDefaultProject ? this.currentHost : this.canonicalHost;
  }
  
  updateDocument() {
    if (this.currentProject && document.location.protocol === 'http:') {
      document.location.protocol = 'https:';
    }

    document.title = this.docTitle;
  }
}

const doMain = async (locationProvider) => {
  const { hostname, 
          currentProject, 
          canonicalProject, 
          canonicalHost, 
          currentHost, 
          path,
          docTitle
        } = locationProvider;
  
  locationProvider.updateDocument();
  
  // navigates to an internal URL and recursively re-invokes main to re-render the page.
  const navigateInternal = async (url, replace = false) => {
    document.body.classList.remove('unloaded', 'loading', 'loaded', 'errored');
    document.body.classList.add('unloaded');
    if (!replace) {
      window.history.pushState(null, docTitle, url);
    } else {
      window.history.replaceState(null, docTitle, url);      
    }
    document.scrollingElement.scrollTop = 0;
    // calling main within a function within a function called by main.
    // possibly not ideal
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
        <a href="//${currentHost}/">${currentHost}</a>
      <span></h1>

      ${currentProject && HTML`
        <nav class="links">
          <a href="${`https://glitch.com/edit/#!/${currentProject}?path=src/client.js`}">edit source code</a><br />
        </nav>
      `}
    </header>
  `);

  const blockers = [];
  
  if (path.length === 0) {
    return await navigateInternal(defaultPath, true);
  } else if (path.length <= 2) {
    const [gamesSlug, runnerSlug] = path;
    if (!gamesSlug) throw new Error("no game(s) in URL");

    const gameSlugs = gamesSlug.split(/\+/g).filter(Boolean);
    if (gameSlugs.length == 0) throw new Error("no game(s) in URL");

    const content = new BestsReport({gameSlugs, runnerSlug, currentHost});
    // setTimeout(() => {
    //   // look, it works! that's the only reason this is here. delete it later.
    //   content.props = {gameSlugs: ['zoombinis'], runnerSlug: 'Uglie', currentHost};
    //   setTimeout(() => {
    //     content.props = {gameSlugs, runnerSlug, currentHost};
    //   }, 1000);
    // }, 3000);

    output.appendChild(content.element);
    blockers.push(content.rendered);
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
    if (!event.target.closest('a')) return; 
    let target = new URL(event.target.closest('a').href);
    if (target.host == canonicalHost) {
      target.host = document.location.host;
    }
    if (target.host === document.location.host) {
      console.debug(`🔗 Internal navigation to ${target.href}`);
      event.preventDefault();
      event.stopPropagation();
      navigateInternal(target.href);
    }
  });

  console.debug("😅 Rendering...");
  await Promise.all(blockers);
  console.info("😁 Rendered successfully!");
  document.body.classList.remove('unloaded', 'loading', 'loaded', 'errored');
  document.body.classList.add('loaded');
  
  const hash = window.location.hash;
  if (document.scrollingElement.scrollTop === 0 && hash > '#') {
    const target = document.querySelector(hash);
    if (target) {
      target.classList.add('target');
      target.scrollIntoView();
    }
  }
};

const main = async () => {
  document.body.classList.remove('unloaded', 'loading', 'loaded', 'errored');
  document.body.classList.add('loading');

  const errorMessage = document.querySelector('#error-message');
  try {
    await doMain(new LocationProvider());
    document.body.classList.remove('unloaded', 'loading', 'loaded', 'errored');
    document.body.classList.add('loaded');
  } catch (error) {
    document.body.classList.remove('loading');
    document.body.classList.add('errored');
    errorMessage.textContent = `${error}\n\n${error.stack}`;
    throw error;
  }
};

let lastLocation = new URL(document.location.href);
window.addEventListener('popstate', () => {
  const newLocation = new URL(document.location.href);
  if (newLocation.href !== lastLocation.href) {
    if (new URL('#', newLocation).href !== new URL('#', lastLocation).href) {
      console.info(`🎈 History state popped, now at ${window.location.href}`);
      main();
    } else {
      console.debug("🙄 Ignoring hash-only history state change.");
    }
  } else {
    console.debug("🤔 Ignoring non-URL-changing history state change.");
  }
  lastLocation = newLocation;
});

main();
