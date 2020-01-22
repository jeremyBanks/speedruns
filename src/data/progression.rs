use getset::Getters;
use serde::Serialize;

use crate::data::{database::Linked, types::*};

#[derive(Debug, Clone, Getters, Serialize)]
#[get = "pub"]
pub struct ProgressionRun {
    run: Linked<Run>,
}

pub fn progression(runs: &[Linked<Run>]) -> Vec<ProgressionRun> {
    let runs: Vec<Linked<Run>> = runs.to_vec();

    if runs.is_empty() {
        return vec![]
    }

    let mut progression: Vec<ProgressionRun> = vec![];

    for run in runs.iter() {
        let new = ProgressionRun { run: run.clone() };

        progression.push(new);
    }

    progression
}
