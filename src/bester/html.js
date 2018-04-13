// HTML templating!

import {aarray} from '/assets/bester/utils.js';


ADD_A_DEFAULT_CONTENT_SECURITY_POLICY: {
  // The HTML`${}` escaping is non-contextual, so it can't protect against
  // some types of unsafe interpolation, such as in script tag content.
  // A Content Security Policy can help. If the document doesn't appear to
  // have a policy defined, we add a strict policy as a default.

  try {
    new Function(''); 
  } catch (error) {
    break ADD_A_DEFAULT_CONTENT_SECURITY_POLICY;
  }

  if (document.querySelector('meta[http-equiv="Content-Security-Policy]')) {
    // This can just be an empty <meta http-equiv="Content-Security-Policy">.
    break ADD_A_DEFAULT_CONTENT_SECURITY_POLICY;
  }

  const csp = document.createElement('meta');
  csp.metaEquiv = 'Content-Security-Policy';
  csp.content = `default-src 'self'; img-src *`;
  document.head.appendChild(csp);
}


export const escape = text => {
    return String(text)
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;')
      .replace(/"/g, '&quot;')
      .replace(/'/g, '&#39;');
};


class ThisShouldNeverHappenError extends Error {
  constructor(message = "ಠ_ಠ") {
    super(message);
    debugger;
  }
};


const placeholderClassSuffixes = crypto.getRandomValues(new Uint32Array(256));


export class HTMLPieces {
  constructor(pieces) {
    for (const piece of pieces)
      if (!((typeof piece === 'string') || (piece && typeof piece.then === 'function') || piece instanceof HTMLPieces))
        throw new TypeError();
    
    // Flatten any HTMLPieces that are passed in synchronously.
    this.pieces = [].concat(...pieces.map(
        piece => (piece instanceof HTMLPieces) ? piece.pieces : [piece]));

    for (const piece of this.pieces)
      if (!((typeof piece === 'string') || (typeof piece.then === 'function')))
        throw new ThisShouldNeverHappenError();

    this.async = this.pieces.some(piece => typeof piece.then === 'function');

    Object.freeze(this.pieces);
    Object.freeze(this);
  }

  static from(content) {
    if (content instanceof HTMLPieces) {
      return content;
    } else if (typeof content === 'string') {
      return new HTMLPieces([escape(content)]);
    } else if (content === undefined) {
      return new HTMLPieces([]);
    } else if (content === null) {
      return new HTMLPieces(['null']);
    } else if (typeof content === 'function') {
      return HTMLPieces.from(content());
    } else if (content[Symbol.asyncIterator]) {
      const iterator = content[Symbol.asyncIterator]();
      const doNext = () => {
        return HTMLPieces.from(iterator.next().then(({value, done}) => {
          if (done) return;
          return [value, doNext()];
        }));
      };
      return doNext();
    } else if (content[Symbol.iterator]) {
      const pieces = [];
      for (const item of content) {
        pieces.push(HTMLPieces.from(item));
      }
      return new HTMLPieces(pieces);
    } else if (typeof content.then === 'function') {
      return new HTMLPieces([content.then(HTMLPieces.from)]);
    } else {
      return new HTMLPieces([String(content)]);
    }
  }
  
  static literal(strings, ...substitutions) {
    const wrappedSubstitutions = substitutions.map(HTMLPieces.from);
    const pieces = [];
    for (let i = 0; i < strings.length; i++) {
      pieces.push(strings[i]);
      if (i < wrappedSubstitutions.length) {
        pieces.push(wrappedSubstitutions[i]);
      }
    }
    return new HTMLPieces(pieces);
  }
  
  async *flatResolvedPieces() {
    for (const piece of this.pieces) {
      const syncPiece = await piece;

      if (typeof syncPiece === 'string') {
        yield syncPiece;
      } else if (syncPiece instanceof HTMLPieces) {
        for await (const subPiece of syncPiece.flatResolvedPieces()) {
          yield syncPiece;
        }
      } else {
        throw new ThisShouldNeverHappenError(
            "some awaited piece in this.pieces is not a string nor an HTMLPieces");
      }
    }
  }

  string() {
    if (this.async) {
      return aarray(this.flatResolvedPieces()).then(pieces => pieces.join);
    } else {
      return this.pieces.join(''); 
    }
  }
  
  // TODO: do we have any reason to acutally care about returning
  // a fragment, and not returning a PromiseComment? If not, then
  // let's just switch so we don't need this weird double-return and
  // additional method below. But I guess this kind-of ruins the sync case?
  // Can we still detect and distinguish? We should be able to.
  fragmentAndDone() {
    const stringPieces = [];
    const placeholders = new Map();
    
    for (const piece of this.pieces) {
      if (typeof piece === 'string') {
        stringPieces.push(piece);
      } else if (typeof piece.then === 'function') {
        const i = placeholders.size + 1;

        const suffix = placeholderClassSuffixes[i % placeholderClassSuffixes.length];
        const placeholderClassName =`placeholder-${i}-${suffix}`;
        stringPieces.push(`<script type="text/plain" class="${placeholderClassName}"></script>`);
        
        placeholders.set(`script.${placeholderClassName}`, piece);
      } else {
        throw new ThisShouldNeverHappenError(
            "some piece in this.pieces is not a string nor a Promise");
      }
    }
    
    const html = stringPieces.join('');

    const doc = (new DOMParser).parseFromString(`<!doctype html><body><template>${html}</template>`, 'text/html');
    if (doc.body.children.length !== 1) {
      throw new Error(`found ${doc.body.children.length} elements expecting 1 in wrapper document body`);
    }
    const result = doc.body.firstElementChild.content;

    const blockers = [];
    for (const [selector, replacement] of placeholders) {
      const matches = result.querySelectorAll(selector);
      if (matches.length !== 1) {
        throw new SyntaxError(
          "Probably got async element value when not expecting element. " +
          "Maybe you put a promise for an attribute value. That's not supported. " +
          "You can only put async values where elements are allowed.");
      }

      const replacementFragmentAndDone = replacement.then(html => html.fragmentAndDone());
      const subFragment = replacementFragmentAndDone.then(x => x[0]);
      const done = replacementFragmentAndDone.then(x => x[1]);
      matches[0].replaceWith(new PromiseComment(subFragment));
      blockers.push(done);
    }

    return [result, Promise.all(blockers).then(() => result)];
  }

  fragment() {
    const [fragment, done] = this.fragmentAndDone();
    return fragment;
  }

  async element() {
    const [fragment, done] = this.fragmentAndDone();
    await done;
    if (fragment.children.length !== 1)  {
      throw new Error(`found ${fragment.children.length} elements expecting 1`);
    }
    return fragment.firstElementChild;
  }
};


export const HTML = (...args) => HTMLPieces.literal(...args);

HTML.from = HTMLPieces.from;

HTML.fragment = (...args) => HTMLPieces.literal(...args).fragment();

HTML.element = (...args) => HTMLPieces.literal(...args).element();

HTML.string = (...args) => HTMLPieces.literal(...args).string();

HTML.prototype = HTMLPieces.prototype;

export default HTML;


export class PromiseComment extends Comment {
  constructor(content) {
    super(`Promise (pending)`);
    
    const promise = (async () => {
      try {
        const result = await content;
        this.replaceWith(result);
        this.data = `Promise (fulfilled)`;
        return result;
      } catch (error) {
        this.data = `Promise (rejected): ${error}\n${error.stack}\n`;
        throw error;
      }
    })();

    this.then = (...args) => promise.then(...args);
  }
};
