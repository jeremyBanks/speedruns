import HTML from '/assets/bester/html.js';
import {zip, devAwaitDeep, compareAll, compareDefault} from '/assets/bester/utils.js';
import {RootComponent, Component} from '/assets/bester/component.js';

import * as speedrun from '/assets/speedrun.js';


export class Header extends Component {
  static render({currentHost, currentProject}) {
    return HTML`<header>
      <h1><span>
        <img src="/assets/icon.png">
        <a href="//${currentHost}/">${currentHost}</a>
      <span></h1>

      ${currentProject && HTML`
        <nav class="links">
          <a href="${`https://glitch.com/edit/#!/${currentProject}?path=src/client.js`}">edit source code</a><br />
        </nav>
      `}
    </header>`;
  }
}


export class Footer extends Component {
  static render({}) {
    return HTML`<footer>
      This site displays data from <a href="https://www.speedrun.com/about">speedrun.com</a>,
      used under <a href="https://creativecommons.org/licenses/by-nc/4.0/">the CC BY-NC license</a> and
      loaded from <a href="https://github.com/speedruncomorg/api/blob/master/version1/README.md#readme">their API</a>.
    </footer>`;
  }
}

export class BestsReport extends RootComponent {
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
