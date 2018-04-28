import HTML from '/assets/bester/html.js';
import {Component} from '/assets/bester/component.js';
import {style} from '/assets/bester/style.js';


export class HomeBody extends Component {
  get labelStyle() {
    return style({
      opacity: 0.75
    });
  }

  render({}) {
    return HTML`
      <p>This site compares personal and world record speed run progress over time.</p>

<pre>
                     <u>Example URLs</u>
<span ${this.labelStyle}>        runner view:</span> <a href="/@banks">/@banks</a>
<span ${this.labelStyle}>          game view:</span> <a href="/wc2">/sc1</a>
<span ${this.labelStyle}>   + multiple games:</span> <a href="/wc2+wc2btdp">/wc2+wc2btdp</a>
<span ${this.labelStyle}>   + personal bests:</span> <a href="/wc2+wc2btdp/@zpr">/wc2+wc2btdp/@zpr</a>
<span ${this.labelStyle}>         level view:</span> <a href="/smwext/world-1/@banks+@lui">/smwext/world-1/@banks+@lui</a>
</pre>

    `;
  }
  
  get style() {
    return style({
      display: 'block',
      margin: {top: '32px', bottom: '32px'},
      text: {align: 'left'},
    });
  }
}
