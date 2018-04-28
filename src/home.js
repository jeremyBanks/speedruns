import HTML from '/assets/bester/html.js';
import {Component} from '/assets/bester/component.js';
import {style} from '/assets/bester/style.js';


export class HomeBody extends Component {
  render({}) {
    return HTML`
      <p>This site lets you compare personal and world record progressions over time.</p>

      <p>
        Example URLs:
      </p>

<pre>
      
      <ul>
        <li><code><a href="/@banks">/@banks</a></code></li>
        <li><code><a href="/wc2+wc2btdp/@zpr">/wc2+wc2btdp/@zpr</a></code></li>
        <li><code><a href="/smwext/world-1/@banks">/smwext/world-1/@banks</a></code></li>
        <li><code><a href="/smwext/@QuiteSuperMario#level-xd1rnr7k">/smwext/@QuiteSuperMario#level-xd1rnr7k</a></code></li>
      </ul>
    `;
  }
  
  get style() {
    return style({text: {align: 'left'}});
  }
}
