import {TO_HTML} from '/assets/bester/html.js';


export class Component {
  constructor(props = null) {
    this.props = null;
    this.render = null;
    // this.state? nope.
    Object.freeze(this);
    this.setProps(props);
  }
  
  this

  [TO_HTML]() {
    return this.render;
  }

  render() {
    throw new Error("not implemented"); 
  }
}
