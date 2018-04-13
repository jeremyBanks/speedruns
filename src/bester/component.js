import {TO_HTML} from '/assets/bester/html.js';


export class Component {
  constructor(data) {
    this.data = data;
  }

  [TO_HTML]() {
    return this.render;
  }

  render() {
    throw new Error("not implemented"); 
  }
}
