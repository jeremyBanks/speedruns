use std::{collections::HashSet, convert::TryFrom};

use getset::Getters;
use serde::Serialize;

use crate::data::{database::Linked, types::*};

#[derive(Debug, Clone, Getters, Serialize)]
#[get = "pub"]
pub struct LeaderboardRun {
    rank:      u64,
    time_ms:   u64,
    is_tied:   bool,
    tied_rank: u64,
    run:       Linked<Run>,
}

/// Ranks a set of runs (all for the same game/category/level) using the
/// timing specified for the game rules, then by run date, then by
/// submission datetime, discarding lower-ranked runs by the same runner
/// unless rank_obsoletes is true.
pub fn leaderboard(runs: &[Linked<Run>], rank_obsoletes: bool) -> Vec<LeaderboardRun> {
    if runs.is_empty() {
        return vec![]
    }

    let mut runs: Vec<Linked<Run>> = runs.to_vec();

    let game_id = runs[0].game_id;
    let category_id = runs[0].category_id;
    let level_id = runs[0].level_id;
    assert!(
        runs.iter().all(|r| r.game_id == game_id
            && r.category_id == category_id
            && r.level_id == level_id),
        "runs must all be from same game and category and level"
    );

    let game = runs[0].game();

    runs.sort_by_key(|run| {
        let time_ms = run
            .times_ms()
            .get(game.primary_timing())
            .expect("run missing primary timing");

        (time_ms, *run.date(), *run.created())
    });

    let mut ranked_players: HashSet<&Vec<RunPlayer>> = HashSet::new();

    let mut leaderboard: Vec<LeaderboardRun> = vec![];

    let mut n = 0;
    for run in runs.iter() {
        if !rank_obsoletes && !ranked_players.insert(run.players()) {
            // this run is obsolete, skip it
            continue
        }

        n += 1;

        let time_ms = run
            .times_ms()
            .get(game.primary_timing())
            .expect("run missing primary timing");
        let rank = u64::try_from(n).unwrap();
        let mut tied_rank = rank;
        let mut is_tied = false;

        if let Some(ref mut previous) = leaderboard.last_mut() {
            if time_ms == *previous.time_ms() {
                is_tied = true;
                previous.is_tied = true;
                tied_rank = previous.tied_rank;
            }
        }

        let new = LeaderboardRun {
            rank,
            time_ms,
            is_tied,
            tied_rank,
            run: run.clone(),
        };

        leaderboard.push(new);
    }

    leaderboard
}
