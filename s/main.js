import HTML from '/html.js';

(f => (async () => 0) f())(async done => {
  
});

export const main = async () => {
  await 0;

  main.result.then(
    _ => document.querySelector('#premain-message').remove(),
    error => {
      document.querySelector('#premain-message').textContent = String(error.stack);
      throw error;
    });

  const hostname = document.location.host;
  const projectName = hostname.match(/^[a-z0-9\-]+\.glitch\.me$/) ? hostname.split('.')[0] : null;
  const title = `${projectName || 'speedrun'}.glitch.me`;

  document.title = title;

  const output = document.querySelector('#main');

  const heading = HTML.element`<h1>${title}</h1>`;
  if (projectName) {
    heading.appendChild(HTML.fragment` (<a href="${`https://glitch.com/edit/#!/${projectName}`}">view source</a>)`);
  }
  output.appendChild(heading);
};

main.result = main();
