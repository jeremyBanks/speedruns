future Jeremy here.

let's make this a library.

let's display graphs of times over time, such as ours vs WR

-----

- finish view model!

- use this.async in HTML

- persist cache on disk and define an expiration policy

- make sure our view model json thingie supports async iterators properly

----

- display non-top-3 PBs

- split out your view and display model:

    Bests{
      glitchProjectName: string?,
      player: Player{
        nick: string,
        id: string?,
        url: string
      }
      
      games: Game[]{
        name: string,
        url: string,
        
        iconUrl: string,
        trophyUrls: string[],
        
        // these are only multiple for ties:
        
        gameRecords: Record[]{
          name: string,
          url: string,
          topRuns: Run[]{
            timeStr: string,
            player: Player,
            place: int > 0,
            date maybe?
          },
          personalBests: Run[]
        },
        levelRecords: Record[],
      }
    }
