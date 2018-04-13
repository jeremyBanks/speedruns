import {HTML, TO_HTML} from '/assets/bester/html.js';


export class Component {
  constructor(props = null) {
    this.props = props;
    this.rendered = Promise.resolve(this.constructor.render(props).fragment()).then();
    Object.freeze(this);
  }

  [TO_HTML]() {
    return this.rendered;
  }

  static render(props) {
    throw new Error("not implemented"); 
  }
}

export class JSONPre extends Component {
  static render(props) {
    return HTML.element`<pre>${JSON.stringify(props, null, 2)}</pre>`;
  }
}
