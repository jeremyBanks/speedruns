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

const war2 = 'y65zy46e';
const war2x = 'y65zy46e';
const war2x = 'wkponpj2';

export const extraData = {
  [runs(war2, war2x, 'kwj5l7r9')]: [ // orc1
    run('AverageAvocado', 999, 'YOUTUBEYOUTUBE', '2015-01-01')
  ],
  [runs(war2, war2x, 'owo017vw')]: [ // orc2
    run('AverageAvocado', 999, 'YOUTUBEYOUTUBE', '2015-01-01')
  ],
  [runs(war2, war2x, 'xd17ejqd')]: [ // orc3
    run('AverageAvocado', 999, 'YOUTUBEYOUTUBE', '2015-01-01')
  ],
  [runs(war2, war2x, 'ewpjm7lw')]: [ // orc4
    run('AverageAvocado', 999, 'YOUTUBEYOUTUBE', '2015-01-01')
  ],
  [runs(war2, war2x, 'y9mjq5l9')]: [ // orc5
    run('AverageAvocado', 999, 'YOUTUBEYOUTUBE', '2015-01-01')
  ],
  [runs(war2, war2x, '5wkjzk5d')]: [ // orc6
    run('AverageAvocado', 999, 'YOUTUBEYOUTUBE', '2015-01-01')
  ],
  [runs(war2, war2x, '592zg0g9')]: [ // orc7
    run('AverageAvocado', 999, 'YOUTUBEYOUTUBE', '2015-01-01')
  ],
  [runs(war2, war2x, '29v3ey1w')]: [ // orc8
    run('AverageAvocado', 999, 'YOUTUBEYOUTUBE', '2015-01-01')
  ],
  [runs(war2, war2x, 'xd4j760d')]: [ // orc9
    run('AverageAvocado', 999, 'YOUTUBEYOUTUBE', '2015-01-01')
  ],
  [runs(war2, war2x, 'xd0gr00w')]: [ // orc10
    run('AverageAvocado', 999, 'YOUTUBEYOUTUBE', '2015-01-01')
  ],
  [runs(war2, war2x, 'rw64pvn9')]: [ // orc11
    run('AverageAvocado', 999, 'YOUTUBEYOUTUBE', '2015-01-01')
  ],
  [runs(war2, war2x, 'n937507d')]: [ // orc12
    run('AverageAvocado', 999, 'YOUTUBEYOUTUBE', '2015-01-01')
  ],
  [runs(war2, war2x, 'z986vj7d')]: [ // orc13
    run('AverageAvocado', 999, 'YOUTUBEYOUTUBE', '2015-01-01')
  ],
  [runs(war2, war2x, 'rdn0j5nw')]: [ // orc14
  ],

  [runs(war2, war2x, 'ldyy7ejd')]: [ // human1
    run('AverageAvocado', 999, 'YOUTUBEYOUTUBE', '2015-01-01')
  ],
  [runs(war2, war2x, 'n93750nd')]: [ // human2
    run('AverageAvocado', 999, 'YOUTUBEYOUTUBE', '2015-01-01')
  ],
  [runs(war2, war2x, 'z986vjgd')]: [ // human3
    run('AverageAvocado', 999, 'YOUTUBEYOUTUBE', '2015-01-01')
  ],
  [runs(war2, war2x, 'rdn0j5qw')]: [ // human4
    run('AverageAvocado', 999, 'YOUTUBEYOUTUBE', '2015-01-01')
  ],
  [runs(war2, war2x, 'ldyy7ekd')]: [ // human5
    run('AverageAvocado', 999, 'YOUTUBEYOUTUBE', '2015-01-01')
  ],
  [runs(war2, war2x, 'gdre4j69')]: [ // human6
    run('AverageAvocado', 999, 'YOUTUBEYOUTUBE', '2015-01-01')
  ],
  [runs(war2, war2x, 'nwllx60w')]: [ // human7
    run('AverageAvocado', 999, 'YOUTUBEYOUTUBE', '2015-01-01')
  ],
  [runs(war2, war2x, 'ywe1k4yd')]: [ // human8
    run('AverageAvocado', 999, 'YOUTUBEYOUTUBE', '2015-01-01')
  ],
  [runs(war2, war2x, '69z3orld')]: [ // human9
    run('AverageAvocado', 999, 'YOUTUBEYOUTUBE', '2015-01-01')
  ],
  [runs(war2, war2x, 'r9g20z5d')]: [ // human10
    run('AverageAvocado', 999, 'YOUTUBEYOUTUBE', '2015-01-01')
  ],
  [runs(war2, war2x, 'o9xn520w')]: [ // human11
    run('AverageAvocado', 999, 'YOUTUBEYOUTUBE', '2015-01-01')
  ],
  [runs(war2, war2x, '4956oj0d')]: [ // human12
    run('AverageAvocado', 999, 'YOUTUBEYOUTUBE', '2015-01-01')
  ],
  [runs(war2, war2x, 'rdqj20k9')]: [ // human13
    run('AverageAvocado', 999, 'YOUTUBEYOUTUBE', '2015-01-01')
  ],
  [runs(war2, war2x, '5d7vk7gd')]: [ // human14
    run('AverageAvocado', 999, 'YOUTUBEYOUTUBE', '2015-01-01')
  ],
  
  [runs(war2x, war2x, 'XXXXXXXXX')]: [ // xhuman1
  ],
  [runs(war2x, war2x, 'XXXXXXXXX')]: [ // xhuman2
  ],
  [runs(war2x, war2x, 'XXXXXXXXX')]: [ // xhuman3
  ],
  [runs(war2x, war2x, 'XXXXXXXXX')]: [ // xhuman4
  ],
  [runs(war2x, war2x, 'XXXXXXXXX')]: [ // xhuman5
  ],
  [runs(war2x, war2x, 'XXXXXXXXX')]: [ // xhuman6
  ],
  [runs(war2x, war2x, 'XXXXXXXXX')]: [ // xhuman7
  ],
  [runs(war2x, war2x, 'XXXXXXXXX')]: [ // xhuman8
  ],
  [runs(war2x, war2x, 'XXXXXXXXX')]: [ // xhuman9
  ],
  [runs(war2x, war2x, 'XXXXXXXXX')]: [ // xhuman10
  ],
  [runs(war2x, war2x, 'XXXXXXXXX')]: [ // xhuman11
  ],
  [runs(war2x, war2x, 'XXXXXXXXX')]: [ // xhuman12
  ],

  [runs(war2x, war2x, 'kwj5lyn9')]: [ // xorc1
    run('AverageAvocado', 284, 'https://youtu.be/eeBgiR8OBlU?t=66', '2015-05-15')
  ],
  [runs(war2x, war2x, 'owo01kow')]: [ // xorc2
    run('AverageAvocado', 741, 'https://youtu.be/5YjjECd8LMg?t=45', '2015-05-16')
  ],
  [runs(war2x, war2x, 'xd17e0ed')]: [ // xorc3
    run('AverageAvocado', 951, 'https://youtu.be/0AnFa52gul0?t=39', '2015-05-22')
  ],
  [runs(war2x, war2x, 'ewpjmqkw')]: [ // xorc4
    run('AverageAvocado', 860, 'https://youtu.be/AdtTlUA4sa0?t=56', '2015-05-23')
  ],
  [runs(war2x, war2x, 'y9mjqp59')]: [ // xorc5
    run('AverageAvocado', 1039, 'https://youtu.be/mOsxBWDLGjM?t=62', '2015-05-29')
  ],
  [runs(war2x, war2x, '5wkjzr2d')]: [ // xorc6
    run('AverageAvocado', 1373, 'https://youtu.be/W5I7WNnARpc?t=47', '2015-05-30')
  ],
  [runs(war2x, war2x, '592zgoo9')]: [ // xorc7
    run('AverageAvocado', 1983, 'https://youtu.be/XU5rZn8VNP0?t=62', '2015-06-05')
  ],
  [runs(war2x, war2x, '29v3ek3w')]: [ // xorc8
    run('AverageAvocado', 1771, 'https://youtu.be/IoVa94tVuzA?t=36', '2015-06-06')
  ],
  [runs(war2x, war2x, 'xd4j732d')]: [ // xorc9
    run('AverageAvocado', 578, 'https://youtu.be/KNKMXghZbnY?t=62', '2015-06-12')
  ],
  [runs(war2x, war2x, 'xd0gr7xw')]: [ // xorc10
    run('AverageAvocado', 1333, 'https://youtu.be/VjWPkHj3qio?t=66', '2015-06-12')
  ],
  [runs(war2x, war2x, 'rw64pnr9')]: [ // xorc11
    run('AverageAvocado', 1721, 'https://youtu.be/10awDV6v9t0?t=51', '2015-06-16')
  ],
  [runs(war2x, war2x, 'n9375gnd')]: [ // xorc12
    run('AverageAvocado', 2307, 'https://youtu.be/tUgBU_3yO6s?t=54', '2015-06-20')
  ],
};
