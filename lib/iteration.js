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
// to every promise before it's awaited.
export const awaitDeep = async (asyncJsonables, transformation = undefined) {
    
};

awaitDeep.DEV = promise => new Promise((resolve, reject) => {
  
});

  return promise.then(result => ({
    '🔵': 'Promise',
    value,
  }), error => ({
    '🔴': 'Promise',
    error,
  }));
};
