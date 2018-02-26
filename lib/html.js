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


const placeholderTagName =
    'placeholder-' + crypto.getRandomValues(new Uint32Array(4)).join('-');
const placeholderClassSuffixes = crypto.getRandomValues(new Uint32Array(256));

customElements.define(placeholderTagName, HTMLElement);


export class HTMLPieces {
  constructor(pieces) {
    // Flatten any HTMLPieces that are passed in synchronously.
    this.pieces = [].concat(...pieces.map(
        piece => (typeof piece === 'string') ? [piece] : piece.pieces));
    
    Object.freeze(this.pieces);
    Object.freeze(this);
  }

  static from(content) {
    if (content instanceof HTMLPieces) {
      return content;
    } else if (typeof content === 'string') {
      return new HTMLPieces([escape(content)]);
    } else if (content === undefined) {
      return new HTMLPieces([''])
    } else if (content === null) {
      return new HTMLPieces(['null']);
    } else if (typeof content === 'function') {
      return HTMLPieces.from(content());
    } else if (content[Symbol.asyncIterator]) {
      return HTMLPieces.from(async () => {
        // TODO: this shouldn't need to wait for the whole list
        const pieces = [];
        for await (const item of content) {
          pieces.push(...HTMLPieces.from(item).pieces);
        }
        return pieces;
      });
    } else if (content[Symbol.iterator]) {
      const pieces = [];
      for (const item of content) {
        pieces.push(...HTMLPieces.from(item).pieces);
      }
      return new HTMLPieces(pieces);
    } else if (typeof content.then === 'function') {
      // this results in nested pieces. don't we want to avoid that?
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
  
  async *flatStringPieces() {
    for (const piece of this.pieces) {
      if (typeof piece === 'string') {
        yield piece;
      } else if (piece instanceof HTMLPieces) {
        for await (const subPiece of piece.flatStringPieces()) {
          yield piece;
        }
      } else {
        throw new ThisShouldNeverHappenError(
            "some piece in this.pieces is not a string nor an HTMLPieces");
      }
    }
  }

  async string() {
    const stringPieces = [];
    for await (const piece of this.flatStringPieces()) {
      stringPieces.push(piece);
    }
    return stringPieces.join('');
  }
  
  fragmentAndDone() {
    const stringPieces = [];
    const placeholders = new Map();
    
    for (const piece of this.pieces) {
      if (typeof piece === 'string') {
        stringPieces.push(piece);
      } else if (piece instanceof HTMLPieces) {
        const i = placeholders.count + 1;

        const suffix = placeholderClassSuffixes[i % placeholderClassSuffixes.length];
        const placeholderClassName =`placeholder-${i}-${suffix}`;
        pieces.push(`<${placeholderTagName} class="${placeholderClassName}"></${placeholderTagName}>`);
        
        placeholders.set(`.${placeholderClassName}`, piece);
      } else {
        throw new ThisShouldNeverHappenError(
            "some piece in this.pieces is not a string nor an HTMLPieces");
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
        throw new SyntaxError("got async element value but not expecting element");
      }
      const [subFragment, done] = replacement.fragmentAndDone();
      blockers.push(done);
      matches[0].replaceWith(subFragments);
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

HTML.fragment = (...args) => HTMLPieces.literal(...args).fragment();

HTML.element = async (...args) => HTMLPieces.literal(...args).element();

HTML.string = async (...args) => HTMLPieces.literal(...args).string();

HTML.prototype = HTMLPieces.prototype;

export default HTML;


export class PromiseComment extends Comment {
  constructor(content) {
    super(`Promise (pending)`);
    
    const promise = (async () => {
      try {
        const result = await this.content;
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
