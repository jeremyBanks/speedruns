use getset::Getters;
use serde::Serialize;

use crate::data::{
    database::Linked,
    leaderboard::{leaderboard, LeaderboardRun},
    types::*,
};

#[derive(Debug, Clone, Getters, Serialize)]
#[get = "pub"]
pub struct ProgressionRun {
    improvement_ms:  u64,
    run:             Linked<Run>,
    leaderboard_run: LeaderboardRun,
}

pub fn progression(runs: &[Linked<Run>]) -> Vec<ProgressionRun> {
    let leaderboard_runs = leaderboard(runs);

    leaderboard_runs
        .into_iter()
        .map(|lr| ProgressionRun {
            improvement_ms:  0,
            run:             lr.run().clone(),
            leaderboard_run: lr,
        })
        .collect()
}
