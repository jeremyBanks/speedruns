- finish view model!

- use this.async

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
