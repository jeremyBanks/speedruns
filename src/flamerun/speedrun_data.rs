use super::persistent::Persistent;
use chrono::{DateTime, Duration, NaiveDate, Utc};
use core::str::FromStr;
use reqwest;

use std::{
    collections::BTreeMap,
    error::Error,
    fmt,
    fmt::{Debug, Display},
};

pub struct SpeedRunComData {
    data: Persistent<Data>,
}

impl SpeedRunComData {
    pub fn open(filename: &str, refresh: Option<bool>) -> Self {
        let mut self_ = SpeedRunComData {
            data: Persistent::open(filename),
        };

        let needs_refresh = match self_.data.get().last_refreshed {
            None => true,
            Some(timestamp) => (Utc::now() - timestamp).num_hours() >= 4,
        };

        if refresh.unwrap_or(needs_refresh) {
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

    pub fn games(&self) -> &BTreeMap<String, Game> {
        &self.data.get().games
    }

    pub fn runs(&self) -> &BTreeMap<String, Run> {
        &self.data.get().runs
    }

    fn refresh(&mut self) -> Result<(), ()> {
        let last_refreshed = Utc::now();

        let game_ids = vec!["wc2", "wc2btdp", "sc1", "scbw", "braid"];

        for game_id in game_ids {
            if let Err(error) = self.refresh_game(game_id) {
                warn!("Failed to refresh game {}: {:?}.", game_id, error);
            }
        }

        self.data.get_mut().last_refreshed = Some(last_refreshed);

        self.data.sync();

        Ok(())
    }

    fn refresh_game(&mut self, game_id_or_slug: &str) -> Result<(), Box<dyn Error>> {
        let api = "https://www.speedrun.com/api/v1";

        let game_url = format!(
            "{}/games/{}?embed=categories,levels,variables",
            api, game_id_or_slug
        );
        debug!("Refreshing game metadata from {:?}.", game_url);
        let mut game_response = reqwest::get(&game_url)?;
        if !game_response.status().is_success() {
            return Err(NonSuccessResponseStatus {
                status: game_response.status(),
                url:    game_url,
            }
            .into())
        }
        let game_json = game_response.text()?;
        let games_data: speedruncom_api::game::Response = serde_json::from_str(&game_json)?;
        let game = games_data.data;
        let game_id = game.id.clone();

        self.data.get_mut().games.insert(
            game.id.clone(),
            Game {
                game_id:              game.id,
                name:                 game.names.international,
                run_categories:       game
                    .categories
                    .data
                    .iter()
                    .filter(|category_data| {
                        category_data.type_ == speedruncom_api::game::CategoryType::PerGame
                    })
                    .map(|category_data| FullRunCategory {
                        category_id: category_data.id.clone(),
                        name:        category_data.name.clone(),
                    })
                    .collect(),
                levels:               game
                    .levels
                    .data
                    .iter()
                    .map(|level_data| Level {
                        level_id: level_data.id.clone(),
                        name:     level_data.name.clone(),
                    })
                    .collect(),
                level_run_categories: game
                    .categories
                    .data
                    .iter()
                    .filter(|category_data| {
                        category_data.type_ == speedruncom_api::game::CategoryType::PerLevel
                    })
                    .map(|category_data| LevelRunCategory {
                        category_id: category_data.id.clone(),
                        name:        category_data.name.clone(),
                    })
                    .collect(),
            },
        );

        let mut next_runs_url = Some(format!(
            "{}/runs?game={}&embed=players&max=200",
            api, game_id
        ));

        while next_runs_url != None {
            let runs_url = next_runs_url.unwrap();

            debug!("Refreshing runs from {:?}.", runs_url);
            let mut runs_response = reqwest::get(&runs_url)?;
            if !runs_response.status().is_success() {
                return Err(NonSuccessResponseStatus {
                    status: runs_response.status(),
                    url:    runs_url,
                }
                .into())
            }
            let json = runs_response.text()?;
            let runs_data: speedruncom_api::runs::Response = serde_json::from_str(&json)?;

            next_runs_url = runs_data.next_page_url().clone();

            let runs = &mut self.data.get_mut().runs;
            for run in runs_data.data {
                runs.insert(
                    run.id.clone(),
                    Run {
                        run_id:      run.id,
                        status:      run.status.into(),
                        player:      run.players.into(),
                        game_id:     run.game,
                        level_id:    run.level,
                        category_id: run.category,
                        performed:   NaiveDate::from_str(
                            &run.date.unwrap_or_else(|| "1970-01-01".to_string()),
                        )?,
                        submitted:   DateTime::<Utc>::from_str(
                            &run.submitted
                                .unwrap_or_else(|| "1970-01-01T00:00:00Z".to_string()),
                        )?,
                        duration:    Duration::milliseconds(
                            (run.times.primary_t * 1000.0) as i64,
                        ),
                    },
                );
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
struct NonSuccessResponseStatus {
    status: reqwest::StatusCode,
    url:    String,
}
impl Error for NonSuccessResponseStatus {}
impl Display for NonSuccessResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields)]
struct Data {
    last_refreshed: Option<DateTime<Utc>>,
    games:          BTreeMap<String, Game>,
    runs:           BTreeMap<String, Run>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Game {
    pub game_id:              String,
    pub name:                 String,
    pub run_categories:       Vec<FullRunCategory>,
    pub levels:               Vec<Level>,
    pub level_run_categories: Vec<LevelRunCategory>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Level {
    pub level_id: String,
    pub name:     String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct FullRunCategory {
    pub category_id: String,
    pub name:        String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct LevelRunCategory {
    pub category_id: String,
    pub name:        String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Run {
    pub run_id:      String,
    pub game_id:     String,
    pub category_id: String,
    pub level_id:    Option<String>,
    pub status:      RunStatus,
    pub player:      Player,
    pub performed:   NaiveDate,
    pub submitted:   DateTime<Utc>,
    #[serde(serialize_with = "serialize_duration")]
    #[serde(deserialize_with = "deserialize_duration")]
    pub duration:    Duration,
}

fn serialize_duration<S>(x: &Duration, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    s.serialize_i64(x.num_milliseconds())
}

fn deserialize_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let ms: i64 = serde::Deserialize::deserialize(deserializer)?;
    Ok(Duration::milliseconds(ms))
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum Player {
    User {
        user_id:      String,
        name:         String,
        country_code: Option<String>,
    },
    Guest {
        name: String,
    },
    MultiplePlayers,
}
impl Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Player::User { name, .. } => name,
                Player::Guest { name } => name,
                Player::MultiplePlayers => "multiple players",
            }
        )
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum RunStatus {
    Pending,
    Verified,
    Rejected,
}

/// Internal types for speedrun.com API responses.
mod speedruncom_api {
    pub mod game {
        #[derive(Deserialize, Debug)]
        pub struct Response {
            pub data: Game,
        }

        #[derive(Deserialize, Debug)]
        pub struct Game {
            pub id:           String,
            pub names:        Names,
            pub abbreviation: String,
            pub levels:       Levels,
            pub categories:   Categories,
        }

        #[derive(Deserialize, Debug)]
        pub struct Names {
            pub international: String,
        }

        #[derive(Deserialize, Debug)]
        pub struct Levels {
            pub data: Vec<Level>,
        }

        #[derive(Deserialize, Debug)]
        pub struct Level {
            pub id:   String,
            pub name: String,
        }

        #[derive(Deserialize, Debug)]
        pub struct Categories {
            pub data: Vec<Category>,
        }

        #[derive(Deserialize, Debug)]
        pub struct Category {
            pub id:    String,
            pub name:  String,
            #[serde(rename = "type")]
            pub type_: CategoryType,
        }

        #[derive(Deserialize, Debug, PartialEq)]
        #[serde(rename_all = "kebab-case")]
        pub enum CategoryType {
            PerGame,
            PerLevel,
        }
    }

    pub mod runs {
        #[derive(Deserialize, Debug)]
        pub struct Response {
            pub data:       Vec<Run>,
            pub pagination: Pagination,
        }

        impl Response {
            pub fn next_page_url(&self) -> Option<String> {
                for link in self.pagination.links.iter() {
                    if let PaginationLink::Next { uri } = link {
                        return Some(uri.clone())
                    }
                }
                None
            }
        }

        #[derive(Deserialize, Debug)]
        pub struct Pagination {
            pub links: Vec<PaginationLink>,
        }

        #[derive(Deserialize, Debug)]
        #[serde(tag = "rel")]
        #[serde(rename_all = "kebab-case")]
        pub enum PaginationLink {
            Next { uri: String },
            Prev,
        }

        #[derive(Deserialize, Debug)]
        pub struct Run {
            pub id:        String,
            pub status:    RunStatus,
            pub game:      String,
            pub level:     Option<String>,
            pub category:  String,
            pub players:   PlayersData,
            pub date:      Option<String>,
            pub submitted: Option<String>,
            pub times:     Times,
        }

        #[derive(Deserialize, Debug)]
        #[serde(tag = "status")]
        #[serde(rename_all = "kebab-case")]
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
                        Player::User {
                            ref id,
                            ref names,
                            ref location,
                        } => super::super::Player::User {
                            user_id:      id.clone(),
                            name:         names.international.clone(),
                            country_code: match location {
                                None => None,
                                Some(location) => Some(location.country.code.clone()),
                            },
                        },
                        Player::Guest { ref name } =>
                            super::super::Player::Guest { name: name.clone() },
                    }
                }
            }
        }

        #[derive(Deserialize, Debug)]
        pub struct Times {
            pub primary_t: f64,
        }

        #[derive(Deserialize, Debug)]
        #[serde(tag = "rel")]
        #[serde(rename_all = "kebab-case")]
        pub enum Player {
            User {
                id:       String,
                names:    UserNames,
                location: Option<Location>,
            },
            Guest {
                name: String,
            },
        }

        #[derive(Deserialize, Debug)]
        pub struct UserNames {
            pub international: String,
        }

        #[derive(Deserialize, Debug)]
        pub struct Location {
            pub country: Country,
        }

        #[derive(Deserialize, Debug)]
        pub struct Country {
            pub code: String,
        }
    }
}
