import HTML from '/html.js';

(main => {const done = Promise.resolve().then(_ => main(done));})(async done => {
  done.then(
    _ => document.querySelector('#premain-message').remove(),
    error => {
      document.querySelector('#premain-message').textContent = String(error.stack);
      throw error;
    });

  const hostname = document.location.host;
  const projectName = hostname.match(/^[a-z0-9\-]+\.glitch\.me$/) ? hostname.split('.')[0] : null;
  const title = `${projectName || 'speedrun'}.glitch.me`;
  
  const apiRoot = 'https://www.speedrun.com/api/v1';

  document.title = title;

  const output = document.querySelector('#main');
  const out = child => output.appendChild(child);

  const heading = HTML.element`<h1>${title}</h1>`;
  if (projectName) {
    heading.appendChild(HTML.fragment` (<a href="${`https://glitch.com/edit/#!/${projectName}`}">view source</a>)`);
  }
  out(heading);
  
  const response = await fetch(`${apiRoot}/games/wc2`);
  const data = await response.json();

  out(HTML.element`<pre>${JSON.stringify(data, null, 2)}</pre>`);
});
