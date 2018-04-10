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
    
    let durationString;
    {
      const hours = Math.floor(durationSeconds / (60 * 60));
      const minutes = Math.floor(durationSeconds / 60) % 60;
      const seconds = Math.floor(durationSeconds) % (60 * 60);
      const miliseconds = durationSeconds % 1.0;
      durationString += 'PT';
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
const R = (...args) => new UnofficialRun(...args);

export const extraData = {
  'runs?game=o1yry26q&category=wdmw5ee2&level=xd17ejqd&status=verified&orderby=date&direction=asc': [
    R('AverageAvocado', 1721, 'https://youtu.be/10awDV6v9t0?t=51', '2015-06-16')
  ]
};
