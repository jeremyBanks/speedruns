export const main = async () => {
  await 0;

  onLoad(main.result);

  const output = document.querySelector('#main');

  const hostname = document.location.host;
  const projectName = hostname.match(/^[a-z0-9\-]+\.glitch\.me$/) ? hostname.split('.')[0] : null;
  const title = `${projectName || 'speedrun'}.glitch.me`;

  document.title = title;

  const heading = Object.assign(document.createElement('h1'), {textContent: title});
  if (projectName) {
    heading.appendChild(document.createTextNode(' ('));
    heading.appendChild(Object.assign(document.createElement('a'), {
      href: `https://glitch.com/edit/#!/${projectName}`,
      textContent: 'view source'}));
    heading.appendChild(document.createTextNode(')'));
  }
  output.appendChild(heading);

  hello.world;
};

const cleanUp = async result => {
  result.then(
    _ => document.querySelector('#premain-message').remove(),
    error => {
      document.querySelector('#premain-message').textContent = String(error.stack);
      throw error;
    });
};

main.result = main();
