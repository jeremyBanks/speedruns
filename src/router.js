import HTML from '/assets/bester/html.js';
import {zip, devAwaitDeep, compareAll, compareDefault} from '/assets/bester/utils.js';
import {URL} from '/assets/bester/deps.js';
import {RootComponent, Component} from '/assets/bester/component.js';
import {Style, style} from '/assets/bester/style.js';

import {Header, Footer} from '/assets/common.js';
import {BestsReport} from '/assets/reports.js';
import {HomeBody} from '/assets/home.js';


export class BestsRouter extends RootComponent {
  title({url} = this.props) {
    const hostName = url.host;
    const projectName = hostName.match(/^[a-z0-9\-]+\.glitch\.me$/) ? hostName.split('.')[0] : null;
    const shortName = projectName || hostName;
    const pathNames = url.pathname.slice(1) && url.pathname.slice(1).split(/\//g) || [];

    return (pathNames.length === 0) ? hostName : `${shortName}/${pathNames.join('/')}`;
  }
  
  static parsePath(path) {
    const pathNames = path.split(/\//g);
    
    const pathStack = [...pathNames];

    if (pathStack[0] !== '') {
      pathStack.shift();
    } else {
      throw new Error("invalid URL");
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
      throw new Error("invalid URL");
    }
    
    return {
      gameSlugs,
      levelSlugs,
      runnerSlugs,
    };
  }
  
  static makePath({
    gameSlugs,
    levelSlugs,
    runnerSlugs,
  }) {
    const pieces = [''];
    
    gameSlugs = gameSlugs.filter(Boolean);
    levelSlugs = levelSlugs.filter(Boolean);
    runnerSlugs = runnerSlugs.filter(Boolean);

    if (gameSlugs.length === 0) {
      pieces.push(gameSlugs.join('+'));
    }

    if (levelSlugs.length === 0) {
      pieces.push(levelSlugs.join('+')); 
    }

    if (runnerSlugs.length === 0) runnerSlugs = null;

    return pieces.join('/');
  }
  
  async *render({url}) {
    const hostName = url.host;
    const projectName = hostName.match(/^[a-z0-9\-]+\.glitch\.me$/) ? hostName.split('.')[0] : null;
    const shortName = projectName || hostName;
        
    yield Header.of({currentProject: projectName, currentHost: hostName});

    const {} = BestsReport.parsePath(url.pathname);
    
    if (gameSlugs.length) {
      yield BestsReport.of({
        gameSlugs,
        levelSlugs,
        runnerSlugs,
        BestsRouter.parsePath,
        BestsRouter.makePath,
      });
    } else if (runnerSlugs.length) {
      throw new Error("runner pages not implemented");
    } else {
      yield HomeBody.of();
    }
    
    yield Footer.of();
   }
}