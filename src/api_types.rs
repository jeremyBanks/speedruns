use chrono::{DateTime, NaiveDate, Utc};
use getset::Getters;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn is_default<T: Default + PartialEq>(x: &T) -> bool {
    *x == T::default()
}

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
    #[serde(default, skip_serializing_if = "is_default")]
    created: Option<DateTime<Utc>>,
    ruleset: Ruleset,
    #[serde(default, skip_serializing_if = "is_default")]
    platforms: Data<Vec<Platform>>,
    #[serde(default, skip_serializing_if = "is_default")]
    developers: Data<Vec<Developer>>,
    #[serde(default, skip_serializing_if = "is_default")]
    publishers: Data<Vec<Publisher>>,
    #[serde(default, skip_serializing_if = "is_default")]
    categories: Data<Vec<Category>>,
    #[serde(default, skip_serializing_if = "is_default")]
    engines: Data<Vec<Engine>>,
    #[serde(default, skip_serializing_if = "is_default")]
    levels: Data<Vec<Level>>,
    #[serde(default, skip_serializing_if = "is_default")]
    assets: HashMap<String, Option<Asset>>,
    #[serde(default, skip_serializing_if = "is_default")]
    gametypes: Data<Vec<GameType>>,
    #[serde(default, skip_serializing_if = "is_default")]
    variables: Data<Vec<Variable>>,
    #[serde(default, skip_serializing_if = "is_default")]
    moderators: HashMap<String, ModeratorType>,
    #[serde(default, skip_serializing_if = "is_default")]
    regions: Data<Vec<GameRegion>>,
    #[serde(default, skip_serializing_if = "is_default")]
    genres: Data<Vec<Genre>>,
    weblink: String,
    #[serde(default, skip_serializing_if = "is_default")]
    links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[get = "pub"]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct User {
    id: String,
    names: Names,
    #[serde(default, skip_serializing_if = "is_default")]
    twitch: Option<Uri<String>>,
    #[serde(default, skip_serializing_if = "is_default")]
    twitter: Option<Uri<String>>,
    #[serde(default, skip_serializing_if = "is_default")]
    youtube: Option<Uri<String>>,
    #[serde(default, skip_serializing_if = "is_default")]
    hitbox: Option<Uri<String>>,
    #[serde(default, skip_serializing_if = "is_default")]
    weblink: Option<String>,
    #[serde(default, skip_serializing_if = "is_default")]
    speedrunslive: Option<Uri<String>>,
    #[serde(default, skip_serializing_if = "is_default")]
    signup: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "is_default")]
    location: Option<Location>,
    role: UserRole,
    name_style: NameStyle,
    #[serde(default, skip_serializing_if = "is_default")]
    links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[get = "pub"]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Run {
    id: String,
    date: NaiveDate,
    category: String,
    comment: String,
    #[serde(default, skip_serializing_if = "is_default")]
    level: Option<String>,
    #[serde(default, skip_serializing_if = "is_default")]
    split: Option<String>,
    #[serde(default, skip_serializing_if = "is_default")]
    links: Vec<Link>,
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
    #[serde(default, skip_serializing_if = "is_default")]
    code: Option<String>,
    names: Names,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[get = "pub"]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct UserRegion {
    #[serde(default, skip_serializing_if = "is_default")]
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
    #[serde(default, skip_serializing_if = "is_default")]
    rules: Option<String>,
    players: CategoryPlayers,
    weblink: String,
    #[serde(default, skip_serializing_if = "is_default")]
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
    #[serde(default, skip_serializing_if = "is_default")]
    rules: Option<String>,
    weblink: String,
    #[serde(default, skip_serializing_if = "is_default")]
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
    #[serde(default, skip_serializing_if = "is_default")]
    links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[get = "pub"]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct GameType {
    id: String,
    name: String,
    #[serde(default, skip_serializing_if = "is_default")]
    links: Vec<Link>,
    #[serde(default, skip_serializing_if = "is_default")]
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
    #[serde(default, skip_serializing_if = "is_default")]
    category: Option<String>,
    scope: VariableScope,
    mandatory: bool,
    user_defined: bool,
    obsoletes: bool,
    values: VariableValues,
    #[serde(default, skip_serializing_if = "is_default")]
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
    #[serde(default, skip_serializing_if = "is_default")]
    values: HashMap<String, VariableValue>,
    #[deprecated]
    #[serde(rename = "_note")]
    _note: String,
    #[deprecated]
    #[serde(default, skip_serializing_if = "is_default")]
    choices: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[get = "pub"]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct VariableValue {
    label: String,
    #[serde(default, skip_serializing_if = "is_default")]
    rules: Option<String>,
    #[serde(default, skip_serializing_if = "is_default")]
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
    #[serde(default, skip_serializing_if = "is_default")]
    links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[get = "pub"]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Genre {
    id: String,
    name: String,
    #[serde(default, skip_serializing_if = "is_default")]
    links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[get = "pub"]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Engine {
    id: String,
    name: String,
    #[serde(default, skip_serializing_if = "is_default")]
    links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[get = "pub"]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Developer {
    id: String,
    name: String,
    #[serde(default, skip_serializing_if = "is_default")]
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
    #[serde(default, skip_serializing_if = "is_default")]
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
    #[serde(default, skip_serializing_if = "is_default")]
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
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[get = "pub"]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Names {
    #[serde(default, skip_serializing_if = "is_default")]
    international: Option<String>,
    #[serde(default, skip_serializing_if = "is_default")]
    japanese: Option<String>,
    #[serde(default, skip_serializing_if = "is_default")]
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
pub struct Uri<T> {
    uri: T,
}

impl<T> std::ops::Deref for Uri<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.uri
    }
}
