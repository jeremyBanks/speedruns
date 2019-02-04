use chrono::{DateTime, NaiveDate, Utc};
use getset::Getters;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[get = "pub"]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Game {
    id: String,
    names: Names,
    abbreviation: String,
    weblink: String,
    release_date: NaiveDate,
    released: u32,
    romhack: bool,
    created: Option<DateTime<Utc>>,
    ruleset: Ruleset,
    platforms: Data<Vec<Platform>>,
    developers: Data<Vec<Developer>>,
    publishers: Data<Vec<Publisher>>,
    categories: Data<Vec<Category>>,
    engines: Data<Vec<Engine>>,
    levels: Data<Vec<Level>>,
    assets: HashMap<String, Option<Asset>>,
    gametypes: Data<Vec<GameType>>,
    variables: Data<Vec<Variable>>,
    moderators: HashMap<String, ModeratorType>,
    regions: Data<Vec<Region>>,
    genres: Data<Vec<Genre>>,
    links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[get = "pub"]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Category {
    id: String,
    name: String,
    miscellaneous: bool,
    #[serde(rename = "type")]
    type_: CategoryType,
    rules: Option<String>,
    players: CategoryPlayers,
    weblink: String,
    links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub enum CategoryType {
    PerGame,
    PerLevel,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[get = "pub"]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct CategoryPlayers {
    #[serde(rename = "type")]
    type_: CategoryPlayersType,
    value: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub enum CategoryPlayersType {
    Exactly,
    UpTo,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[get = "pub"]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Level {
    id: String,
    name: String,
    rules: Option<String>,
    weblink: String,
    links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum Timing {
    Realtime,
    RealtimeNoloads,
    Ingame,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub enum ModeratorType {
    SuperModerator,
    Moderator,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[get = "pub"]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Publisher {
    id: String,
    name: String,
    links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[get = "pub"]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct GameType {
    id: String,
    name: String,
    links: Vec<Link>,
    allows_base_game: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[get = "pub"]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Variable {
    id: String,
    name: String,
    links: Vec<Link>,
    is_subcategory: bool,
    category: Option<String>,
    scope: VariableScope,
    mandatory: bool,
    user_defined: bool,
    obsoletes: bool,
    values: VariableValues,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "type")]
#[serde(deny_unknown_fields)]
pub enum VariableScope {
    Global,
    FullGame,
    AllLevels,
    SingleLevel { level: String },
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[get = "pub"]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct VariableValues {
    default: Option<String>,
    values: HashMap<String, VariableValue>,
    #[deprecated]
    #[serde(rename = "_note")]
    _note: String,
    #[deprecated]
    choices: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[get = "pub"]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct VariableValue {
    label: String,
    rules: Option<String>,
    flags: Option<VariableValueFlags>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[get = "pub"]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct VariableValueFlags {
    miscellaneous: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[get = "pub"]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Region {
    id: String,
    name: String,
    links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[get = "pub"]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Genre {
    id: String,
    name: String,
    links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[get = "pub"]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Engine {
    id: String,
    name: String,
    links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[get = "pub"]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Developer {
    id: String,
    name: String,
    links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[get = "pub"]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Platform {
    id: String,
    name: String,
    released: u32,
    links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[get = "pub"]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Ruleset {
    default_time: Timing,
    emulators_allowed: bool,
    require_verification: bool,
    require_video: bool,
    run_times: Vec<Timing>,
    show_milliseconds: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(tag = "rel", content = "uri")]
pub enum Link {
    #[serde(rename = "self")]
    Self_(String),
    Leaderboard(String),
    Games(String),
    Game(String),
    Runs(String),
    Levels(String),
    Level(String),
    Categories(String),
    Category(String),
    Variables(String),
    Records(String),
    Series(String),
    DerivedGames(String),
    Romhacks(String),
    BaseGame(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[get = "pub"]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Names {
    international: String,
    japanese: Option<String>,
    twitch: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[get = "pub"]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Asset {
    uri: String,
    height: u32,
    width: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[get = "pub"]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Data<T> {
    data: T,
}

impl<T> std::ops::Deref for Data<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.data
    }
}
