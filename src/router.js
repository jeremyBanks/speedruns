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
  
  async *render({url}) {
    const hostName = url.host;
    const projectName = hostName.match(/^[a-z0-9\-]+\.glitch\.me$/) ? hostName.split('.')[0] : null;
    const shortName = projectName || hostName;
    const pathNames = url.pathname.slice(1) && url.pathname.slice(1).split(/\//g) || [];
        
    yield Header.of({currentProject: projectName, currentHost: hostName});

    if (pathNames.length === 0) {
      yield HomeBody.of();
    } else if (pathNames.length <= 2) {
      let [gamesSlug, runnerSlug] = pathNames;
      
      if (runnerSlug) runnerSlug = runnerSlug.replace(/^@+/, '');

      const gameSlugs = gamesSlug.split(/\+/g).filter(Boolean);
      if (gameSlugs.length == 0) throw new Error("no game(s) in URL");

      yield BestsReport.of({gameSlugs, runnerSlug, currentHost: hostName});
    } else {
      throw new Error("404/invalid URL");
    }
    
    yield Footer.of();
   }
}