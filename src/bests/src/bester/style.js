import {HTML} from './html.js';

// A set of CSS style property values, which may also be used as an HTML string style attribute.
export class Style {
  constructor(props) {
    Object.assign(this, props);
    Object.freeze(this);
  }
  
  [HTML.fromThis]() {
    return HTML`style="${Style.attrValue(this)}"`
  }

  static attrValue(data, propPrefix = '') {
    return Object.keys(data).map(key => {
      const value = data[key];
      if (key === '_') { key = ''; }
      const propName = [propPrefix, key].filter(Boolean).join('-');
      if (typeof value === 'string') {
        return `${propName}: ${value};`
      } else if (typeof value === 'number' && Number.isFinite(value)) {
        return `${propName}: ${value};`
      } else if (value && typeof value === 'object') {
        return this.attrValue(value, propName);
      } else {
        throw new TypeError("css value has unexpected type");
      }
    }).join(' ');
  }  
}


export const style = data => new Style(data);