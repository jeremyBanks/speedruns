use super::persistent::Persistent;
use chrono::{Date, DateTime, Duration, NaiveDate, Utc};
use core::{convert::From, str::FromStr};
use reqwest;
use std::{collections::HashMap, error::Error, fmt::Debug, fs};

pub struct SpeedRunComData {
    data: Persistent<Data>,
}

impl SpeedRunComData {
    pub fn open(filename: &str) -> Self {
        let mut self_ = SpeedRunComData {
            data: Persistent::open(filename),
        };

        let needs_refresh = match self_.data.get().last_refreshed {
            None => true,
            Some(timestamp) => (Utc::now() - timestamp).num_hours() >= 4,
        };

        if needs_refresh {
            let refreshed = self_.refresh();

            if let Err(error) = refreshed {
                warn!(
                    "failed to refresh data {:?}, continuing with cached data",
                    error
                );
            }
        } else {
            debug!("Skipping refresh.");
        }

        self_
    }

    fn refresh(&mut self) -> Result<(), ()> {
        let last_refreshed = Utc::now();
        let runs = &mut self.data.get_mut().runs;

        let run_urls = vec![
            "https://www.speedrun.com/api/v1/runs?game=o1yry26q&embed=players&max=200",
            "https://www.speedrun.com/api/v1/runs?game=y65zy46e&embed=players&max=200",
        ];

        for url in run_urls {
            let updated: Result<(), Box<dyn Error>> = try {
                let mut response = reqwest::get(url)?;
                if !response.status().is_success() {
                    return Err(());
                }
                let json = response.text()?;
                let parsed: speedruncom_api::runs::Response = serde_json::from_str(&json)?;
                debug!("Refreshing data from {:?}.", url);

                for run in parsed.data {
                    runs.insert(
                        run.id.clone(),
                        Run {
                            run_id: run.id,
                            status: run.status.into(),
                            player: run.players.into(),
                            game_id: run.game,
                            level_id: run.level,
                            category_id: run.category,
                            performed: NaiveDate::from_str(&run.date.unwrap())?,
                            submitted: DateTime::<Utc>::from_str(&run.submitted.unwrap())?,
                            duration: run.times.primary_t.into(),
                        },
                    );
                }
            };

            if let Err(error) = updated {
                warn!("Data update from {:?} failed: {:?}", url, error);
            }
        }

        self.data.get_mut().last_refreshed = Some(last_refreshed);

        self.data.sync();

        Ok(())
    }

    pub fn games(&self) -> &HashMap<String, Game> {
        &self.data.get().games
    }

    pub fn runs(&self) -> &HashMap<String, Run> {
        &self.data.get().runs
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
struct Data {
    last_refreshed: Option<DateTime<Utc>>,
    games: HashMap<String, Game>,
    runs: HashMap<String, Run>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Game {
    game_id: String,
    name: String,
    levels: Vec<Level>,
    run_categories: Vec<FullRunCategory>,
    level_run_categories: Vec<LevelRunCategory>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Level {
    level_id: String,
    name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FullRunCategory {
    category_id: String,
    name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LevelRunCategory {
    category_id: String,
    name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Run {
    run_id: String,
    game_id: String,
    category_id: String,
    level_id: Option<String>,
    status: RunStatus,
    player: Player,
    performed: NaiveDate,
    submitted: DateTime<Utc>,
    duration: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Player {
    User { user_id: String, name: String },
    Guest { name: String },
    MultiplePlayers,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RunStatus {
    Pending,
    Verified,
    Rejected,
}

mod speedruncom_api {
    pub mod runs {
        #[derive(Deserialize, Debug)]
        pub struct Response {
            pub data: Vec<Run>,
        }

        #[derive(Deserialize, Debug)]
        pub struct Run {
            pub id: String,
            pub status: RunStatus,
            pub game: String,
            pub level: Option<String>,
            pub category: String,
            pub players: PlayersData,
            pub date: Option<String>,
            pub submitted: Option<String>,
            pub times: Times,
        }

        #[derive(Deserialize, Debug)]
        #[serde(tag = "status")]
        #[serde(rename_all = "snake_case")]
        pub enum RunStatus {
            Verified,
            New,
            Rejected,
        }

        impl Into<super::super::RunStatus> for RunStatus {
            fn into(self) -> super::super::RunStatus {
                match self {
                    RunStatus::New => super::super::RunStatus::Pending,
                    RunStatus::Verified => super::super::RunStatus::Verified,
                    RunStatus::Rejected => super::super::RunStatus::Rejected,
                }
            }
        }

        #[derive(Deserialize, Debug)]
        pub struct PlayersData {
            pub data: Vec<Player>,
        }

        impl Into<super::super::Player> for PlayersData {
            fn into(self) -> super::super::Player {
                if self.data.len() != 1 {
                    super::super::Player::MultiplePlayers
                } else {
                    match self.data[0] {
                        Player::User { ref id, ref names } => super::super::Player::User {
                            user_id: id.clone(),
                            name: names.international.clone(),
                        },
                        Player::Guest { ref name } => {
                            super::super::Player::Guest { name: name.clone() }
                        }
                    }
                }
            }
        }

        #[derive(Deserialize, Debug)]
        #[serde(tag = "rel")]
        #[serde(rename_all = "snake_case")]
        pub enum Player {
            User { id: String, names: UserNames },
            Guest { name: String },
        }

        #[derive(Deserialize, Debug)]
        pub struct UserNames {
            pub international: String,
        }

        #[derive(Deserialize, Debug)]
        pub struct Times {
            pub primary_t: u32,
        }
    }
}
