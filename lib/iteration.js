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


export const devAwaitDeep = async (rootValue, maxTimeout = 4096) => {
  const timeout = new Promise(resolve => setTimeout(() => {
    resolve({'⏱️': 'Pending Promise'});
  }, maxTimeout));

  const awaitDeepEach = async value => {
    value = await Promise.race([timeout, value]).catch(error => ({
      '⚠️': 'Rejected Promise',
      message: String(error),
    }));

    if (typeof value !== 'object' || value === null) {
      return value;
    }

    if (Array.isArray(value)) {
      const result = [];
      for (const [key, child] of value.entries()) {
        result.push(await awaitDeepEach(child));
      }
      return result;
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
