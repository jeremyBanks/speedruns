export const main = async () => {
  result.then(
    _ => document.querySelector('#main-loading').remove(),
    error => document.querySelector('#main-loading').textContent = String(error));

  const output = document.querySelector('#main');
  
  output.foo.bar;
};

const result = Promise.resolve().then(main);
