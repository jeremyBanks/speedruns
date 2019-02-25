//! Types for all speedrun.com API resources we use.
//! <https://github.com/speedruncomorg/api/tree/master/version1>
#![allow(missing_docs)]
use chrono::{DateTime, NaiveDate, Utc};
use derive_more::Deref;
use getset::Getters;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[remain::sorted]
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

#[remain::sorted]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct CategoryPlayers {
    #[serde(rename = "type")]
    type_: CategoryPlayersType,
    value: u32,
}

#[remain::sorted]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub enum CategoryPlayersType {
    Exactly,
    UpTo,
}

#[remain::sorted]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub enum CategoryType {
    PerGame,
    PerLevel,
}

#[remain::sorted]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters, Deref)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct Data<T> {
    data: T,
}

#[remain::sorted]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct Game {
    abbreviation: String,
    assets:       HashMap<String, Option<GameAsset>>,
    categories:   Data<Vec<Category>>,
    created:      Option<DateTime<Utc>>,
    developers:   Data<Vec<GameDeveloper>>,
    engines:      Data<Vec<GameEngine>>,
    gametypes:    Data<Vec<GameType>>,
    genres:       Data<Vec<GameGenre>>,
    id:           String,
    levels:       Data<Vec<Level>>,
    links:        Vec<Link>,
    moderators:   HashMap<String, GameModeratorType>,
    names:        Names,
    platforms:    Data<Vec<Platform>>,
    publishers:   Data<Vec<GamePublisher>>,
    regions:      Data<Vec<Region>>,
    release_date: NaiveDate,
    released:     u32,
    romhack:      bool,
    ruleset:      GameRuleset,
    variables:    Data<Vec<Variable>>,
    weblink:      String,
}

#[remain::sorted]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct GameAsset {
    height: u32,
    uri:    String,
    width:  u32,
}

#[remain::sorted]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct GameDeveloper {
    id:    String,
    links: Vec<Link>,
    name:  String,
}

#[remain::sorted]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct GameEngine {
    id:    String,
    links: Vec<Link>,
    name:  String,
}

#[remain::sorted]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct GameGenre {
    id:    String,
    links: Vec<Link>,
    name:  String,
}

#[remain::sorted]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub enum GameModeratorType {
    Moderator,
    SuperModerator,
}

#[remain::sorted]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct GamePublisher {
    id:    String,
    links: Vec<Link>,
    name:  String,
}

#[remain::sorted]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct GameRuleset {
    default_time:         GameRulesetTiming,
    emulators_allowed:    bool,
    require_verification: bool,
    require_video:        bool,
    run_times:            Vec<GameRulesetTiming>,
    show_milliseconds:    bool,
}

#[remain::sorted]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
#[allow(non_camel_case_types)]
pub enum GameRulesetTiming {
    #[serde(rename = "ingame")]
    IGT,
    #[serde(rename = "realtime")]
    RTA,
    #[serde(rename = "realtime_noloads")]
    RTA_NL,
}

#[remain::sorted]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct GameType {
    allows_base_game: Option<bool>,
    id:               String,
    links:            Vec<Link>,
    name:             String,
}

#[remain::sorted]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct Level {
    id:      String,
    links:   Vec<Link>,
    name:    String,
    rules:   Option<String>,
    weblink: String,
}

#[remain::sorted]
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

#[remain::sorted]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct Names {
    international: Option<String>,
    japanese:      Option<String>,
    twitch:        Option<String>,
}

#[remain::sorted]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct Platform {
    id:       String,
    links:    Vec<Link>,
    name:     String,
    released: u32,
}

#[remain::sorted]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct Region {
    id:    String,
    links: Vec<Link>,
    name:  String,
}

#[remain::sorted]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct Run {
    category:  String,
    comment:   Option<String>,
    date:      Option<NaiveDate>,
    game:      String,
    id:        String,
    level:     Option<String>,
    links:     Vec<Link>,
    players:   Vec<RunPlayer>,
    splits:    Option<RunSplitsOrBuggyValue>,
    status:    RunStatus,
    submitted: Option<DateTime<Utc>>,
    system:    RunSystem,
    times:     RunTimes,
    values:    HashMap<String, String>,
    videos:    Option<RunVideos>,
    weblink:   Option<String>,
}

#[remain::sorted]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "kebab-case", tag = "rel")]
pub enum RunPlayer {
    Guest { name: String, uri: String },
    User { id: String, uri: String },
}

#[remain::sorted]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "kebab-case", tag = "rel")]
pub enum RunSplits {
    #[serde(rename = "splits.io")]
    RunSplitsIo { uri: String },
}

#[remain::sorted]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "kebab-case", untagged)]
pub enum RunSplitsOrBuggyValue {
    BuggyValue(String),
    RunSplits(RunSplits),
}

#[remain::sorted]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "kebab-case", tag = "status")]
pub enum RunStatus {
    New,
    Rejected {
        examiner: Option<String>,
        reason:   Option<String>,
    },
    Verified {
        examiner: Option<String>,
        #[serde(rename = "verify-date")]
        verify_date: Option<DateTime<Utc>>,
    },
}

#[remain::sorted]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct RunSystem {
    emulated: bool,
    platform: Option<String>,
    region:   Option<String>,
}

#[remain::sorted]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
#[get = "pub"]
pub struct RunTimes {
    ingame:             Option<String>,
    ingame_t:           Option<f32>,
    primary:            Option<String>,
    primary_t:          Option<f32>,
    realtime:           Option<String>,
    realtime_noloads:   Option<String>,
    realtime_noloads_t: Option<f32>,
    realtime_t:         Option<f32>,
}

#[remain::sorted]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct RunVideos {
    links: Option<Vec<Uri>>,
    text:  Option<String>,
}

#[remain::sorted]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters, Deref)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct Uri {
    uri: String,
}

#[remain::sorted]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct User {
    hitbox:        Option<Uri>,
    id:            String,
    links:         Vec<Link>,
    location:      Option<UserLocation>,
    name_style:    UserNameStyle,
    names:         Names,
    role:          UserRole,
    signup:        Option<DateTime<Utc>>,
    speedrunslive: Option<Uri>,
    twitch:        Option<Uri>,
    twitter:       Option<Uri>,
    weblink:       Option<String>,
    youtube:       Option<Uri>,
}

#[remain::sorted]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct UserLocation {
    country: UserLocationCountry,
    region:  Option<UserLocationRegion>,
}

#[remain::sorted]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct UserLocationCountry {
    code:  Option<String>,
    names: Names,
}

#[remain::sorted]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct UserLocationRegion {
    code:  Option<String>,
    names: Names,
}

#[remain::sorted]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "kebab-case", tag = "style")]
pub enum UserNameStyle {
    Gradient {
        #[serde(rename = "color-from")]
        color_from: UserNameStyleColor,
        #[serde(rename = "color-to")]
        color_to: UserNameStyleColor,
    },
    Solid {
        color: UserNameStyleColor,
    },
}

#[remain::sorted]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct UserNameStyleColor {
    dark:  String,
    light: String,
}

#[remain::sorted]
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

#[remain::sorted]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct Variable {
    category:       Option<String>,
    id:             String,
    is_subcategory: bool,
    links:          Vec<Link>,
    mandatory:      bool,
    name:           String,
    obsoletes:      bool,
    scope:          VariableScope,
    user_defined:   bool,
    values:         VariableValues,
}

#[remain::sorted]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "kebab-case", tag = "type")]
pub enum VariableScope {
    AllLevels,
    FullGame,
    Global,
    SingleLevel { level: String },
}

#[remain::sorted]
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

#[remain::sorted]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct VariableValue {
    flags: Option<VariableValueFlags>,
    label: String,
    rules: Option<String>,
}

#[remain::sorted]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Getters)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[get = "pub"]
pub struct VariableValueFlags {
    miscellaneous: Option<bool>,
}
