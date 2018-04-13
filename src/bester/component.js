import {TO_HTML} from '/assets/bester/html.js';


export class Component {
  constructor(data = null) {
    this.data = null;
    this.render = null;
    Object.freeze(this
    this.setData(data);
  }
  
  

  [TO_HTML]() {
    return this.render;
  }

  render() {
    throw new Error("not implemented"); 
  }
}
