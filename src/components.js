import HTML from '/assets/bester/html.js';
import {zip, devAwaitDeep, compareAll, compareDefault} from '/assets/bester/utils.js';
import {RootComponent, Component} from '/assets/bester/component.js';
import {Style, style} from '/assets/bester/style.js';

import * as speedrun from '/assets/speedrun.js';


export class Header extends Component {
  get style() {
    return style({
      text: {align: 'left'}
    });
  }
  
  get headerTextStyle() {
    return style({
      display: 'inline',
      border: {
        bottom: {
          _: '2px solid #000',
          left: {radius: '10px 6px'},
          right: {radius: '32px 2px'},
        }
      },
      position: 'relative',
      top: '-7px',
      padding: {right: '4px'}
    });
  }
  
  get headerTextInnerStyle() {
    return style({
      position: 'relative',
      top: '7px'
    });
  }
  
  get linksStyle() {
    return style({
      float: 'right',
      margin: {top: '4px'},
      font: {size: '12px'},
      line: {height: '16px'},
      text: {align: 'right'}
    });
  }

  render({currentHost, currentProject}) {
    return HTML`<header>
      <h1 ${this.headerTextStyle}><span ${this.headerTextInnerStyle}>
        <img src="/assets/icon.png">
        <a href="//${currentHost}/">${currentHost}</a>
      <span></h1>

      ${currentProject && HTML`
        <nav class="links" ${this.linksStyle}>
          <a href="${`https://glitch.com/edit/#!/${currentProject}?path=src/client.js`}">edit on Glitch</a><br />
        </nav>
      `}
    </header>`;
  }
}


export class Footer extends Component {
  get style() {
    return style({
      font: {size: '0.75em'},
      margin: {top: '128px'}
    });
  }

  render({}) {
    return HTML`<footer>
      This site displays data from <a href="https://www.speedrun.com/about">speedrun.com</a>,
      used under <a href="https://creativecommons.org/licenses/by-nc/4.0/">the CC BY-NC license</a> and
      loaded from <a href="https://github.com/speedruncomorg/api/blob/master/version1/README.md#readme">their API</a>.
    </footer>`;
  }
}

export class BestsReport extends RootComponent {
  get preStyle() {
    return style({
      font: {size: '12px'},
      margin: '16px 0'
    });
  }

  render({gameSlugs, runnerSlug, currentHost}) {
    return HTML`<pre ${this.preStyle}>${async function*() {  
      const gamesSlug = gameSlugs.join('+');

      const games = await Promise.all(gameSlugs.map(s => speedrun.Game.get(s)));

      if (runnerSlug) {
        yield HTML`World record and ${runnerSlug}'s personal best <span no-print>[<a href="//${currentHost}/${gamesSlug}">remove</a>] </span>progressions over time.\n`;
      } else {
        yield "World record progressions over time. Click a runner name to compare their bests.\n";
      }

      yield "\n";
      yield "A consistent linear scale is only used for duration differences between runs within a given category/level, not differences between between categories/levels.\n";
      yield "\n";

      for (const game of games) {
        yield BestsReportGame.of({game, currentHost, gamesSlug, runnerSlug});
      }
    }}</pre>`;
  }
};

class BestsReportGame extends Component { 
  get gameLinkStyle() {
    return style({
      display: 'inline-block',
      font: {
        size: '16px',
        weight: 'bold'
      }
    });
  }

  async *render({game, currentHost, gamesSlug, runnerSlug}) {
    yield HTML`      <a ${this.gameLinkStyle} id="${game.slug}" href="//${currentHost}/${game.slug}${runnerSlug ? `/${runnerSlug}` : ''}">${game.nick}</a>\n`;
    yield "\n";

    const runsByLevel = await game.runsByCategoryLevelPairs();

    for (const [level, runs] of runsByLevel) {
      yield BestsReportRun.of({level, runs, runnerSlug, currentHost, gamesSlug});
    }
    yield "\n";
    yield "\n";
  }
}


class BestsReportRun extends Component {
  get noRunsTextStyle() {
    return style({opacity: 0.5});
  }

  get levelLinkStyle() {
    return style({
      display: 'inline-block',
      font: {
        size: '16px',
        weight: 'bold'
      }
    });
  }
  
  graphBarStyleAttrStringFixMe({worldRecord = false, personalBest = false, previousPersonalBest = false}) {
    return Style.attrValue({
      color: 'transparent',
      background:
        (worldRecord && personalBest) ? 'linear-gradient(to bottom, #000080 0%, #FFD700 100%)' :
        (worldRecord) ? 'linear-gradient(to bottom, #DFA700 0%, #FFD700 100%)' :
        (personalBest) ? 'linear-gradient(to bottom, #000080 0%, rgba(0, 0, 128, 0.125) 100%)' :
        (previousPersonalBest) ? 'rgba(0, 0, 128, 0.125)' :
        'magenta'
    });
  }
  
  async *render({level, runs, runnerSlug, currentHost, gamesSlug}) {
    yield HTML`          <a ${this.levelLinkStyle} id="level-${level.slug}" href="//${currentHost}/${gamesSlug}${runnerSlug ? `/${runnerSlug}` : ''}#level-${level.slug}">${level.nick}</a>\n`;

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
      yield HTML`                      <span ${this.noRunsTextStyle}>(no runs)</span>\n`;
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

        const isPersonal = personalRecords.includes(record);
        const isBoth = isPersonal && worldRecords.includes(record);

        const indicatorHTML = HTML(`<span style="${this.graphBarStyleAttrStringFixMe({worldRecord: true, personalBest: isPersonal})}">` + indicators.replace(/(.)(▐)/, `$1</span><span style="${this.graphBarStyleAttrStringFixMe({personalBest: isPersonal, previousPersonalBest: !isPersonal})}">$2`) + `</span>`)

        const runner = await record.runner;
        yield HTML`<a href="${record.url}">${record.durationText.padStart(9)} ${record.date}</a> <a href="//${currentHost}/${gamesSlug}/${runner.nick}#${level.slug}">${runner.nick.padEnd(14)} ${indicatorHTML}</a>\n`;
      }
    }
    yield "\n";
  }
}
