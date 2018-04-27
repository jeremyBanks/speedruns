import HTML from '/assets/bester/html.js';
import {document, window, URL} from '/assets/bester/deps.js';
import {Component, RootComponent} from '/assets/bester/component.js';
import {BestsReport, Header, Footer} from '/assets/components.js';


const defaultPath = '/wc2+wc2btdp/banks';


class BestsRouter extends RootComponent {
  get title() {
    const {url} = this.props;

    const hostName = url.host;
    const projectName = hostName.match(/^[a-z0-9\-]+\.glitch\.me$/) ? hostName.split('.')[0] : null;
    const shortName = projectName || hostName;
    const pathNames = url.pathname.slice(1) && url.pathname.slice(1).split(/\//g) || [];

    const title = 
      (pathNames.length === 0) ? hostName : `${shortName}/${pathNames.join('/')}`;
  }
  
  render({url}) {
    // hmmmm!
    document.title = this.title;

    const hostName = url.host;
    const projectName = hostName.match(/^[a-z0-9\-]+\.glitch\.me$/) ? hostName.split('.')[0] : null;
    const shortName = projectName || hostName;
    const pathNames = url.pathname.slice(1) && url.pathname.slice(1).split(/\//g) || [];
    
    if (pathNames.length === 0) {
      return this.render({url: new URL(defaultPath, url)});
    } else if (pathNames.length <= 2) {
      const [gamesSlug, runnerSlug] = pathNames;
      if (!gamesSlug) throw new Error(`no game(s) in URL, ${JSON.stringify(pathNames)}`);

      const gameSlugs = gamesSlug.split(/\+/g).filter(Boolean);
      if (gameSlugs.length == 0) throw new Error("no game(s) in URL");

      return [
        Header.of({currentProject: projectName, currentHost: hostName}),
        BestsReport.of({gameSlugs, runnerSlug, currentHost: hostName}),
        Footer.of()
      ];
    } else {
      throw new Error("404/invalid URL");
    }
   }
}

const doMain = async (showIncomplete = false) => {
  const currentHost = document.location.host;
  
  // navigates to an internal URL and recursively re-invokes main to re-render the page.
  const navigateInternal = async (url, replace = false) => {
    document.body.classList.remove('unloaded', 'loading', 'loaded', 'errored');
    document.body.classList.add('unloaded');
    if (!replace) {
      window.history.pushState(null, url, url);
    } else {
      window.history.replaceState(null, url, url);      
    }
    document.scrollingElement.scrollTop = 0;
    // calling main within a function within a function called by main.
    // possibly not ideal
    return await main();
  };

  const mainContainer = document.querySelector('#main');
  
  const output = await HTML.element`<div></div>`; 

  const blockers = [];
  
  const content = BestsRouter.of({url: document.location});

  output.appendChild(content.element);
  blockers.push(content.rendered);

  output.addEventListener('click', event => {
    // only catch unmodified left clicks.
    if (event.buttons > 1) return;
    if (event.altKey || event.ctrlKey || event.metaKey || event.shiftKey) return;

    if (!event.target.closest('a')) return;

    let target = new URL(event.target.closest('a').href);
    if (target.host === document.location.host) {
      console.debug(`🔗 Internal navigation to ${target.href}`);
      event.preventDefault();
      event.stopPropagation();
      navigateInternal(target.href);
    }
  });

  
  console.debug("😅 Rendering...");
  if (showIncomplete) {
    mainContainer.textContent = '';
    mainContainer.appendChild(output);
  }
    
  await Promise.all(blockers);

  if (!showIncomplete) {
    mainContainer.textContent = '';
    mainContainer.appendChild(output);
  }

  console.info("😁 Rendered successfully!");
  document.body.classList.remove('unloaded', 'loading', 'loaded', 'errored');
  document.body.classList.add('loaded');
  
  const hash = document.location.hash;
  if (document.scrollingElement.scrollTop === 0 && hash > '#') {
    const target = document.querySelector(hash);
    if (target) {
      target.classList.add('target');
      target.scrollIntoView();
    }
  }
};

const main = async () => {
  let wasUnloaded = false;
  if (document.body.classList.contains('unloaded')) {
    wasUnloaded = true;
    document.body.classList.remove('unloaded', 'loading', 'loaded', 'errored');
    document.body.classList.add('loading');
  }

  const errorMessage = document.querySelector('#error-message');
  try {
    await doMain(wasUnloaded);
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
      console.info(`🎈 History state popped, now at ${document.location.href}`);
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
