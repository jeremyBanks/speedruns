// These are runs that may not qualify as "speed runs" for the purpose of the
// speedrun.com leaderboard, but which we'd still like to include in the
// historical data we display.

const getExtraRuns = () => {
  const _ = {};
  return {
    'o1yry26q': { // WarCraft 2: Tides of Darkness
      'wdmw5ee2': { // Missions
        'kwj5l7r9': [ // Orc 1
          run('AverageAvocado', 217, _.avoUrlOrc2013 = 'https://www.youtube.com/playlist?list=PL5b0fctdcCFMyFFMP8KHrBlh4lj1y6oQk', _.avoDate2013 = '2013-06   '),
          run('AverageAvocado', 224, _.avoUrl2015 = 'https://www.youtube.com/playlist?list=PLm5DuBIoS54B89M4RQFbNUvZv0HhOye3R', _.avoDate2015 = '2015-01   '),
          run('Cire2047', 229, 'https://youtu.be/Pl0ZkfDKEsg?t=62', '2011-05-01'),
        ],
        'owo017vw': [ // Orc 2
          run('AverageAvocado', 100, _.avoUrlOrc2013, _.avoDate2013),
          run('AverageAvocado', 102, _.avoUrl2015, _.avoDate2015),
          run('Cire2047', 188, 'https://youtu.be/4DTZpqWcSiE?t=47', '2011-05-05'),
        ],
        'xd17ejqd': [ // Orc 3
          run('AverageAvocado', 519, _.avoUrlOrc2013, _.avoDate2013),
          run('AverageAvocado', 464, _.avoUrl2015, _.avoDate2015),
          run('Cire2047', 544, _.cireUrlOrc = 'https://www.youtube.com/playlist?list=PLumTHdkN2x_nwV_rcgadktukTaBZiubYc', '2011-05-05'),
        ],
        'ewpjm7lw': [ // Orc 4
          run('AverageAvocado', 941, _.avoUrlOrc2013, _.avoDate2013),
          run('AverageAvocado', 1143, _.avoUrl2015, _.avoDate2015),
          run('Cire2047', 960, _.cireUrlOrc, '2011-05-06'),
        ],
        'y9mjq5l9': [ // Orc 5
          run('AverageAvocado', 1444, _.avoUrlOrc2013, _.avoDate2013),
          run('AverageAvocado', 1154, _.avoUrl2015, _.avoDate2015),
          run('Cire2047', 1369, _.cireUrlOrc, '2011-05-09'),
        ],
        '5wkjzk5d': [ // Orc 6
          run('AverageAvocado', 225, _.avoUrlOrc2013, _.avoDate2013),
          run('AverageAvocado', 224, _.avoUrl2015, _.avoDate2015),
          run('Cire2047', 298, _.cireUrlOrc, '2012-12-27'),
        ],
        '592zg0g9': [ // Orc 7
          run('AverageAvocado', 1608, _.avoUrlOrc2013, _.avoDate2013),
          run('AverageAvocado', 1235, _.avoUrl2015, _.avoDate2015),
          run('Cire2047', 2414, _.cireUrlOrc, '2012-12-27'),
        ],
        '29v3ey1w': [ // Orc 8
          run('AverageAvocado', 1276, _.avoUrlOrc2013, _.avoDate2013),
          run('AverageAvocado', 1066, _.avoUrl2015, _.avoDate2015),
          run('Cire2047', 1748, _.cireUrlOrc, '2012-12-27'),
        ],
        'xd4j760d': [ // Orc 9
          run('AverageAvocado', 1223, _.avoUrlOrc2013, _.avoDate2013),
          run('AverageAvocado', 1180, _.avoUrl2015, _.avoDate2015),
          run('Cire2047', 1750, _.cireUrlOrc, '2012-12-27'),
        ],
        'xd0gr00w': [ // Orc 10
          run('AverageAvocado', 1953, _.avoUrlOrc2013, _.avoDate2013),
          run('AverageAvocado', 1437, _.avoUrl2015, _.avoDate2015),
          run('Cire2047', 1950, 'https://youtu.be/2CmiwWkFVf8?t=34', '2012-12-28'),
        ],
        'rw64pvn9': [ // Orc 11
          run('AverageAvocado', 1891, _.avoUrlOrc2013, _.avoDate2013),
          run('AverageAvocado', 1675, _.avoUrl2015, _.avoDate2015),
          run('Cire2047', 2372, _.cireUrlOrc, '2012-12-28'),
        ],
        'n937507d': [ // Orc 12
          run('AverageAvocado', 3065, _.avoUrlOrc2013, _.avoDate2013),
          run('AverageAvocado', 2295, _.avoUrl2015, _.avoDate2015),
          run('Cire2047', 3212, _.cireUrlOrc, '2013-01-04'),
        ],
        'z986vj7d': [ // Orc 13
          run('AverageAvocado', 2397, _.avoUrlOrc2013, _.avoDate2013),
          run('AverageAvocado', 1761, _.avoUrl2015, _.avoDate2015),
          run('Cire2047', 3277, _.cireUrlOrc, '2013-01-12'),
        ],
        'rdn0j5nw': [ // Orc 14
          run('AverageAvocado', 2508, _.avoUrlOrc2013, _.avoDate2013),
          run('AverageAvocado', 1833, _.avoUrl2015, _.avoDate2015),
          run('Cire2047', 3024, _.cireUrlOrc, '2015-06-28'),
        ],

        'ldyy7ejd': [ // human1
          run('AverageAvocado', 302, _.avoUrlHuman2013 = 'https://www.youtube.com/playlist?list=PL5b0fctdcCFN2J7hzVshQzYdhdyHBusrF', _.avoDate2013),
          run('AverageAvocado', 208, _.avoUrl2015, _.avoDate2015),
          run('Cire2047', 277, 'https://youtu.be/CSeONuQXKww?t=57', '2011-04-15'),
        ],
        'n93750nd': [ // human2
          run('AverageAvocado', 76, 'https://youtu.be/5_pAZwO4LdY?t=51', _.avoDate2013),
          run('AverageAvocado', 89, _.avoUrl2015, _.avoDate2015),
          run('Cire2047', 116, _.cireUrlHuman = 'https://www.youtube.com/playlist?list=PLC960334B12409C7E', '2011-04-15'),
        ],
        'z986vjgd': [ // human3
          run('AverageAvocado', 551, 'https://youtu.be/lhACxCKSrIA?t=51', _.avoDate2013),
          run('AverageAvocado', 409, _.avoUrl2015, _.avoDate2015),
          run('Cire2047', 882, _.cireUrlHuman, '2011-04-15'),
        ],
        'rdn0j5qw': [ // human4
          run('AverageAvocado', 989, 'https://youtu.be/n4A0nOJRbcI?t=43', _.avoDate2013),
          run('AverageAvocado', 710, _.avoUrl2015, _.avoDate2015),
          run('Cire2047', 1309, _.cireUrlHuman, '2011-04-16'),
        ],
        'ldyy7ekd': [ // human5
          run('AverageAvocado', 908, 'https://youtu.be/-wRQqXiP3-U?t=72', _.avoDate2013),
          run('AverageAvocado', 966, _.avoUrl2015, _.avoDate2015),
          run('Cire2047', 1569, _.cireUrlHuman, '2011-04-16'),
        ],
        'gdre4j69': [ // human6
          run('AverageAvocado', 981, 'https://youtu.be/KSwZOZRo1cU?t=34', _.avoDate2013),
          run('AverageAvocado', 927, _.avoUrl2015, _.avoDate2015),
          run('Cire2047', 1549, _.cireUrlHuman, '2011-04-17'),
        ],
        'nwllx60w': [ // human7
          run('AverageAvocado', 1246, 'https://youtu.be/QOdjzMpJUYA?t=44', _.avoDate2013),
          run('AverageAvocado', 1008, _.avoUrl2015, _.avoDate2015),
          run('Cire2047', 2043, _.cireUrlHuman, '2011-04-17'),
        ],
        'ywe1k4yd': [ // human8
          run('AverageAvocado', 856, 'https://youtu.be/bjpkuqV9hVg?t=81', _.avoDate2013),
          run('AverageAvocado', 918, _.avoUrl2015, _.avoDate2015),
          run('Cire2047', 2049, _.cireUrlHuman, '2011-04-17'),
        ],
        '69z3orld': [ // human9
          run('AverageAvocado', 432, 'https://youtu.be/LdC2jO7Kr0I?t=27', _.avoDate2013),
          run('AverageAvocado', 503, _.avoUrl2015, _.avoDate2015),
          run('Cire2047', 120, _.cireUrlHuman, '2011-04-17'),
        ],
        'r9g20z5d': [ // human10
          run('AverageAvocado', 1354, 'https://youtu.be/vcvTVZgeVcs?t=33', _.avoDate2013),
          run('AverageAvocado', 1002, _.avoUrl2015, _.avoDate2015),
          run('Cire2047', 2066, _.cireUrlHuman, '2011-04-19'),
        ],
        'o9xn520w': [ // human11
          run('AverageAvocado', 1107, 'https://youtu.be/T91yL8WqYdQ?t=50', _.avoDate2013),
          run('AverageAvocado', 1037, _.avoUrl2015, _.avoDate2015),
          run('Cire2047', 1835, _.cireUrlHuman, '2011-04-24'),
        ],
        '4956oj0d': [ // human12
          run('AverageAvocado', 1158, 'https://youtu.be/gmeAiz2ud60?t=57', _.avoDate2013),
          run('AverageAvocado', 1180, _.avoUrl2015, _.avoDate2015),
          run('Cire2047', 1458, _.cireUrlHuman, '2011-04-30'),
        ],
        'rdqj20k9': [ // human13
          run('AverageAvocado', 2702, 'https://youtu.be/fpPQj50jGn0?t=54', _.avoDate2013),
          run('AverageAvocado', 2337, _.avoUrl2015, _.avoDate2015),
          run('Cire2047', 3110, _.cireUrlHuman, '2011-04-30'),
        ],
        '5d7vk7gd': [ // human14
          run('AverageAvocado', 1810, 'https://youtu.be/SCLKKmiXRck?t=37', _.avoDate2013),
          run('AverageAvocado', 1462, _.avoUrl2015, _.avoDate2015),
          run('Cire2047', 3745, _.cireUrlHuman, '2011-05-01'),
        ],
      }
    },
    'y65zy46e': { // WarCraft 2x: Beyond the Dark Portal
      'wkponpj2': { // Missions
        'z986v8gd': [ // xHuman 1
          run('AverageAvocado', 411, _.avoUrlXHuman2013 = 'https://www.youtube.com/playlist?list=PL5b0fctdcCFMrfEc2a3oI_25cPHwmccxG', _.avoDate2013),
        ],
        'rdn0jpqw': [ // xHuman 2
          run('AverageAvocado', 1476, 'https://youtu.be/sXIsb-N66_g?t=44', _.avoDate2013),
        ],
        'ldyy7pkd': [ // xHuman 3
          run('AverageAvocado', 3027, 'https://youtu.be/rzNGZfoQKXQ?t=31', _.avoDate2013),
        ],
        'gdre4q69': [ // xHuman 4
          run('AverageAvocado', 1668, _.avoUrlXHuman2013, _.avoDate2013),
        ],
        'nwllx70w': [ // xHuman 5
          run('AverageAvocado', 1921, _.avoUrlXHuman2013, _.avoDate2013),
        ],
        'ywe1k8yd': [ // xHuman 6
          run('AverageAvocado', 1921, 'https://youtu.be/zEpneIWa7pE?t=49', _.avoDate2013),
        ],
        '69z3o4ld': [ // xHuman 7
          run('AverageAvocado', 709, _.avoUrlXHuman2013, _.avoDate2013),
        ],
        'r9g2035d': [ // xHuman 8
          run('AverageAvocado', 2529, _.avoUrlXHuman2013, _.avoDate2013),
        ],
        'o9xn5l0w': [ // xHuman 9
          run('AverageAvocado', 2598, _.avoUrlXHuman2013, _.avoDate2013),
        ],
        '4956o80d': [ // xHuman 10
          run('AverageAvocado', 1686, _.avoUrlXHuman2013, _.avoDate2013),
        ],
        'rdqj21k9': [ // xHuman 11
          run('AverageAvocado', 1786, _.avoUrlXHuman2013, _.avoDate2013),
        ],
        '5d7vk3gd': [ // xHuman 12
          run('AverageAvocado', 607, _.avoUrlXHuman2013, _.avoDate2013),
        ],

        'kwj5lyn9': [ // xOrc 1
          run('AverageAvocado', 357, _.avoUrlXOrc2013 = 'https://www.youtube.com/playlist?list=PL5b0fctdcCFNBE21S42pVshwj6Bh4adRE', _.avoDate2013),
          run('AverageAvocado', 284, 'https://youtu.be/eeBgiR8OBlU?t=66', '2015-05-15'),
        ],
        'owo01kow': [ // xOrc 2
          run('AverageAvocado', 1277, _.avoUrlXOrc2013, _.avoDate2013),
          run('AverageAvocado', 741, 'https://youtu.be/5YjjECd8LMg?t=45', '2015-05-16'),
        ],
        'xd17e0ed': [ // xOrc 3
          run('AverageAvocado', 1459, _.avoUrlXOrc2013, _.avoDate2013),
          run('AverageAvocado', 951, 'https://youtu.be/0AnFa52gul0?t=39', '2015-05-22'),
        ],
        'ewpjmqkw': [ // xOrc 4
          run('AverageAvocado', 2837, _.avoUrlXOrc2013, _.avoDate2013),
          run('AverageAvocado', 860, 'https://youtu.be/AdtTlUA4sa0?t=56', '2015-05-23'),
        ],
        'y9mjqp59': [ // xOrc 5
          run('AverageAvocado', 2409, _.avoUrlXOrc2013, _.avoDate2013),
          run('AverageAvocado', 1039, 'https://youtu.be/mOsxBWDLGjM?t=62', '2015-05-29'),
        ],
        '5wkjzr2d': [ // xOrc 6
          run('AverageAvocado', 1909, _.avoUrlXOrc2013, _.avoDate2013),
          run('AverageAvocado', 1373, 'https://youtu.be/W5I7WNnARpc?t=47', '2015-05-30'),
        ],
        '592zgoo9': [ // xOrc 7
          run('AverageAvocado', 2950, _.avoUrlXOrc2013, _.avoDate2013),
          run('AverageAvocado', 1983, 'https://youtu.be/XU5rZn8VNP0?t=62', '2015-06-05'),
        ],
        '29v3ek3w': [ // xOrc 8
          run('AverageAvocado', 2694, _.avoUrlXOrc2013, _.avoDate2013),
          run('AverageAvocado', 1771, 'https://youtu.be/IoVa94tVuzA?t=36', '2015-06-06'),
        ],
        'xd4j732d': [ // xOrc 9
          run('AverageAvocado', 792, _.avoUrlXOrc2013, _.avoDate2013),
          run('AverageAvocado', 578, 'https://youtu.be/KNKMXghZbnY?t=62', '2015-06-12'),
        ],
        'xd0gr7xw': [ // xOrc 10
          run('AverageAvocado', 2225, _.avoUrlXOrc2013, _.avoDate2013),
          run('AverageAvocado', 1333, 'https://youtu.be/VjWPkHj3qio?t=66', '2015-06-12'),
        ],
        'rw64pnr9': [ // xOrc 11
          run('AverageAvocado', 2536, _.avoUrlXOrc2013, _.avoDate2013),
          run('AverageAvocado', 1721, 'https://youtu.be/10awDV6v9t0?t=51', '2015-06-16'),
        ],
        'n9375gnd': [ // xOrc 12
          run('AverageAvocado', 2307, 'https://youtu.be/tUgBU_3yO6s?t=54', '2015-06-20'),
        ],
      }
    }
  };
};


const run = (...args) => new UnofficialRunFakeApiResponse(...args);


class UnofficialRunFakeApiResponse {
  constructor(playerName, durationSeconds, url, date) {
    this.ℹ️ = this.constructor.name;
    this.weblink = url;
    this.players = {data: [
      {
          rel: 'guest',
          name: playerName,
      }
    ]};
    this.submitted = date + 'T00:00:00.000Z';
    this.date = date;

    // this.level = levelId;
    // this.categoryId = level.category;
    
    let durationString = 'PT';
    durationString: {
      if (durationSeconds === 0) {
        durationString += '0';
        break durationString;
      }
      const hours = Math.floor(durationSeconds / (60 * 60));
      const minutes = Math.floor(durationSeconds / 60) % 60;
      const seconds = durationSeconds % 60; // may include miliseconds
      if (hours) durationString += `${String(hours).padStart(2, '0')}H`;
      if (minutes) durationString += `${String(minutes).padStart(2, '0')}M`;
      if (seconds) durationString += `${String(seconds).padStart(2, '0')}S`;
    }

    this.times = {
      primary: durationString,
      primary_t: durationSeconds,
    };
  }
}


export const extraRuns = getExtraRuns(); 
