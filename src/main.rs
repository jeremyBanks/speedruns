#![feature(custom_attribute)]
#![feature(try_blocks)]
#![feature(try_from)]

#[macro_use]
extern crate log;
use env_logger;
#[macro_use]
extern crate serde_derive;
use itertools::Itertools;

use chrono::{Datelike, Duration};
use std::{collections::BTreeMap, error::Error};

mod persistent;
mod speedrun_data;

use self::speedrun_data::{Run, SpeedRunComData};

pub fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let data = SpeedRunComData::open("data.json");

    let runs_by_level = data
        .runs()
        .values()
        .map(|run| (run.level_id.clone(), run))
        .into_group_map();

    let games = data.games().values();
    let mut records_by_level_id = BTreeMap::new();
    for game in games {
        println!("\n\n           {}\n", game.name);

        let mut all_level_records = Vec::<Record>::new();
        for level in game.levels.iter() {
            let runs = &runs_by_level[&Some(level.level_id.clone())];

            let mut runs_chronological = runs.clone();
            runs_chronological.sort_by(|a, b| {
                a.performed
                    .cmp(&b.performed)
                    .then(a.submitted.cmp(&b.submitted))
            });

            let mut level_records = Vec::<Record>::new();
            for run in runs_chronological {
                let new_record = match level_records.last() {
                    None => Some(Record {
                        run,
                        improvement: Duration::zero(),
                    }),
                    Some(record) => {
                        let improvement = record.run.duration - run.duration;
                        if improvement > Duration::zero() {
                            Some(Record { run, improvement })
                        } else {
                            None
                        }
                    }
                };
                if let Some(record) = new_record {
                    level_records.push(record);
                }
            }

            records_by_level_id.insert(level.level_id.clone(), level_records.clone());
            all_level_records.append(&mut level_records);
        }

        let worst_sum: Duration = game
            .levels
            .iter()
            .map(|l| &records_by_level_id[&l.level_id])
            .map(|records| records.first().unwrap().run.duration)
            .fold(Duration::zero(), |a, b| a + b);

        let best_sum: Duration = game
            .levels
            .iter()
            .map(|l| &records_by_level_id[&l.level_id])
            .map(|records| records.last().unwrap().run.duration)
            .fold(Duration::zero(), |a, b| a + b);

        all_level_records.sort_by(|a, b| {
            a.run
                .performed
                .cmp(&b.run.performed)
                .then(a.run.submitted.cmp(&b.run.submitted))
        });

        let mut sum = worst_sum;
        for record in all_level_records {
            let level = game
                .levels
                .iter()
                .filter(|l| Some(l.level_id.clone()) == record.run.level_id)
                .next()
                .unwrap();

            sum = sum - record.improvement;

            if record.run.performed.year() < 2018 {
                continue;
            }

            let improvement = if record.improvement == Duration::zero() {
                "".to_string()
            } else {
                fmt_duration(-record.improvement)
            };

            println!(
                "  {:>8} {:<16} {} {:>8} {:>8} in {:>6}",
                improvement,
                record.run.player.to_string(),
                record.run.performed,
                fmt_duration(sum),
                level.name.to_string().split(":").next().unwrap(),
                fmt_duration(record.run.duration),
            );
        }
    }

    println!("\n");

    Ok(())
}

fn fmt_duration(duration: Duration) -> String {
    let signed_ms = duration.num_milliseconds();
    let ms = signed_ms.abs();
    let _ms_part = ms % 1000;
    let s = ms / 1000;
    let s_part = s % 60;
    let m = s / 60;
    let m_part = m % 60;
    let h = m / 60;

    let sign = if signed_ms < 0 { "-" } else { "" };
    if h > 0 {
        format!("{}{}h{:02}m{:02}s", sign, h, m_part, s_part)
    } else if m_part > 0 {
        format!("{}{}m{:02}s", sign, m_part, s_part)
    } else if s_part > 0 {
        format!("{}{}s", sign, s_part)
    } else {
        format!("0")
    }
}

#[derive(Debug, Clone)]
struct Record<'a> {
    pub run: &'a Run,
    pub improvement: Duration,
}
