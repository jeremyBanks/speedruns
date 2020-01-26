use std::collections::HashMap;

use getset::Getters;
use itertools::Itertools;
use serde::Serialize;

use crate::data::{
    database::Linked,
    leaderboard::{leaderboard, LeaderboardRun},
    types::*,
};

#[derive(Debug, Clone, Getters, Serialize)]
#[get = "pub"]
pub struct ProgressionRun {
    progress_ms:     u64,
    run:             Linked<Run>,
    leaderboard_run: LeaderboardRun,
}

pub fn progression(runs: &[Linked<Run>]) -> Vec<ProgressionRun> {
    if runs.is_empty() {
        return vec![]
    }

    let game_id = runs[0].game_id;
    let category_id = runs[0].category_id;
    assert!(
        runs.iter()
            .all(|r| r.game_id == game_id && r.category_id == category_id),
        "runs must all be from same game and category"
    );

    assert!(
        runs.iter().all(|r| r.level_id == None),
        "levels not supported yet"
    );

    let runs_by_level: HashMap<Option<u64>, Vec<Linked<Run>>> = runs
        .iter()
        .map(|run| (run.level_id.clone(), run.clone()))
        .into_group_map();

    let mut progression: Vec<ProgressionRun> = Vec::new();

    for (_level_id, runs) in runs_by_level {
        let mut runs_by_date = leaderboard(&runs.to_vec());
        runs_by_date.sort_by(|a, b| {
            a.run()
                .date()
                .cmp(&b.run().date())
                .then(a.run().created().cmp(&b.run().created()))
        });

        // collect all runs which are progress
        // don't need to worry about sum time yet!
    }

    // let runs: Vec<Linked<Run>> = runs.to_vec();

    // if runs.is_empty() {
    //     return vec![];
    // }

    // let mut progression: Vec<ProgressionRun> = vec![];

    // for run in runs.iter() {
    //     // let new = ProgressionRun { run: run.clone() };

    //     // progression.push(new);
    // }

    progression.sort_by(|a, b| {
        a.run()
            .date()
            .cmp(&b.run().date())
            .then(a.run().created().cmp(&b.run().created()))
    });

    progression
}
