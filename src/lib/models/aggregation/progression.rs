use std::collections::HashMap;

use getset::Getters;
use itertools::Itertools;
use serde::Serialize;

use crate::{
    aggregation::leaderboard::{leaderboard, LeaderboardRun},
    Game, Run,
};

#[derive(Debug, Clone, Getters, Serialize)]
#[get = "pub"]
pub struct ProgressionRun {
    progress_ms: u64,
    run: Run,
    leaderboard_run: Option<LeaderboardRun>,
}

pub fn progression<'runs>(
    game: &'_ Game,
    runs: impl Iterator<Item = &'runs Run>,
) -> Vec<ProgressionRun> {
    let runs: Vec<&Run> = runs.collect();

    if runs.is_empty() {
        return vec![];
    }

    let game_id = runs[0].game_id;
    let category_id = runs[0].category_id;
    assert!(
        runs.iter()
            .all(|r| r.game_id == game_id && r.category_id == category_id),
        "runs must all be from same game and category"
    );

    #[allow(clippy::clone_double_ref)]
    let runs_by_level: HashMap<Option<u64>, Vec<&'runs Run>> = runs
        .iter()
        .sorted_by(|a, b| {
            a.date()
                .cmp(&b.date())
                .then(a.created().cmp(&b.created()))
                .then(a.id().cmp(&b.id()))
        })
        .map(|run| (run.level_id, run.clone()))
        .into_group_map();

    let mut progression: Vec<ProgressionRun> = Vec::new();

    for (_level_id, runs) in runs_by_level.iter().sorted_by(|a, b| a.0.cmp(b.0)) {
        let mut best_ms: Option<u64> = None;

        let mut leaderboard_runs_by_id: HashMap<u64, LeaderboardRun> = HashMap::new();
        for leaderboard_run in leaderboard(game, runs.iter().cloned(), false) {
            let id = *leaderboard_run.run().id();
            leaderboard_runs_by_id.insert(id, leaderboard_run);
        }

        for run in runs.iter() {
            let is_progress;
            let mut progress_ms = 0;
            let run_time = run
                .times_ms
                .get(&game.primary_timing)
                .expect("primary timing");

            match best_ms {
                None => {
                    is_progress = true;
                }
                Some(best_ms) => {
                    is_progress = run_time < best_ms;
                    if is_progress {
                        progress_ms = best_ms - run_time;
                    }
                }
            }

            if is_progress {
                progression.push(ProgressionRun {
                    progress_ms,
                    run: Run::clone(run),
                    leaderboard_run: leaderboard_runs_by_id.remove(run.id()),
                });
                best_ms = Some(run_time);
            }
        }
    }

    // let runs: Vec<&'run Run> = runs.to_vec();

    // if runs.is_empty() {
    //     return vec![];
    // }

    // let mut progression: Vec<ProgressionRun> = vec![];

    // for run in runs.iter() {
    //     // let new = ProgressionRun { run: run.clone() };

    //     // progression.push(new);
    // }

    // reverse-chronologial
    progression.sort_by(|a, b| {
        b.run
            .date()
            .cmp(&a.run.date())
            .then(b.run.created().cmp(&a.run.created()))
    });
    progression
}
