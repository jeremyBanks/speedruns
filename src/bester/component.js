// bester.Component subclasses render an Object of props into a bester.HTML
// instance, and add some class-name-based CSS classes to a prefix placeholder
// element facilitate styling and dedbugging.

import {HTML} from '/assets/bester/html.js';
import {LazySymbolScope} from '/assets/bester/utils.js';


// We want it to be slightly inconvenient to violate our interfaces.
const internal = new LazySymbolScope('internal ');


export class Component {
  constructor(props = null) {
    const classNames = [];
    let currentClass = this.constructor;
    while (currentClass && currentClass.name && currentClass !== Component) {
      classNames.push(currentClass.name);
      currentClass = Object.getPrototypeOf(currentClass);
    }
    this[internal.classNames] = classNames;

    this[internal.props] = null;
    this[internal.element] = null;
    this[internal.renderedHTML] = null;

    Object.seal(this);

    this[internal.setProps](props);
  }

  get props() {
    return this[internal.props];
  }

  static render(props) {
    throw new Error("not implemented"); 
  }

  get rendered() {
    return this[internal.renderedHTML].done().then(() => this);
  }

  [HTML.fromThis]() {
    return HTML`<bester-component class="${this[internal.classNames].join(" ")}">${this[internal.renderedHTML]}</bester-component>`;
  }

  [internal.getElement]() {
    if (!this[internal.element]) {
      console.debug('🔨 Creating element for ');
      this[internal.element] = document.createElement('bester-component');
      this[internal.element].classList.add(...this[internal.classNames]);
      this[internal.element].appendChild(this[internal.renderedHTML].fragment());
      
      Promise.resolve().then(() => this[internal.onElementCreated]());
    }
    return this[internal.element];
  }

  [internal.setProps](props) {
    this[internal.props] = Object.freeze(Object.assign({}, this.props, props));

    this[internal.renderedHTML] = HTML.from(this.constructor.render(props));
    if (this[internal.element]) {
      this[internal.element].textContent = '';
      this[internal.element].appendChild(this[internal.renderedHTML].fragment());
      
      let renderedProps = this.props;
      this[internal.renderedHTML].done().then(result => {
        if (this.props !== renderedProps) {
          return;
        }

        Promise.resolve().then(() => this[internal.onElementRendered](this.element));
      });
    }
  }

  [internal.onElementCreated]() {}

  [internal.onElementRendered]() {}
}


// FOR NOW, only a root component allows its props to be changed, so everything must be re-rendered at once.
// Maybe we could call this an Application, and give it some of the router logic too, accepting a location object?
// or maybe just do that in our subclass, not here.
// anyway this is the only place the rendered events are exposed because it's the only place they'll work
// because otherwise this.element doesn't exist or isn't actually being used.
export class RootComponent extends Component {
  get element() {
    return this[internal.getElement]();
  }

  set props(props) {
    this[internal.setProps](props);
  }
  
  [internal.onElementCreated]() {
    this.onElementCreated();
  }

  onElementCreated() {
    // called after an associated element is created, if ever.
  }
  
  [internal.onElementRendered]() {
    this.onElementRendered();
  }

  onElementRendered() {
    // called after a render completes, if its props are still current.
  }
}
