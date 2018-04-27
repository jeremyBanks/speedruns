import HTML from '/assets/bester/html.js';
import {Component} from '/assets/bester/component.js';
import {style} from '/assets/bester/style.js';


export class Header extends Component {
  get style() {
    return style({
      text: {align: 'left'}
    });
  }
  
  get headerTextStyle() {
    return style({
      display: 'inline',
      border: {
        bottom: {
          _: '2px solid #000',
          left: {radius: '10px 6px'},
          right: {radius: '32px 2px'},
        }
      },
      position: 'relative',
      top: '-7px',
      padding: {right: '4px'}
    });
  }
  
  get headerTextInnerStyle() {
    return style({
      position: 'relative',
      top: '7px'
    });
  }
  
  get linksStyle() {
    return style({
      float: 'right',
      margin: {top: '4px'},
      font: {size: '12px'},
      line: {height: '16px'},
      text: {align: 'right'}
    });
  }

  render({currentHost, currentProject}) {
    return HTML`<header>
      <h1 ${this.headerTextStyle}><span ${this.headerTextInnerStyle}>
        <img src="/assets/icon.png">
        <a href="//${currentHost}/">${currentHost}</a>
      <span></h1>

      ${currentProject && HTML`
        <nav class="links" ${this.linksStyle}>
          <a href="${`https://glitch.com/edit/#!/${currentProject}?path=src/client.js`}">edit on Glitch</a><br />
        </nav>
      `}
    </header>`;
  }
}


export class Footer extends Component {
  get style() {
    return style({
      font: {size: '0.75em'},
      margin: {top: '128px'}
    });
  }

  render({}) {
    return HTML`<footer>
      This site displays data from <a href="https://www.speedrun.com/about">speedrun.com</a>,
      used under <a href="https://creativecommons.org/licenses/by-nc/4.0/">the CC BY-NC license</a> and
      loaded from <a href="https://github.com/speedruncomorg/api/blob/master/version1/README.md#readme">their API</a>.
    </footer>`;
  }
}