import {RootComponent, Component} from '/assets/bester/component.js';


export class Router extends RootComponent {
  
  
  get onElementCreated() {
    // TODO: hook into location system and stuff through element.currentDocument
  }

  get title() {
    throw new Error("get title() not implemented");
  }
}
