// bester.Component subclasses render an Object of props into a bester.HTML
// instance, and add some class-name-based CSS classes to a prefix placeholder
// element facilitate styling and dedbugging.

import {HTML, TO_HTML} from '/assets/bester/html.js';


class LazySymbolScope {
  constructor() {
    return new Proxy(this, LazySymbolScope.handler); 
  }
}
                       
LazySymbolScope.handler = {
  set(this, key, value, proxy) {
      this[key] = value;
      console.log('PROXY SET');
      return true;
  }
});

const internal = new LazySymbolScope();


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

    Object.seal(this);
  }

  // const private = new LazySymbolScope();
  // XXX: Shhould we only publish this on the root?
  // [private.setProps](props) {
  setProps(props) {
    this.props = Object.freeze(Object.assign({}, this.props, props));

    this.rendered = this.constructor.render(props);

    if (this.element_) {
      this.element_.textContent = '';
      this.element_.appendChild(this.rendered.fragment())
    }
  }

  element() {
    if (this.element_ === null) {
      this.element_ = document.createElement('bester-component');
      this.element_.classList.add(...this.classNames);
      this.element_.appendChild(this.rendered.fragment());
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
