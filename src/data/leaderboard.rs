//! Leaderboard logic.
use std::{collections::HashSet, sync::Arc};

use getset::Getters;
use serde::Serialize;

use crate::data::{
    database::{Database, Linked},
    types::*,
};

#[derive(Debug, Clone, Getters, Serialize)]
#[get = "pub"]
pub struct RankedRun {
    rank:      Id64,
    time_ms:   u64,
    is_tied:   bool,
    tied_rank: Id64,
    run:       Linked<Run>,
}

// obsolete

/// Ranks a set of runs (all for the same game/category/level) using the
/// timing specified for the game rules, then by run date, then by
/// submission datetime, discarding lower-ranked runs by the same runner.
pub fn rank_runs(database: Arc<Database>, runs: &[Linked<Run>]) -> Vec<RankedRun> {
    let mut runs: Vec<Linked<Run>> = runs.to_vec();

    if runs.is_empty() {
        return vec![]
    }

    let game = runs[0].game();

    runs.sort_by_key(|run| {
        let time_ms = run
            .times_ms()
            .get(game.primary_timing())
            .expect("run missing primary timing");

        (time_ms, *run.date(), *run.created())
    });

    let mut ranked_players: HashSet<&Vec<RunPlayer>> = HashSet::new();

    let mut ranks: Vec<RankedRun> = vec![];

    let mut n = 0;
    for run in runs.iter() {
        if !ranked_players.insert(run.players()) {
            // this run is obsolete, skip it
            continue
        }

        n += 1;

        let time_ms = run
            .times_ms()
            .get(game.primary_timing())
            .expect("run missing primary timing");
        let rank = Id64::new(n as u64).expect("id is zero?!");
        let mut tied_rank = rank;
        let mut is_tied = false;

        if let Some(ref mut previous) = ranks.last_mut() {
            if time_ms == *previous.time_ms() {
                is_tied = true;
                previous.is_tied = true;
                tied_rank = previous.tied_rank;
            }
        }

        let new = RankedRun {
            rank,
            time_ms,
            is_tied,
            tied_rank,
            run: run.clone(),
        };

        ranks.push(new);
    }

    ranks
}
