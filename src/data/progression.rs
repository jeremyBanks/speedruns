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
    let leaderboard_runs = leaderboard(runs);

    let _runs_by_level = runs
        .iter()
        .map(|run| (run.level_id.clone(), run))
        .into_group_map();

    leaderboard_runs
        .into_iter()
        .map(|lr| ProgressionRun {
            progress_ms:     0,
            run:             lr.run().clone(),
            leaderboard_run: lr,
        })
        .collect()

    // let runs: Vec<Linked<Run>> = runs.to_vec();

    // if runs.is_empty() {
    //     return vec![];
    // }

    // let mut progression: Vec<ProgressionRun> = vec![];

    // for run in runs.iter() {
    //     // let new = ProgressionRun { run: run.clone() };

    //     // progression.push(new);
    // }

    // progression
}
