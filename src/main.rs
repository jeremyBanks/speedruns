#![feature(custom_attribute)]
#![feature(try_blocks)]
#![feature(try_from)]
#![feature(slice_concat_ext)]

#[macro_use]
extern crate log;
use env_logger;
#[macro_use]
extern crate serde_derive;
use chrono::{Duration, Utc};
use itertools::Itertools;
use std::{collections::BTreeMap, error::Error};

mod persistent;
mod speedrun_data;
mod texty;
use clap;

use self::{
    speedrun_data::{Run, SpeedRunComData},
    texty::color_with_hash,
};

/// TODOs:
/// - refactor this stuff out of main
/// - add support for categories (like separate games), and full-game runs.

pub fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let args = clap::App::new("flamerun")
        .about("Displays record progressions from speedrun.com.")
        .arg(
            clap::Arg::with_name("yes_refresh")
                .short("r")
                .long("refresh")
                .help("Forces all data to be refreshed, even if it's fresh."),
        )
        .arg(
            clap::Arg::with_name("no_refresh")
                .short("l")
                .long("offline")
                .help("Prevents data from being refreshed, even if it's stale."),
        )
        .arg(
            clap::Arg::with_name("max_age_days")
                .long("max-age-days")
                .value_name("DAYS")
                .default_value("720")
                .require_equals(true)
                .help("The maximum age in days of records to display."),
        )
        .arg(
            clap::Arg::with_name("games")
                .long("games")
                .value_name("GAMES")
                .help("Comma-separated list of game slugs or IDs. Defaults to all known games."),
        )
        .get_matches();

    let refresh: Option<bool> = if args.is_present("no_refresh") {
        Some(false)
    } else if args.is_present("yes_refresh") {
        Some(true)
    } else {
        None
    };

    let max_age_days: i64 = args
        .value_of("max_age_days")
        .unwrap()
        .parse()
        .expect("days must be a number");

    let data = SpeedRunComData::open("data.json", refresh);

    let runs_by_level = data
        .runs()
        .values()
        .filter(|run| run.status != speedrun_data::RunStatus::Rejected)
        .map(|run| (run.level_id.clone(), run))
        .into_group_map();

    let games = data.games().values();
    let mut records_by_level_id = BTreeMap::new();
    for game in games {
        println!("\n\n           {}\n", game.name);

        let mut all_level_records = Vec::<Record>::new();
        for level in game.levels.iter() {
            let runs_option = runs_by_level.get(&Some(level.level_id.clone()));
            let runs = match runs_option {
                Some(runs) => runs,
                None => {
                    debug!("  no runs for {:?}", level);
                    records_by_level_id.insert(level.level_id.clone(), Vec::new());
                    continue;
                }
            };

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
            .filter(|records| records.len() > 0)
            .map(|records| records.first().unwrap().run.duration)
            .fold(Duration::zero(), |a, b| a + b);

        let _best_sum: Duration = game
            .levels
            .iter()
            .map(|l| &records_by_level_id[&l.level_id])
            .filter(|records| records.len() > 0)
            .map(|records| records.last().unwrap().run.duration)
            .fold(Duration::zero(), |a, b| a + b);

        all_level_records.sort_by(|a, b| {
            a.run
                .performed
                .cmp(&b.run.performed)
                .then(a.run.submitted.cmp(&b.run.submitted))
        });

        println!("  date         runner        level               time  sum      delta");

        let mut sum = worst_sum;
        for record in all_level_records {
            let level = game
                .levels
                .iter()
                .filter(|l| Some(l.level_id.clone()) == record.run.level_id)
                .next()
                .unwrap();

            sum = sum - record.improvement;

            if Utc::today().naive_utc() - record.run.performed > Duration::days(max_age_days) {
                continue;
            }

            let term_bg_black = "\x1b[40m";
            let fg_yellow = "\x1b[93m";
            let fg_grey = "\x1b[37m";
            let fg_white = "\x1b[97m";
            let term_style_reset = "\x1b[0m";
            let improvement_text;
            let record_style;
            if record.improvement == Duration::zero() {
                improvement_text = "".to_string();
            } else {
                improvement_text = fmt_duration(record.improvement);
            };
            let improvement = &format!("{:>5}", improvement_text);

            if records_by_level_id[&level.level_id]
                .last()
                .unwrap()
                .run
                .run_id
                == record.run.run_id
            {
                // This is the current record.
                record_style = fg_yellow;
            } else {
                record_style = fg_white;
            };

            println!(
                "{}",
                [
                    " ",
                    term_bg_black,
                    " ",
                    // date
                    &color_with_hash(&record.run.performed.to_string()),
                    " ",
                    // flag
                    fg_white,
                    &record.run.player.flag().unwrap_or(" ".to_string()),
                    " ",
                    // runner
                    &color_with_hash(&format!("{:<12}", record.run.player.to_string())[..12]),
                    "  ",
                    // level
                    &color_with_hash(&format!("{:<16}", level.name.to_string())[..16]),
                    fg_grey,
                    " in ",
                    // record/sum
                    &record_style,
                    &format!("{:>5}", fmt_duration(record.run.duration)),
                    fg_grey,
                    "/",
                    &fmt_duration(sum),
                    "  ",
                    // delta
                    fg_white,
                    improvement,
                    // reset before EOL
                    " ",
                    term_style_reset,
                ]
                .join("")
            );
        }
    }

    println!("\n");

    Ok(())
}

fn fmt_duration(duration: Duration) -> String {
    let signed_ms = duration.num_milliseconds();
    let ms = signed_ms.abs();
    let ms_part = ms % 1000;
    let s = ms / 1000;
    let s_part = s % 60;
    let m = s / 60;
    let m_part = m % 60;
    let h = m / 60;

    let sign = if signed_ms < 0 { "-" } else { "" };
    if h > 0 {
        format!("{}{}:{:02}:{:02}", sign, h, m_part, s_part)
    } else if m_part > 0 || s_part > 0 {
        format!("{}{}:{:02}", sign, m_part, s_part)
    } else if ms_part > 0 {
        format!("{}0.{:03}", sign, ms_part)
    } else {
        format!("0")
    }
}

#[derive(Debug, Clone)]
struct Record<'a> {
    pub run: &'a Run,
    pub improvement: Duration,
}
