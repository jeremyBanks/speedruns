export const inBrowser = typeof window === 'object' && typeof document === 'object';


// like Python's itertools.zip_longest
// from stackoverflow.com/a/10284006
export const zip = (...args) => {
  const longest = args.reduce((a, b) => a.length > b.length ? a : b, []);
  return longest.map((_, i) => args.map(array => array[i]));
};

// Loops up a series of property names starting from the subject, until one is null or undefined.
export const nProps = (subject, ...propNames) => {
  let destination = subject;
  for (const propName of propNames) {
    if (destination === null || destination === undefined) {
      break;
    }
    destination = destination[propName];
  }
  return destination;
};


// converts an async iterator into a sync array.
export const aarray = async (iterable) => {
  const values = [];
  for await (const value of iterable) {
    values.push(value);
  }
  return values;
};


export const compareAll = (...comparisons) => (a, b) =>
    comparisons.reduce((m, f) => m || f(a, b), 0);


export const compareDefault = (a, b) =>
    a < b ? -1 :
    a > b ? 1 :
    0;


export class LazySymbolScope {
  constructor(prefix = '') {
    return new Proxy(this, {
      get(self, key, proxy) {
        let value = Reflect.get(...arguments);
        if (!value) {
          value = Symbol(`${prefix}${key}`);
          self[key] = value;
        }
        return value;
      }
    }); 
  }
}


export const devAwaitDeep = async (rootValue, forcedTimeout = new Promise(() => {}), maxTimeout = 0x10000) => {
  const timeout = Promise.race([forcedTimeout.then(() => ({
    '⏱️': 'Pending Promise',
    message: `still pending after forced timeout`,
  })), new Promise(resolve => setTimeout(() => {
    resolve({
      '⏱️': 'Pending Promise',
      message: `still pending after ${maxTimeout}ms timeout`,
    });
  }, maxTimeout))]);

  const awaitDeepEach = async value => {
    if (typeof value !== 'object' || value === null) {
      // primitive sync base case.
      return value;
    }
    
    value = await Promise.race([value, timeout]).catch(error => ({
      '⚠️': error && error.name || 'Error',
      message: error && error.message,
      stack: error && error.stack.split(/\n/g),
    }));

    if (typeof value !== 'object' || value === null) {
      // primitive async case.
      return value;
    }

    if (Array.isArray(value) || value[Symbol.iterator]) {
      const result = [];
      for (const [key, child] of value.entries()) {
        result.push(await awaitDeepEach(child));
      }
      return result;
    } else if (value[Symbol.asyncIterator]) {
      const children = [];
      const done = (async() => {
        for await (const child of value) {
          children.push(child);
        }
      })();
      return Promise.race([
        done.then(() => awaitDeepEach(children)),
        timeout.then(async() => ({
          '⏱️': 'Non-Exhausted Iterator',
          message: `still not exhausted after timeout`,
          children: await awaitDeepEach(children),
        }))
      ]);
    } else {
      const result = {};
      for (const [key, child] of Object.entries(value)) {
        result[key] = await awaitDeepEach(child);
      }
      return result;
    }
  };
  
  return awaitDeepEach(rootValue);
};