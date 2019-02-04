//! Types for all speedrun.com API resources we use.
//! <https://github.com/speedruncomorg/api/tree/master/version1>
//!
//! ## Style
//! - Everything in this file that isn't sensitive to ordering should be ordered
//!   alphabetically. That includes imports, top-level items, struct fields, and
//!   enum variants. It may not include attributes and attribute arguments.
//! - Subresources that are only used within a single other resource should get
//!   a derivative name, such as UserRole and CategoryPlayersType. Everything
//!   else should get a simple name, such as Game and Category.
use chrono::{DateTime, NaiveDate, Utc};
use derive_more::Deref;
use getset::Getters;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct Game {
    abbreviation: String,
    assets: HashMap<String, Option<Asset>>,
    categories: Data<Vec<Category>>,
    created: Option<DateTime<Utc>>,
    developers: Data<Vec<Developer>>,
    engines: Data<Vec<Engine>>,
    gametypes: Data<Vec<GameType>>,
    genres: Data<Vec<Genre>>,
    id: String,
    levels: Data<Vec<Level>>,
    links: Vec<Link>,
    moderators: HashMap<String, ModeratorType>,
    names: Names,
    platforms: Data<Vec<Platform>>,
    publishers: Data<Vec<Publisher>>,
    regions: Data<Vec<GameRegion>>,
    release_date: NaiveDate,
    released: u32,
    romhack: bool,
    ruleset: Ruleset,
    variables: Data<Vec<Variable>>,
    weblink: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct User {
    hitbox: Option<Uri>,
    id: String,
    links: Vec<Link>,
    location: Option<Location>,
    name_style: NameStyle,
    names: Names,
    role: UserRole,
    signup: Option<DateTime<Utc>>,
    speedrunslive: Option<Uri>,
    twitch: Option<Uri>,
    twitter: Option<Uri>,
    weblink: Option<String>,
    youtube: Option<Uri>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct Run {
    category: String,
    comment: Option<String>,
    date: Option<NaiveDate>,
    game: String,
    id: String,
    level: Option<String>,
    links: Vec<Link>,
    players: Vec<Player>,
    splits: Option<Splits>,
    status: RunStatus,
    submitted: Option<DateTime<Utc>>,
    system: System,
    times: Times,
    values: HashMap<String, String>,
    videos: Option<Videos>,
    weblink: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct Videos {
    links: Option<Vec<Uri>>,
    text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
#[get = "pub"]
pub struct Times {
    ingame_t: Option<f32>,
    ingame: Option<String>,
    primary_t: f32,
    primary: String,
    realtime_noloads_t: Option<f32>,
    realtime_noloads: Option<String>,
    realtime_t: Option<f32>,
    realtime: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "kebab-case", tag = "rel")]
pub enum Splits {
    #[serde(rename = "splits.io")]
    SplitsIo { uri: String },
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "kebab-case", tag = "rel")]
pub enum Player {
    Guest { name: String, uri: String },
    User { id: String, uri: String },
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "kebab-case", tag = "status")]
pub enum RunStatus {
    New,
    Rejected {
        examiner: Option<String>,
        reason: Option<String>,
    },
    Verified {
        examiner: Option<String>,
        #[serde(rename = "verify-date")]
        verify_date: Option<DateTime<Utc>>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct Location {
    country: Country,
    region: Option<UserRegion>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct Country {
    code: Option<String>,
    names: Names,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct UserRegion {
    code: Option<String>,
    names: Names,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "kebab-case", tag = "style")]
pub enum NameStyle {
    Gradient {
        #[serde(rename = "color-from")]
        color_from: ColorThemes,
        #[serde(rename = "color-to")]
        color_to: ColorThemes,
    },
    Solid {
        color: ColorThemes,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct ColorThemes {
    dark: String,
    light: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub enum UserRole {
    Admin,
    Banned,
    #[serde(rename = "contentmoderator")]
    ContentModerator,
    Moderator,
    Programmer,
    Trusted,
    User,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct Category {
    id: String,
    links: Vec<Link>,
    miscellaneous: bool,
    name: String,
    players: CategoryPlayers,
    rules: Option<String>,
    #[serde(rename = "type")]
    type_: CategoryType,
    weblink: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub enum CategoryType {
    PerGame,
    PerLevel,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct CategoryPlayers {
    #[serde(rename = "type")]
    type_: CategoryPlayersType,
    value: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub enum CategoryPlayersType {
    Exactly,
    UpTo,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct Level {
    id: String,
    links: Vec<Link>,
    name: String,
    rules: Option<String>,
    weblink: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum Timing {
    Ingame,
    Realtime,
    RealtimeNoloads,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub enum ModeratorType {
    Moderator,
    SuperModerator,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct Publisher {
    id: String,
    links: Vec<Link>,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct GameType {
    allows_base_game: Option<bool>,
    id: String,
    links: Vec<Link>,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct Variable {
    category: Option<String>,
    id: String,
    is_subcategory: bool,
    links: Vec<Link>,
    mandatory: bool,
    name: String,
    obsoletes: bool,
    scope: VariableScope,
    user_defined: bool,
    values: VariableValues,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "kebab-case", tag = "type")]
#[serde(deny_unknown_fields)]
pub enum VariableScope {
    AllLevels,
    FullGame,
    Global,
    SingleLevel { level: String },
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct VariableValues {
    #[serde(rename = "_note")]
    _note: String,
    choices: HashMap<String, String>,
    default: Option<String>,
    values: HashMap<String, VariableValue>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct VariableValue {
    flags: Option<VariableValueFlags>,
    label: String,
    rules: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct VariableValueFlags {
    miscellaneous: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct GameRegion {
    id: String,
    links: Vec<Link>,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct Genre {
    id: String,
    links: Vec<Link>,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct Engine {
    id: String,
    links: Vec<Link>,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct Developer {
    id: String,
    links: Vec<Link>,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct Platform {
    id: String,
    links: Vec<Link>,
    name: String,
    released: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct System {
    emulated: bool,
    platform: Option<String>,
    region: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct Ruleset {
    default_time: Timing,
    emulators_allowed: bool,
    require_verification: bool,
    require_video: bool,
    run_times: Vec<Timing>,
    show_milliseconds: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(
    deny_unknown_fields,
    rename_all = "kebab-case",
    tag = "rel",
    content = "uri"
)]
pub enum Link {
    BaseGame(String),
    Categories(String),
    Category(String),
    DerivedGames(String),
    Examiner(String),
    Game(String),
    Games(String),
    Leaderboard(String),
    Level(String),
    Levels(String),
    PersonalBests(String),
    Platform(String),
    Records(String),
    Region(String),
    Romhacks(String),
    Runs(String),
    #[serde(rename = "self")]
    Self_(String),
    Series(String),
    Variables(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct Names {
    international: Option<String>,
    japanese: Option<String>,
    twitch: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct Asset {
    height: u32,
    uri: String,
    width: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters, Deref)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct Data<T> {
    data: T,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters, Deref)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct Uri {
    uri: String,
}
