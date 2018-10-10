#![feature(custom_attribute)]
#![feature(try_blocks)]
#![feature(try_from)]

#[macro_use]
extern crate log;
use env_logger;
use reqwest;
use serde;
use serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate derive_more;
use itertools::Itertools;

use std::{
    collections::BTreeMap,
    convert::{From, TryFrom},
    error::Error,
    fs,
};

mod persistent;
mod speedrun_data;

use self::{
    persistent::Persistent,
    speedrun_data::{Game, Run, SpeedRunComData},
};

pub fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let data = SpeedRunComData::open("data.json");

    let war2 = &data.games()["o1yry26q"];
    let runs_by_level = data
        .runs()
        .values()
        .map(|run| (run.level_id.clone(), run))
        .into_group_map();

    for level in war2.levels.iter() {
        let runs = &runs_by_level[&Some(level.level_id.clone())];

        let mut runs_chronological = runs.clone();
        runs_chronological.sort_by(|a, b| {
            a.performed
                .cmp(&b.performed)
                .then(a.submitted.cmp(&b.submitted))
        });

        let mut records = Vec::<Record>::new();
        for run in runs_chronological {
            let new_record = match records.last() {
                None => Some(Record {
                    run,
                    improvement: 0.0,
                }),
                Some(record) => {
                    let improvement = record.run.duration - run.duration;
                    if improvement > 0.0 {
                        Some(Record { run, improvement })
                    } else {
                        None
                    }
                }
            };
            if let Some(record) = new_record {
                records.push(record);
            }
        }

        println!("{}", level.name);
        for record in records {
            if record.improvement == 0.0 {
                println!(
                    "  first {:5}s by {}",
                    record.run.duration, record.run.player
                );
            } else {
                println!(
                    "  -{:<4} {:5}s by {}",
                    format!("{}s", record.improvement),
                    record.run.duration,
                    record.run.player
                );
            }
        }
        println!("\n");
    }

    // TODO:
    // put all of the records in a chronological list.
    // calculate sum of best and worst sum of records.

    Ok(())
}

#[derive(Debug)]
struct Record<'a> {
    pub run: &'a Run,
    pub improvement: f64,
}
