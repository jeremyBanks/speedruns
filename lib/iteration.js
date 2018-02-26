export const zip = (...args) => {
  // like Python's itertools.zip_longest
  // from stackoverflow.com/a/10284006
  const longest = args.reduce((a, b) => a.length > b.length ? a : b, []);
  return longest.map((_, i) => args.map(array => array[i]));
};

export const aarray = async (iterable) => {
  const values = [];
  for await (const value of iterable) {
    values.push(value);
  }
  return values;
};
