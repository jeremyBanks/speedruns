import {RootComponent, Component} from '/assets/bester/component.js';


export class Router extends RootComponent {
  get onElementCreated() {
    const document = this.element.currentDocument;
    
    this.props = {
      pathname: document.location.pathname
    };
    
    document.title = this.title;

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
  }

  get title() {
    throw new Error("get title() not implemented");
  }
}
