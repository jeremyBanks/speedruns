import HTML from '/assets/bester/html.js';
import {document, window, URL} from '/assets/bester/deps.js';
import {Component, RootComponent} from '/assets/bester/component.js';
import {BestsRouter} from '/assets/components.js';

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
  
  const router = BestsRouter.of({url: document.location});

  document.title = router.title();

  output.appendChild(router.element);
  blockers.push(router.rendered);

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
