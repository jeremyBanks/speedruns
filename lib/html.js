const HTMLRuntimeType = class HTML extends String {};


// Our primary HTML`...` escaping tag function returns our HTML String objects.
const htmlTag = (strings, ...substitutions) => {
  const escapedFlattenedSubstitutions =
    substitutions.map(s => [].concat(s).map(HTML.escape).join(''));
  const pieces = [];
  for (let i = 0; i < strings.length; i++) {
    pieces.push(strings[i]);
    if (i < escapedFlattenedSubstitutions.length) {
      pieces.push(escapedFlattenedSubstitutions[i]);
    }
  }
  return new HTMLRuntimeType(pieces.join(''));
};

// HTML.string`...` converts the interpolated HTML to a primitive string.
const stringTag = (strings, ...substitutions) => {
  return HTML(strings, ...substitutions).toString();
};

// HTML.fragment`...` safely parses the interpolated HTML as an HTMLFragment.
const fragmentTag = (strings, ...substitutions) => {
  const html = HTML(strings, ...substitutions);
  const doc = (new DOMParser).parseFromString(`<!doctype html><body><template>${html}</template>`, 'text/html');
  return (doc.querySelector('template')).content;
};

// HTML.element`...` returns the single child element in the interpolated HTML or throws if more or less than one.
const elementTag = (strings, ...substitutions) => {
  const html = HTML(strings, ...substitutions);
  const doc = (new DOMParser).parseFromString(`<!doctype html><body><template>${html}</template>`, 'text/html');
  if (doc.body.children.length !== 1) {
    throw new Error(`found ${doc.body.children.length} elements expecting 1 in wrapper document body`);
  }
  const templateContent = doc.body.firstElementChild.content;
  if (templateContent.children.length !== 1)  {
    throw new Error(`found ${templateContent.children.length} elements expecting 1 in ${html}`);
  }
  return templateContent.firstElementChild;
};

// HTML.escape(...) will return an HTML object unchanged, but convert any 
// other value to a string and escape it to create a new HTML object.
const htmlEscape = (text) => {
  if (text instanceof HTMLRuntimeType) {
    return text;
  }
  if (text === undefined) {
    text = ''; 
  }
  return new HTMLRuntimeType(
      String(text)
          .replace(/&/g, '&amp;')
          .replace(/</g, '&lt;')
          .replace(/>/g, '&gt;')
          .replace(/"/g, '&quot;')
          .replace(/'/g, '&#39;'));
};

// HTML.unsafeRaw(...) exposes our private constructor for unfortunate misuse.
const htmlUnsafeRaw = (s) => {
  return new HTMLRuntimeType(s);
};

// We define our non-type HTML function and namespace here.
export const HTML = Object.assign(
  htmlTag, {
    string: stringTag,
    fragment: fragmentTag,
    element: elementTag,
    escape: htmlEscape,
    unsafeRaw: htmlUnsafeRaw
  }
);

// Make HTML available as a default export in addition to a named one.
export { HTML as default };
