// like Python's itertools.zip_longest
// from stackoverflow.com/a/10284006
export const zip = (...args) => {
  const longest = args.reduce((a, b) => a.length > b.length ? a : b, []);
  return longest.map((_, i) => args.map(array => array[i]));
};

// converts an async iterator into a sync array.
export const aarray = async (iterable) => {
  const values = [];
  for await (const value of iterable) {
    values.push(value);
  }
  return values;
};


export const devAwaitDeep = async (rootValue, forcedTimeout = new Promise(() => 0), baseTimeout = 16384) => {
  const timeout = Promise.race([forcedTimeout.then(() => ({
    '⏱️': 'Pending Promise',
    message: `still pending after forced timeout`,
  })), new Promise(resolve => setTimeout(() => {
    resolve({
      '⏱️': 'Pending Promise',
      message: `still pending after ${baseTimeout}ms timeout`,
    });
  }, baseTimeout))]);

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
