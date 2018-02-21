export const main = async () => {
  await 0;

  result.then(
    _ => document.querySelector('#premain-message').remove(),
    error => document.querySelector('#premain-message').textContent = String(error));

  const output = document.querySelector('#main');

  const {hostname} = document.location;
  const projectName = hostname.matches(/^[a-z0-9\-]+\.glitch\.io/) ? hostname.split('.')[0] : null;
  const heading = Object.assign(document.createElement('h1'), {textContent: `${projectName || 'speedrun'}.glitch.io`});
  
  if (projectName) {
    heading.appendChild(document.createTextNode(' ('));
    heading.appendChild(Object.assign(document.createElement('a'), {href: `https://glitch.com/edit/#!/${projectName}`}));
    heading.appendChild(document.createTextNode(')'));
  }
};

const result = main();
