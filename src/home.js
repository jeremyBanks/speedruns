import HTML from '/assets/bester/html.js';
import {Component} from '/assets/bester/component.js';
import {style} from '/assets/bester/style.js';


export class HomeBody extends Component {
  get urlStyle() {
    return style({
      text: {decoration: 'underline'}
    });
  }

  render({}) {
    return HTML`
      <p>This site lets you compare personal and world record progressions over time.</p>

<pre>
                     <u>Example URLs</u>
        Player View: <a ${this.urlStyle} href="/@banks">/@banks</a>
          Game View: <a ${this.urlStyle} href="/wc2">/sc1</a>
   + Multiple Games: <a ${this.urlStyle} href="/wc2+wc2btdp">/wc2+wc2btdp</a>
   + Personal Bests: <a ${this.urlStyle} href="/wc2+wc2btdp/@zpr">/wc2+wc2btdp/@zpr</a>
         Level View: <a ${this.urlStyle} href="/smwext/world-1/@banks+@lui">/smwext/world-1/@banks+@lui</a>
</pre>

    `;
  }
  
  get style() {
    return style({
      display: 'block',
      margin: {top: '16px', bottom: '32px'},
      text: {align: 'left'},
    });
  }
}
