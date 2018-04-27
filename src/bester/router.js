import {RootComponent, Component} from './component.js';


// A router has only one prop, pathname, which is automatically set and updated from the document.
// It also hooks internal link presses to handle them itself.
export class Router extends RootComponent {
  get title() {
    throw new Error("get title() not implemented");
  }

  onElementCreated() {
    const document = this.element.currentDocument;
    
    this.props = {
      pathname: document.location.pathname
    };

    // NOTE that these are never removed from the document.
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
            pathname: document.location.pathname
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
  
  addClickListener(document) {
    // TODO
  }
}
