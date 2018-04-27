import HTML from '/assets/bester/html.js';
import {document, window} from '/assets/bester/deps.js';
import {Component} from '/assets/bester/component.js';
import {BestsReport, Header, Footer} from '/assets/components.js';


const defaultPath = '/wc2+wc2btdp';


class BestsRouter extends Component {
  render({url}) {
    const hostName = url.host;
    const projectName = hostname.match(/^[a-z0-9\-]+\.glitch\.me$/) ? hostname.split('.')[0] : null;
    const shortName = projectName || hostName;
    const pathNames = url.pathname.slice(1).split(/\//g);

    const title = 
      (pathNames.length === 0) ? hostName : `${shortName}/${this.path.join('/')}`;
    
    
   }
}


  
  
  updateDocument() {

    document.title = this.docTitle;
  }
}

const doMain = async (locationProvider, showIncomplete = false) => {
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
  
  const output = await HTML.element`<div></div>`; 

  output.appendChild(await HTML.element`${Header.of({currentProject, currentHost})}`);

  const blockers = [];
  
  if (path.length === 0) {
    return await navigateInternal(defaultPath, true);
  } else if (path.length <= 2) {
    const [gamesSlug, runnerSlug] = path;
    if (!gamesSlug) throw new Error("no game(s) in URL");

    const gameSlugs = gamesSlug.split(/\+/g).filter(Boolean);
    if (gameSlugs.length == 0) throw new Error("no game(s) in URL");

    const content = BestsReport.of({gameSlugs, runnerSlug, currentHost});

    output.appendChild(content.element);
    blockers.push(content.rendered);
  } else {
    throw new Error("404/invalid URL");
  }

  output.appendChild(await HTML.element`${Footer.of()}`);

  output.addEventListener('click', event => {
    // only catch unmodified left clicks.
    if (event.buttons > 1) return;
    if (event.altKey || event.ctrlKey || event.metaKey || event.shiftKey) return;

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
    await doMain(new LocationProvider(), wasUnloaded);
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
