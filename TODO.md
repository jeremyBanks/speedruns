split out your view and display model:

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
        gameRecords: Record[]{
          name: string,
          url: string,
          topRuns: Run[]{
            timeStr: string,
            player: Player,
            place: int > 0,
          },
          personalBests: Run[]
        },
        levelRecords: Record[],
      }
    }
