// bester.Component subclasses render an Object of props into a bester.HTML
// instance, and add some class-name-based CSS classes to a prefix placeholder
// element facilitate styling and dedbugging.

import {HTML, HTML.fromThis} from '/assets/bester/html.js';
import {LazySymbolScope} from '/assets/bester/utils.js';


const internal = new LazySymbolScope('internal ');


export class Component {
  constructor(props = null) {
    this.classNames = [];
    let currentClass = this.constructor;
    while (currentClass && currentClass.name && currentClass !== Component) {
      this.classNames.push(currentClass.name);
      currentClass = Object.getPrototypeOf(currentClass);
    }

    this.element_ = null;
    this.props = null;

    Object.seal(this);
    
    this[internal.setProps](props);
  }

  static render(props) {
    throw new Error("not implemented"); 
  }

  [HTML.fromThis]() {
    return HTML`<bester-component class="${this.classNames.join(" ")}">${this.rendered}</bester-component>`;
  }

  [internal.getElement]() {
    if (this.element_ === null) {
      this.element_ = document.createElement('bester-component');
      this.element_.classList.add(...this.classNames);
      this.element_.appendChild(this.rendered.fragment());
    }
    return this.element_;
  }

  [internal.setProps](props) {
    this.props = Object.freeze(Object.assign({}, this.props, props));

    this.rendered = this.constructor.render(props);

    if (this.element_) {
      this.element_.textContent = '';
      this.element_.appendChild(this.rendered.fragment())
    }
  }
}

// FOR NOW, only a root component allows its props to be changed, so everything must be re-rendered at once.
export class RootComponent extends Component {
  getElement() {
    return [internal.getElement]();
  }

  setProps(props) {
    return [internal.setProps](props);
  }
}
