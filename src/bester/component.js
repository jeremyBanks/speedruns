import {HTML, TO_HTML} from '/assets/bester/html.js';


export class Component {
  constructor(props = null) {
    this.props = props;
    this.rendered = HTML.from(this.constructor.render(props)).element().then(element => {
      element.classList.add(this.constructor.name);
      return element;
    });
    Object.freeze(this);
  }

  [TO_HTML]() {
    return this.rendered;
  }

  static render(props) {
    throw new Error("not implemented"); 
  }
}

class JSONPre extends Component {
  static render(props) {
    return HTML`<pre>${JSON.stringify(props, null, 2)}</pre>`;
  }
}
