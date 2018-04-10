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

const war2 = 'o1yry26q';
const war2Mission = 'wdmw5ee2';
const war2x = 'y65zy46e';
const war2xMission = 'wkponpj2';

let _ = {}; 
export const extraData = {
  // These are runs that may not qualify as "speed runs" for the purpose of the
  // speedrun.com leaderboard, but which I'd still like to include in the
  // historical data.

  [runs(war2, war2Mission, 'kwj5l7r9')]: [ // orc1
    run('AverageAvocado', 217, _.avoUrlOrc2013 = 'https://www.youtube.com/playlist?list=PL5b0fctdcCFMyFFMP8KHrBlh4lj1y6oQk', _.avoDate2013 = '2013-06-00'),
    run('AverageAvocado', 224, _.avoUrl2015 = 'https://www.youtube.com/playlist?list=PLm5DuBIoS54B89M4RQFbNUvZv0HhOye3R', _.avoDate2015 = '2015-00-00'),
  ],
  [runs(war2, war2Mission, 'owo017vw')]: [ // orc2
    run('AverageAvocado', 100, _.avoUrlOrc2013, _.avoDate2013),
    run('AverageAvocado', 102, _.avoUrl2015, _.avoDate2015),
  ],
  [runs(war2, war2Mission, 'xd17ejqd')]: [ // orc3
    run('AverageAvocado', 519, _.avoUrlOrc2013, _.avoDate2013),
    run('AverageAvocado', 464, _.avoUrl2015, _.avoDate2015),
  ],
  [runs(war2, war2Mission, 'ewpjm7lw')]: [ // orc4
    run('AverageAvocado', 941, _.avoUrlOrc2013, _.avoDate2013),
    run('AverageAvocado', 1143, _.avoUrl2015, _.avoDate2015),
  ],
  [runs(war2, war2Mission, 'y9mjq5l9')]: [ // orc5
    run('AverageAvocado', 1444, _.avoUrlOrc2013, _.avoDate2013),
    run('AverageAvocado', 1154, _.avoUrl2015, _.avoDate2015),
  ],
  [runs(war2, war2Mission, '5wkjzk5d')]: [ // orc6
    run('AverageAvocado', 225, _.avoUrlOrc2013, _.avoDate2013),
    run('AverageAvocado', 224, _.avoUrl2015, _.avoDate2015),
  ],
  [runs(war2, war2Mission, '592zg0g9')]: [ // orc7
    run('AverageAvocado', 1608, _.avoUrlOrc2013, _.avoDate2013),
    run('AverageAvocado', 1235, _.avoUrl2015, _.avoDate2015),
  ],
  [runs(war2, war2Mission, '29v3ey1w')]: [ // orc8
    run('AverageAvocado', 1276, _.avoUrlOrc2013, _.avoDate2013),
    run('AverageAvocado', 1066, _.avoUrl2015, _.avoDate2015),
  ],
  [runs(war2, war2Mission, 'xd4j760d')]: [ // orc9
    run('AverageAvocado', 1223, _.avoUrlOrc2013, _.avoDate2013),
    run('AverageAvocado', 1180, _.avoUrl2015, _.avoDate2015),
  ],
  [runs(war2, war2Mission, 'xd0gr00w')]: [ // orc10
    run('AverageAvocado', 1953, _.avoUrlOrc2013, _.avoDate2013),
    run('AverageAvocado', 1437, _.avoUrl2015, _.avoDate2015),
  ],
  [runs(war2, war2Mission, 'rw64pvn9')]: [ // orc11
    run('AverageAvocado', 1891, _.avoUrlOrc2013, _.avoDate2013),
    run('AverageAvocado', 1675, _.avoUrl2015, _.avoDate2015),
  ],
  [runs(war2, war2Mission, 'n937507d')]: [ // orc12
    run('AverageAvocado', 3065, _.avoUrlOrc2013, _.avoDate2013),
    run('AverageAvocado', 2295, _.avoUrl2015, _.avoDate2015),
  ],
  [runs(war2, war2Mission, 'z986vj7d')]: [ // orc13
    run('AverageAvocado', 2397, _.avoUrlOrc2013, _.avoDate2013),
    run('AverageAvocado', 1761, _.avoUrl2015, _.avoDate2015),
  ],
  [runs(war2, war2Mission, 'rdn0j5nw')]: [ // orc14
    run('AverageAvocado', 2508, _.avoUrlOrc2013, _.avoDate2013),
    run('AverageAvocado', 1833, _.avoUrl2015, _.avoDate2015),
  ],

  [runs(war2, war2Mission, 'ldyy7ejd')]: [ // human1
    run('AverageAvocado', 302, _.avoUrlHuman2013 = 'https://www.youtube.com/playlist?list=PL5b0fctdcCFN2J7hzVshQzYdhdyHBusrF', _.avoDate2013),
    run('AverageAvocado', 208, _.avoUrl2015, _.avoDate2015),
  ],
  [runs(war2, war2Mission, 'n93750nd')]: [ // human2
    run('AverageAvocado', 89, _.avoUrl2015, _.avoDate2015),
  ],
  [runs(war2, war2Mission, 'z986vjgd')]: [ // human3
    run('AverageAvocado', 409, _.avoUrl2015, _.avoDate2015),
  ],
  [runs(war2, war2Mission, 'rdn0j5qw')]: [ // human4
    run('AverageAvocado', 710, _.avoUrl2015, _.avoDate2015),
  ],
  [runs(war2, war2Mission, 'ldyy7ekd')]: [ // human5
    run('AverageAvocado', 966, _.avoUrl2015, _.avoDate2015),
  ],
  [runs(war2, war2Mission, 'gdre4j69')]: [ // human6
    run('AverageAvocado', 927, _.avoUrl2015, _.avoDate2015),
  ],
  [runs(war2, war2Mission, 'nwllx60w')]: [ // human7
    run('AverageAvocado', 1008, _.avoUrl2015, _.avoDate2015),
  ],
  [runs(war2, war2Mission, 'ywe1k4yd')]: [ // human8
    run('AverageAvocado', 918, _.avoUrl2015, _.avoDate2015),
  ],
  [runs(war2, war2Mission, '69z3orld')]: [ // human9
    run('AverageAvocado', 503, _.avoUrl2015, _.avoDate2015),
  ],
  [runs(war2, war2Mission, 'r9g20z5d')]: [ // human10
    run('AverageAvocado', 1002, _.avoUrl2015, _.avoDate2015),
  ],
  [runs(war2, war2Mission, 'o9xn520w')]: [ // human11
    run('AverageAvocado', 1037, _.avoUrl2015, _.avoDate2015),
  ],
  [runs(war2, war2Mission, '4956oj0d')]: [ // human12
    run('AverageAvocado', 1180, _.avoUrl2015, _.avoDate2015),
  ],
  [runs(war2, war2Mission, 'rdqj20k9')]: [ // human13
    run('AverageAvocado', 2337, _.avoUrl2015, _.avoDate2015),
  ],
  [runs(war2, war2Mission, '5d7vk7gd')]: [ // human14
    run('AverageAvocado', 1462, _.avoUrl2015, _.avoDate2015),
  ],

  [runs(war2x, war2xMission, 'z986v8gd')]: [ // xhuman1
    run('AverageAvocado', 411, _.avoUrlXHuman2013 = 'https://www.youtube.com/playlist?list=PL5b0fctdcCFMrfEc2a3oI_25cPHwmccxG', _.avoDate2013),
  ],
  [runs(war2x, war2xMission, 'rdn0jpqw')]: [ // xhuman2
  ],
  [runs(war2x, war2xMission, 'ldyy7pkd')]: [ // xhuman3
  ],
  [runs(war2x, war2xMission, 'gdre4q69')]: [ // xhuman4
  ],
  [runs(war2x, war2xMission, 'nwllx70w')]: [ // xhuman5
  ],
  [runs(war2x, war2xMission, 'ywe1k8yd')]: [ // xhuman6
  ],
  [runs(war2x, war2xMission, '69z3o4ld')]: [ // xhuman7
  ],
  [runs(war2x, war2xMission, 'r9g2035d')]: [ // xhuman8
  ],
  [runs(war2x, war2xMission, 'o9xn5l0w')]: [ // xhuman9
  ],
  [runs(war2x, war2xMission, '4956o80d')]: [ // xhuman10
  ],
  [runs(war2x, war2xMission, 'rdqj21k9')]: [ // xhuman11
  ],
  [runs(war2x, war2xMission, '5d7vk3gd')]: [ // xhuman12
  ],

  [runs(war2x, war2xMission, 'kwj5lyn9')]: [ // xorc1
    run('AverageAvocado', 357, _.avoUrlXOrc2013 = 'https://www.youtube.com/playlist?list=PL5b0fctdcCFNBE21S42pVshwj6Bh4adRE', _.avoDate2013),
    run('AverageAvocado', 284, 'https://youtu.be/eeBgiR8OBlU?t=66', '2015-05-15'),
  ],
  [runs(war2x, war2xMission, 'owo01kow')]: [ // xorc2
    run('AverageAvocado', 741, 'https://youtu.be/5YjjECd8LMg?t=45', '2015-05-16'),
  ],
  [runs(war2x, war2xMission, 'xd17e0ed')]: [ // xorc3
    run('AverageAvocado', 951, 'https://youtu.be/0AnFa52gul0?t=39', '2015-05-22'),
  ],
  [runs(war2x, war2xMission, 'ewpjmqkw')]: [ // xorc4
    run('AverageAvocado', 860, 'https://youtu.be/AdtTlUA4sa0?t=56', '2015-05-23'),
  ],
  [runs(war2x, war2xMission, 'y9mjqp59')]: [ // xorc5
    run('AverageAvocado', 1039, 'https://youtu.be/mOsxBWDLGjM?t=62', '2015-05-29'),
  ],
  [runs(war2x, war2xMission, '5wkjzr2d')]: [ // xorc6
    run('AverageAvocado', 1373, 'https://youtu.be/W5I7WNnARpc?t=47', '2015-05-30'),
  ],
  [runs(war2x, war2xMission, '592zgoo9')]: [ // xorc7
    run('AverageAvocado', 1983, 'https://youtu.be/XU5rZn8VNP0?t=62', '2015-06-05'),
  ],
  [runs(war2x, war2xMission, '29v3ek3w')]: [ // xorc8
    run('AverageAvocado', 1771, 'https://youtu.be/IoVa94tVuzA?t=36', '2015-06-06'),
  ],
  [runs(war2x, war2xMission, 'xd4j732d')]: [ // xorc9
    run('AverageAvocado', 578, 'https://youtu.be/KNKMXghZbnY?t=62', '2015-06-12'),
  ],
  [runs(war2x, war2xMission, 'xd0gr7xw')]: [ // xorc10
    run('AverageAvocado', 1333, 'https://youtu.be/VjWPkHj3qio?t=66', '2015-06-12'),
  ],
  [runs(war2x, war2xMission, 'rw64pnr9')]: [ // xorc11
    run('AverageAvocado', 1721, 'https://youtu.be/10awDV6v9t0?t=51', '2015-06-16'),
  ],
  [runs(war2x, war2xMission, 'n9375gnd')]: [ // xorc12
    run('AverageAvocado', 2307, 'https://youtu.be/tUgBU_3yO6s?t=54', '2015-06-20'),
  ],
};
