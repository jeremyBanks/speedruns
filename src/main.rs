#![feature(custom_attribute)]
#![feature(try_blocks)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_attributes)]

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

use std::{collections::BTreeMap, convert::From, error::Error, fs};

mod data_source;
mod persistent;

use self::persistent::Persistent;

const THINKING: &str = r#"
    what do I want to do with my data?

    find all levels
    for each level
    sort runs by date, then by submision datetime
    find record-setting runs, strip out the rest
    record how much each run improves the existing record

    take the first run in each category for our initial total time

    sort all record-setting runs by date together
    then go through the, using the recorded deltas to update the sum-of-best-segment after each.

"#;

pub fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let data = data_source::SpeedRunComData::open("data.json");

    let war2 = &data.games()["o1yry26q"];
    let level = &war2.levels[0];
    let runs_by_level = data
        .runs()
        .values()
        .map(|run| (run.level_id.clone(), run))
        .into_group_map();
    let mut runs = runs_by_level[&Some(level.level_id.clone())].clone();
    runs.sort_by(|a, b| {
        a.duration
            .partial_cmp(&b.duration)
            .expect("no NaN here")
            .then(a.performed.cmp(&b.performed))
            .then(a.submitted.cmp(&b.submitted))
    });

    println!("First level: {:#?}", level);
    println!("World record: {:#?}", &runs[0..4]);

    Ok(())
}
