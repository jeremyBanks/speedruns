import HTML from '/assets/bester/html.js';
import {document, window} from '/assets/bester/deps.js';
import {Component} from '/assets/bester/component.js';
import {BestsReport, Header, Footer} from '/assets/components.js';


const defaultPath = '/wc2+wc2btdp';


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
  
  const output = await HTML.element`<div></div>`; 

  output.appendChild(await HTML.element`${new Header({currentProject, currentHost})}`);

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

  output.appendChild(await HTML.element`${new Footer()}`);

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
  await Promise.all(blockers);
  mainContainer.textContent = '';
  mainContainer.appendChild(output);
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
  // document.body.classList.remove('unloaded', 'loading', 'loaded', 'errored');
  // document.body.classList.add('loading');

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
