class UnofficialRun {
  constructor(playerName, durationSeconds, url, date) {
    this.ℹ️ = this.constructor.name;
    this.weblink = url;
    this.players = [
      {
          rel: 'guest',
          name: playerName,
      }
    ];
    this.submitted = date + 'T00:00:00.000Z';
    this.date = date;
    
    let durationString = 'PT';
    {
      const hours = Math.floor(durationSeconds / (60 * 60));
      const minutes = Math.floor(durationSeconds / 60) % 60;
      const seconds = Math.floor(durationSeconds) % 60;
      const miliseconds = durationSeconds % 1.0;
      if (hours) durationString += `${hours}H`;
      if (minutes) durationString += `${minutes}M`;
      if (seconds) durationString += `${seconds}S`;
      if (miliseconds) throw new Error("not supported!");
    }

    this.times = {
      primary: durationString,
      primary_t: durationSeconds,
    };
  }
}
const runs = (game, category, level) => `runs?game=${game}&category=${category}&level=${level}&status=verified&orderby=date&direction=asc&max=200`;
const run = (...args) => new UnofficialRun(...args);

const war2x = 'y65zy46e';
const mission = 'wkponpj2';

export const extraData = {
  [runs(war2x, mission, 'kwj5lyn9')]: [ // xorc1
    run('AverageAvocado', 284, 'https://youtu.be/eeBgiR8OBlU?t=66', '2015-05-15')
  ],
  [runs(war2x, mission, 'owo01kow')]: [ // xorc2
    run('AverageAvocado', 741, 'https://youtu.be/5YjjECd8LMg?t=45', '2015-05-16')
  ],
  [runs(war2x, mission, 'xd17e0ed')]: [ // xorc3
    run('AverageAvocado', 951, 'https://youtu.be/0AnFa52gul0?t=39', '2015-05-22')
  ],
  [runs(war2x, mission, 'ewpjmqkw')]: [ // xorc4
    run('AverageAvocado', 860, 'https://youtu.be/AdtTlUA4sa0?t=56', '2015-05-23')
  ],
  [runs(war2x, mission, 'y9mjqp59')]: [ // xorc5
    run('AverageAvocado', 1039, 'https://youtu.be/mOsxBWDLGjM?t=62', '2015-05-29')
  ],
  [runs(war2x, mission, '5wkjzr2d')]: [ // xorc6
    run('AverageAvocado', 1373, 'https://youtu.be/W5I7WNnARpc?t=47', '2015-05-30')
  ],
  [runs(war2x, mission, '592zgoo9')]: [ // xorc7
    run('AverageAvocado', 0e0000000, 'yyyyyyyyyy', '2015-XX-XX')
  ],
  [runs(war2x, mission, '29v3ek3w')]: [ // xorc8
    run('AverageAvocado', 0e0000000, 'yyyyyyyyyy', '2015-XX-XX')
  ],
  [runs(war2x, mission, 'xd4j732d')]: [ // xorc9
    run('AverageAvocado', 0e0000000, 'yyyyyyyyyy', '2015-XX-XX')
  ],
  [runs(war2x, mission, 'xd0gr7xw')]: [ // xorc10
    run('AverageAvocado', 0e0000000, 'yyyyyyyyyy', '2015-XX-XX')
  ],
  [runs(war2x, mission, 'rw64pnr9')]: [ // xorc11
    run('AverageAvocado', 1721, 'https://youtu.be/10awDV6v9t0?t=51', '2015-06-16')
  ],
  [runs(war2x, mission, 'n9375gnd')]: [ // xorc12
    run('AverageAvocado', 0e0000000, 'yyyyyyyyyy', '2015-XX-XX')
  ],
};
