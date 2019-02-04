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
    regions: Data<Vec<GameRegion>>,
    genres: Data<Vec<Genre>>,
    weblink: String,
    links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[get = "pub"]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct User {
    id: String,
    names: Names,
    twitch: Option<Uri>,
    twitter: Option<Uri>,
    youtube: Option<Uri>,
    hitbox: Option<Uri>,
    speedrunslive: Option<Uri>,
    signup: Option<DateTime<Utc>>,
    location: Option<Location>,
    role: UserRole,
    name_style: NameStyle,
    weblink: Option<String>,
    links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[get = "pub"]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Run {
    id: String,
    date: Option<NaiveDate>,
    submitted: Option<DateTime<Utc>>,
    videos: Option<Videos>,
    category: String,
    game: String,
    system: System,
    players: Vec<Player>,
    comment: Option<String>,
    level: Option<String>,
    splits: Option<Splits>,
    status: RunStatus,
    times: RunTimes,
    values: HashMap<String, String>,
    weblink: Option<String>,
    links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[get = "pub"]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Videos {
    text: Option<String>,
    links: Option<Vec<Uri>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[get = "pub"]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct RunTimes {
    // as an ISO 8601 duration
    primary: String,
    // as a number of seconds
    primary_t: f32,
    realtime: Option<String>,
    realtime_t: Option<f32>,
    realtime_noloads: Option<String>,
    realtime_noloads_t: Option<f32>,
    ingame: Option<String>,
    ingame_t: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(tag = "rel")]
pub enum Splits {
    #[serde(rename = "splits.io")]
    SplitsIo { uri: String },
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(tag = "rel")]
pub enum Player {
    User { id: String, uri: String },
    Guest { name: String, uri: String },
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(tag = "status")]
pub enum RunStatus {
    New,
    Verified {
        examiner: Option<String>,
        #[serde(rename = "verify-date")]
        verify_date: Option<DateTime<Utc>>,
    },
    Rejected {
        examiner: Option<String>,
        reason: Option<String>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[get = "pub"]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Location {
    country: Country,
    region: Option<UserRegion>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[get = "pub"]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Country {
    code: Option<String>,
    names: Names,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[get = "pub"]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct UserRegion {
    code: Option<String>,
    names: Names,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(tag = "style")]
pub enum NameStyle {
    Solid {
        color: ColorThemes,
    },
    Gradient {
        #[serde(rename = "color-from")]
        color_from: ColorThemes,
        #[serde(rename = "color-to")]
        color_to: ColorThemes,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[get = "pub"]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct ColorThemes {
    light: String,
    dark: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub enum UserRole {
    Banned,
    User,
    Trusted,
    Moderator,
    Admin,
    Programmer,
    #[serde(rename = "contentmoderator")]
    ContentModerator,
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
    is_subcategory: bool,
    category: Option<String>,
    scope: VariableScope,
    mandatory: bool,
    user_defined: bool,
    obsoletes: bool,
    values: VariableValues,
    links: Vec<Link>,
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
    #[serde(rename = "_note")]
    _note: String,
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
pub struct GameRegion {
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
pub struct System {
    platform: Option<String>,
    emulated: bool,
    region: Option<String>,
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
    PersonalBests(String),
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
    Examiner(String),
    Platform(String),
    Region(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[get = "pub"]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Names {
    international: Option<String>,
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

impl<T: Default> std::default::Default for Data<T> {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[get = "pub"]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Uri {
    uri: String,
}

impl std::ops::Deref for Uri {
    type Target = String;

    fn deref(&self) -> &String {
        &self.uri
    }
}
