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
      const seconds = Math.floor(durationSeconds) % (60 * 60);
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

export const extraData = {
  [runs('y65zy46e', 'wkponpj2', 'rw64pnr9')]: [
    run('AverageAvocado', 1721, 'https://youtu.be/10awDV6v9t0?t=51', '2015-06-16')
  ]
};
