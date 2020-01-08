import HTML from '/assets/bester/html.js';
import {Component} from '/assets/bester/component.js';
import {style} from '/assets/bester/style.js';


export class HomeBody extends Component {
  get labelStyle() {
    return style({
      opacity: 0.75
    });
  }

  render({makePath}) {
    const link = opts => {
      const path = makePath(opts);
      return HTML`<a href="${path}">${path}</a>`;
    }

    return HTML`
      <p>This site compares personal and world record speed run progress over time.</p>

<pre ${style({line: {height: 1.5}})}>
                     <u>Example URLs</u>
<span ${this.labelStyle}>             runner:</span> ${link({runnerSlugs: ['banks']})}
<span ${this.labelStyle}>               game:</span> ${link({gameSlugs: ['wc2']})}
<span ${this.labelStyle}>     multiple games:</span> ${link({gameSlugs: ['wc2', 'wc2btdp']})}
<span ${this.labelStyle}>with personal bests:</span> ${link({gameSlugs: ['wc2', 'wc2btdp'], runnerSlugs: ['ZPR']})}
<span ${this.labelStyle}>              level:</span> ${link({gameSlugs: ['smwext'], levelSlugs: ['world-1'], runnerSlugs: ['banks', 'lui']})}
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
 