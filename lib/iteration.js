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


const devAwaitDeep = async (rootValue, timeout = 4096) => {
  const rootTimeout = new Promise(resolve => setTimeout(() => {
    resolve({
      '⚪': 'Pending Promise',
      message: `still pending after ${maxWait}ms`,
    });
  }, timeout));

  
};


const doAwaitDeep = async (asyncJsonable, transformation = awaitDeep.DEV, rootKeyName = undefined) => {
  asyncJsonable = await transformation(rootKeyName, asyncJsonable);

  if (typeof asyncJsonable !== 'object' || asyncJsonable === null) {
    return asyncJsonable;
  }

  if (Array.isArray(asyncJsonable)) {
    const result = [];
    for (const [key, value] of asyncJsonable.entries()) {
      result.push(await awaitDeep(transformation(key, value), transformation, key));
    }
    return result;
  } else {
    const result = {};
    for (const [key, value] of Object.entries(asyncJsonable)) {
      result[key] = await awaitDeep(transformation(key, value), transformation, key);
    }
    return result;
  }
};

  // XXX: This really needs a top-level timeout that propogates a cancellation, nothing else.
  
awaitDeep.DEV = (key, value) => new Promise(resolve => {
  Promise.resolve(value).then(resolve, error => resolve({
    '🔴': 'Rejected Promise',
    message: String(error),
  }));

  const maxWait = 2048;  
  setTimeout(() => resolve({
    '⚪': 'Pending Promise',
    message: `still pending after ${maxWait}ms`,
  }), maxWait);
});
