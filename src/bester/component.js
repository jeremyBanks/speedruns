// bester.Component subclasses render an Object of props into a bester.HTML
// instance, and add some class-name-based CSS classes to a prefix placeholder
// element facilitate styling and dedbugging.

import {HTML, TO_HTML} from '/assets/bester/html.js';


export class Component {
  constructor(props = null) {
    this.props = Object.freeze(Object.assign({}, props));

    this.element_ = null;

    this.classNames = [];
    let currentClass = this.constructor;
    while (currentClass && currentClass.name && currentClass !== Component) {
      this.classNames.push(currentClass.name);
      currentClass = Object.getPrototypeOf(currentClass);
    }
    
    this.rendered = this.constructor.render(props);

    Object.freeze(this);
  }

  element() {
    if (this.element_ === null) {
      this._element_ = document.createElement('bester-component');
      this._element.classList.add(...this.classNames);
      this._element.appendChild(this.rendered.fragment);
    }
    return this.element_;
  }

  [TO_HTML]() {
    return HTML`<bester-component class="${this.classNames.join(" ")}">${this.rendered}</bester-component>`;
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
