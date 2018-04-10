class UnofficialRun {
  constructor(playerName, durationSeconds, url, dateTime) {
    this.weblink = url;
    this.players = [
      {
          rel: "guest",
          name: playerName,
      }
    ];
    this.submitted = dateTime;
    this.date = dateTime.split('T')[0];
    
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

export const extraData = {
  'runs?game=o1yry26q&category=wdmw5ee2&level=xd17ejqd&status=verified&orderby=date&direction=asc': [
    new UnofficialRun()
  ]
};
