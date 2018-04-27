import HTML from '/assets/bester/html.js';
import {Component} from '/assets/bester/component.js';
import {style} from '/assets/bester/style.js';


export class HomeBody extends Component {
  render({}) {
    return HTML`
      <p>This site lets you compare personal and world record progressions over time.</p>

      <p>
        Examples:
      </p>

      <ul>
        <li><code><a href="/wc2+wc2btdp/@banks">/wc2+wc2btdp/@banks</a></code></li>
        <li><code><a href="/smwext/@QuiteSuperMario#level-xd1rnr7k">/smwext/@QuiteSuperMario#level-xd1rnr7k</a></code></li>
      </ul>
    `;
  }
  
  get style() {
    return style({text: {align: 'left'}});
  }
}
