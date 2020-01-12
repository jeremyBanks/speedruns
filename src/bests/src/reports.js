import HTML from "/assets/bester/html.js";
import { zip, compareAll, compareDefault } from "/assets/bester/utils.js";
import { RootComponent, Component } from "/assets/bester/component.js";
import { Style, style } from "/assets/bester/style.js";

import * as speedrun from "/assets/speedrun.js";

export class ReportPage extends RootComponent {
  get preStyle() {
    return style({
      font: { size: "12px" },
      margin: "16px 0",
    });
  }

  async render({ gameSlugs, runnerSlugs, levelSlugs }) {
    if (runnerSlugs.length && !gameSlugs.length && !levelSlugs.length) {
      let runnerGames = [];
      let runnerLevels = [];
      for (const runnerSlug of runnerSlugs) {
        const forRunner = await speedrun.Runner.getGamesAndLevels(runnerSlug);
        runnerGames.push(...forRunner.gameSlugs);
        runnerLevels.push(...forRunner.levelSlugs);
      }
      gameSlugs = [...new Set(runnerGames)];
      levelSlugs = [...new Set(runnerLevels)];
    }

    return HTML`<pre ${this.preStyle}>${async function*() {
      const gamesSlug = gameSlugs.join("+");

      const games = await Promise.all(gameSlugs.map(s => speedrun.Game.get(s)));

      yield "A consistent linear scale is only used for duration differences between runs within a given category/level, not differences between between categories/levels.\n";
      yield "\n";

      for (const game of games) {
        yield BestsReportGame.of({ game, gamesSlug, runnerSlugs, levelSlugs });
      }
    }}</pre>`;
  }
}

class BestsReportGame extends Component {
  get gameLinkStyle() {
    return style({
      display: "inline-block",
      font: {
        size: "16px",
        weight: "bold",
      },
    });
  }

  async *render({ game, runnerSlugs, levelSlugs }) {
    yield HTML`      <a ${this.gameLinkStyle} id="${game.slug}" href="/${game.slug}">${game.nick}</a>\n`;
    yield "\n";

    const runsByLevel = await game.runsByCategoryLevelPairs();

    for (const [level, runs] of runsByLevel) {
      if (levelSlugs && levelSlugs.length) {
        let levelMatched = null;
        for (const levelSlug of levelSlugs) {
          if (level.matchesSlug(levelSlug)) {
            levelMatched = level;
            break;
          }
        }
        if (!levelMatched) {
          continue;
        }
      }

      yield BestsReportLevel.of({ game, level, runs, runnerSlugs });
    }
    yield "\n";
    yield "\n";
  }
}

class BestsReportLevel extends Component {
  get noRunsTextStyle() {
    return style({ opacity: 0.5 });
  }

  get levelLinkStyle() {
    return style({
      display: "inline-block",
      font: {
        size: "16px",
        weight: "bold",
      },
    });
  }

  graphBarStyleAttrStringFixMe({
    worldRecord = false,
    personalBest = false,
    previousPersonalBest = false,
  }) {
    return Style.attrValue({
      color: "transparent",
      background:
        worldRecord && personalBest
          ? "linear-gradient(to bottom, #000080 0%, #FFD700 100%)"
          : worldRecord
          ? "linear-gradient(to bottom, #DFA700 0%, #FFD700 100%)"
          : personalBest
          ? "linear-gradient(to bottom, #000080 0%, rgba(0, 0, 128, 0.125) 100%)"
          : previousPersonalBest
          ? "rgba(0, 0, 128, 0.125)"
          : "magenta",
    });
  }

  async *render({ game, level, runs, runnerSlugs }) {
    yield HTML`          <a ${this.levelLinkStyle} href="/${game.slug}/${level.nickSlug}">${level.nick}</a>\n`;

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

    if (runnerSlugs && runnerSlugs.length) {
      let pr = {};
      for (const run of runs) {
        let matchedTargetRunner = null;
        for (const runnerSlug of runnerSlugs) {
          if (run.runner.nick.toLowerCase() === runnerSlug.toLowerCase()) {
            matchedTargetRunner = runnerSlug;
          }
        }
        if (!matchedTargetRunner) {
          continue;
        }

        if (
          !pr[matchedTargetRunner] ||
          run.durationSeconds < pr[matchedTargetRunner].durationSeconds
        ) {
          personalRecords.push(run);
          pr[matchedTargetRunner] = run;
        }
      }
    }

    const maxRecord = Math.max(
      ...worldRecords.map(r => r.durationSeconds),
      ...personalRecords.map(r => r.durationSeconds),
    );
    const minRecord = Math.min(
      ...worldRecords.map(r => r.durationSeconds),
      ...personalRecords.map(r => r.durationSeconds),
    );

    const magnitudeFudge = Math.ceil(
      (Math.log(minRecord) - Math.log(16)) / Math.log(2),
    );

    const maxnitudeFudge = Math.floor(
      Math.min(maxRecord, 60 * 30) / (2 * 60) +
        Math.max(0, Math.log(maxRecord) - Math.log(60 * 60)) / Math.log(1.5),
    );

    const records = [...new Set([...personalRecords, ...worldRecords])].sort(
      compareRuns,
    );

    if (records.length === 0) {
      yield HTML`                      <span ${this.noRunsTextStyle}>(no runs)</span>\n`;
    } else {
      let lastWr = null,
        lastWrIndicators = "";
      let lastPr = null,
        lastPrIndicators = "";

      for (const record of records) {
        let outstandingProgress =
          (record.durationSeconds - minRecord) / (maxRecord - minRecord);
        if (records.length === 1) {
          outstandingProgress = 1;
        }

        if (worldRecords.includes(record)) {
          lastWr = lastWr;
          lastWrIndicators =
            "█" +
            ""
              .padEnd(
                Math.ceil(
                  outstandingProgress * (16 - magnitudeFudge + maxnitudeFudge) +
                    magnitudeFudge,
                ),
              )
              .replace(/./g, "█");
        }
        if (personalRecords.includes(record)) {
          lastPr = record;
          lastPrIndicators =
            "█" +
            ""
              .padEnd(
                Math.ceil(
                  outstandingProgress * (16 - magnitudeFudge + maxnitudeFudge) +
                    magnitudeFudge,
                ),
              )
              .replace(/./g, "▐");
        }

        const indicators = zip(
          Array.from(lastWrIndicators),
          Array.from(lastPrIndicators),
        )
          .map(([a, b]) => (a ? a : b))
          .join("");

        const isPersonal = personalRecords.includes(record);
        const isBoth = isPersonal && worldRecords.includes(record);

        const indicatorHTML = HTML(
          `<span style="${this.graphBarStyleAttrStringFixMe({
            worldRecord: true,
            personalBest: isPersonal,
          })}">` +
            indicators.replace(
              /(.)(▐)/,
              `$1</span><span style="${this.graphBarStyleAttrStringFixMe({
                personalBest: isPersonal,
                previousPersonalBest: !isPersonal,
              })}">$2`,
            ) +
            `</span>`,
        );

        const runner = await record.runner;
        yield HTML`<a href="${record.url}">${record.durationText.padStart(
          10,
        )} ${record.date}</a> <a href="/@${runner.nick}">${runner.nick.padEnd(
          15,
        )}</a> <a href="/${game.slug}/@${runner.nick}">${indicatorHTML}</a>\n`;
      }
    }
    yield "\n";
  }
}
