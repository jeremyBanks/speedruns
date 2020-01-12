import HTML from "/assets/bester/html.js";
import {
  zip,
  devAwaitDeep,
  compareAll,
  compareDefault,
} from "/assets/bester/utils.js";
import { URL } from "/assets/bester/deps.js";
import { RootComponent, Component } from "/assets/bester/component.js";
import { Style, style } from "/assets/bester/style.js";

import { Header, Footer } from "/assets/common.js";
import { ReportPage } from "/assets/reports.js";
import { HomeBody } from "/assets/home.js";

export class BestsRouter extends RootComponent {
  title({ url } = this.props) {
    const hostName = url.host;
    const shortName = hostName;
    const pathNames =
      (url.pathname.slice(1) && url.pathname.slice(1).split(/\//g)) || [];

    return pathNames.length === 0
      ? hostName
      : `${shortName}/${pathNames.join("/")}`;
  }

  static parsePath(path) {
    const pathNames = path.split(/\//g);

    const pathStack = [...pathNames];

    if (pathStack[0] === "") {
      pathStack.shift();
    } else {
      throw new Error("invalid URL - " + JSON.stringify(pathNames));
    }

    if (pathStack.length === 1 && pathStack[0] === "") {
      pathStack.shift();
    }

    let gamesSlug = null;
    let gameSlugs = [];
    let levelsSlug = null;
    let levelSlugs = [];
    let runnersSlug = null;
    let runnerSlugs = [];

    if (pathStack[0] && /^[^@]/.test(pathStack[0])) {
      gamesSlug = pathStack.shift();
      gameSlugs = gamesSlug.split(/\+/g);
    }

    if (pathStack[0] && /^[^@]/.test(pathStack[0])) {
      levelsSlug = pathStack.shift();
      levelSlugs = levelsSlug.split(/\+/g);
    }

    if (pathStack[0] && /^@./.test(pathStack[0])) {
      runnersSlug = pathStack.shift();
      runnerSlugs = runnersSlug.slice(1).split(/\+@/g);
    }

    if (pathStack.length) {
      throw new Error("invalid URL - " + JSON.stringify(pathNames));
    }

    return {
      gameSlugs,
      levelSlugs,
      runnerSlugs,
    };
  }

  static makePath({ gameSlugs, levelSlugs, runnerSlugs }) {
    const pieces = [];

    gameSlugs = (gameSlugs || []).filter(Boolean);
    levelSlugs = (levelSlugs || []).filter(Boolean);
    runnerSlugs = (runnerSlugs || []).filter(Boolean);

    if (gameSlugs.length) {
      pieces.push(gameSlugs.join("+"));

      if (levelSlugs.length) {
        pieces.push(levelSlugs.join("+"));
      }
    } else if (levelSlugs.length) {
      throw new Error("can't have levelSlugs without any gameSlugs");
    }

    if (runnerSlugs.length) {
      pieces.push("@" + runnerSlugs.join("+@"));
    }

    return "/" + pieces.join("/");
  }

  async *render({ url }) {
    const hostName = url.host;

    yield Header.of({ currentProject: "bests", currentHost: hostName });

    const { gameSlugs, levelSlugs, runnerSlugs } = BestsRouter.parsePath(
      url.pathname,
    );

    if (gameSlugs.length || runnerSlugs.length) {
      yield ReportPage.of({
        gameSlugs,
        levelSlugs,
        runnerSlugs,
        makePath: BestsRouter.makePath,
      });
    } else {
      yield HomeBody.of({
        makePath: BestsRouter.makePath,
      });
    }

    yield Footer.of();
  }
}
