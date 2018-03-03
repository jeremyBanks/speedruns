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

// Converts a deep structure of potentially-async JSON-serializable values
// into a single sync value, applying an optional transformation function
// to every value before it's awaited.
// TODO: consider async iterator support? if we use it.
export const awaitDeep = async (asyncJsonable, transformation = awaitDeep.DEV, rootKeyName = undefined) {
  asyncJsonable = await transformation(rootKeyName, asyncJsonable);

  if (typeof asyncJsonable !== 'object' || asyncJsonable === null) {
    // not a collection, nothing to do after awaiting.
    return asyncJsonable;
  }

  if (Array.isArray(asyncJsonable)) {
    for (const value of asyncJsonable) {
      
    }
  } else {
    for (const key of Object.keys(asyncJsonable)) {
      const value = asyncJsonable[value];
    }
  }
};

awaitDeep.DEV = (key, value) => new Promise(resolve => {
  Promise.resolve(value).then(resolve, error => resolve({
    '🔴': 'Rejected Promise',
    message: String(error),
  }));

  const maxWait = 500;  
  setTimeout(() => resolve({
    '⚪': 'Pending Promise',
    message: `still pending after ${maxWait}ms`,
  }, maxWait);
});
