import {RootComponent, Component} from './component.js';


// XXXX THIS IS NOT BEING USED
// 

// A router has only one prop, pathname, which is automatically set and updated from the document.
// It also hooks internal link presses to handle them itself.
export class Router extends RootComponent {
  get title() {
    throw new Error("get title() not implemented");
  }

  onElementCreated() {
    const document = this.element.currentDocument;
    
    this.props = {
      url: document.location.pathname + document.location.query + document.location.hash
    };

    // NOTE that since we never remove these from the document,
    // you probably don't want create multiple Router instances.
    this.addPopStateListener(document);
    this.addClickListener(document);
  }
  
  onElementRendered() {
    const document = this.element.currentDocument;

    document.title = this.title;
  }
  
  addPopStateListener(document) {
    let lastLocation = new URL(document.location.href);
    document.window.addEventListener('popstate', () => {
      const newLocation = new URL(document.location.href);
      if (newLocation.href !== lastLocation.href) {
        if (new URL('#', newLocation).href !== new URL('#', lastLocation).href) {
          console.info(`🎈 History state popped, now at ${document.location.href}`);
          this.props = {
            url: document.location.pathname + document.location.query + document.location.hash
          };
        } else {
          console.debug("🙄 Ignoring hash-only history state change.");
        }
      } else {
        console.debug("🤔 Ignoring non-URL-changing history state change.");
      }
      lastLocation = newLocation;
    });
  }
  
  navigate(url) {
    if (!replace) {
      window.history.pushState(null, docTitle, url);
    } else {
      window.history.replaceState(null, docTitle, url);      
    }
    document.scrollingElement.scrollTop = 0;
    
    this.state = {
      
    }
    
    document.body.classList.remove('unloaded', 'loading', 'loaded', 'errored');
    document.body.classList.add('unloaded');

    return main();
  }
  
  addClickListener(document) {
    document.addEventListener('click', event => {
      // only catch unmodified left clicks.
      if (event.buttons > 1) { return; }
      if (event.altKey || event.ctrlKey || event.metaKey || event.shiftKey) { return; }

      if (!event.target.closest('a')) { return; }

      let target = new URL(event.target.closest('a').href);
      if (target.host === document.location.host) {
        console.debug(`🔗 Internal navigation to ${target.href}`);
        event.preventDefault();
        event.stopPropagation();
        this.navigate(target.href);
      }
    });
  }
}
