// bester.Component subclasses render an Object of props into a bester.HTML
// instance, and add some class-name-based CSS classes to a prefix placeholder
// element facilitate styling and dedbugging.

import {HTML} from './html.js';
import {style} from './style.js';
import {document} from './deps.js';
import {LazySymbolScope} from './utils.js';


const internal = new LazySymbolScope('internal ');
const {
  classNames,
  props,
  element,
  renderedHTML,
  getElement,
  setProps,
  onElementCreated,
  onElementRendered
} = internal;


export class Component {
  static of(props) {
    return new this(props);
  }

  constructor(props = {}) {
    const classes = [];
    let currentClass = this.constructor;
    while (currentClass && currentClass.name && currentClass !== Component) {
      classes.push(currentClass);
      currentClass = Object.getPrototypeOf(currentClass);
    }
    this[classNames] = classes.map(c => c.name);

    this[internal.props] = {};
    this[element] = null;
    this[renderedHTML] = null;

    Object.seal(this);
    // We don't freeze because we do allow the props to be changed in some cases.

    this[setProps](props || {});
  }

  // by default, components should just pass through their contents, but we expect
  // many subclasses to override this.
  get style() {
    return style({
      'display': 'contents'
    });
  }

  get props() {
    return this[props];
  }

  render(props) {
    throw new Error("not implemented"); 
  }

  get rendered() {
    return this[renderedHTML].done().then(() => this);
  }

  [HTML.fromThis]() {
    return HTML`<bester-component class="${this[classNames].join(" ")}" ${this.style}>${this[renderedHTML]}</bester-component>`;
  }

  [getElement]() {
    if (!this[element]) {
      console.debug(`🔨 Creating element for ${this[classNames][0]}.`);
      this[element] = document.createElement('bester-component');
      this[element].classList.add(...this[classNames]);
      this[element].setAttribute('style', this.styleAttributeValue);
      this[element].appendChild(this[renderedHTML].fragment());
      
      Promise.resolve().then(() => this[onElementCreated]());
    }
    return this[element];
  }

  // Note that this replaces all existing props, not just named ones.
  [setProps](props) {
    this[internal.props] = Object.freeze(Object.assign({}, props));

    this[renderedHTML] = HTML.from(this.render(props));
    if (this[element]) {
      this[element].textContent = '';
      this[element].appendChild(this[renderedHTML].fragment());
      
      let renderedProps = this.props;
      this[renderedHTML].done().then(result => {
        if (this.props !== renderedProps) {
          return;
        }

        console.debug(`🍹 Rendered element contents for ${this[classNames][0]}.`);
        Promise.resolve().then(() => this[onElementRendered](this.element));
      });
    }
  }

  [onElementCreated]() {}

  [onElementRendered]() {}
}


// FOR NOW, only a root component allows its props to be changed, so everything must be re-rendered at once.
// Maybe we could call this an Application, and give it some of the router logic too, accepting a location object?
// or maybe just do that in our subclass, not here.
// anyway this is the only place the rendered events are exposed because it's the only place they'll work
// because otherwise this.element doesn't exist or isn't actually being used.
export class RootComponent extends Component {
  get element() {
    return this[getElement]();
  }

  get props() {
    return this[props];
  }

  set props(props) {
    this[setProps](props);
  }
  
  [onElementCreated]() {
    this.onElementCreated();
  }

  onElementCreated() {
    // called after an associated element is created, if ever.
    // XXX: this doesn't wait for it to be attached to the document? so it better use currentDoc or somehting.
  }
  
  [onElementRendered]() {
    this.onElementRendered();
  }

  onElementRendered() {
    // called after a render completes, if its props are still current.
  }

  // Consider adding a new common parent of Component types, instead of violate the Liskov substitution principle by killing this method.
  // If we did that, maybe we should stop making element() lazy, and make it part of setting props on an ElementComponent.
  // maybe rename to not say "root", to allow for eventuall nested element-associated components, but that requires major templating improvements.
  // [HTML.fromThis]() {
  //   throw new Error("cannot convert a RootComponent to HTML directly, you should just use this.element instead");
  // }
}
