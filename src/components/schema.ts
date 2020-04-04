/* tslint:disable */
/* eslint-disable */
// @generated
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL query operation: GetGamePagePreview
// ====================================================

export interface GetGamePagePreview_game {
  __typename: "Game";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * URL slug
   */
  slug: string;
  /**
   * name, in English if possible
   */
  name: string;
}

export interface GetGamePagePreview {
  game: GetGamePagePreview_game | null;
}

export interface GetGamePagePreviewVariables {
  slug: string;
}

/* tslint:disable */
/* eslint-disable */
// @generated
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL query operation: GetGamePage
// ====================================================

export interface GetGamePage_game_gameCategories_leaderboard_run_category {
  __typename: "Category";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com category ID
   */
  srcId: string;
  /**
   * name, in English if possible
   */
  name: string;
}

export interface GetGamePage_game_gameCategories_leaderboard_run_level {
  __typename: "Level";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com level ID
   */
  srcId: string;
  /**
   * URL slug
   */
  slug: string;
  /**
   * name, in English if possible
   */
  name: string;
}

export interface GetGamePage_game_gameCategories_leaderboard_run_players_user {
  __typename: "User";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com user ID
   */
  srcId: string;
  /**
   * URL slug
   */
  slug: string;
}

export interface GetGamePage_game_gameCategories_leaderboard_run_players {
  __typename: "Player";
  name: string;
  isGuest: boolean;
  user: GetGamePage_game_gameCategories_leaderboard_run_players_user | null;
}

export interface GetGamePage_game_gameCategories_leaderboard_run {
  __typename: "Run";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com level ID
   */
  srcId: string;
  timeMs: number;
  videos: string[];
  category: GetGamePage_game_gameCategories_leaderboard_run_category;
  level: GetGamePage_game_gameCategories_leaderboard_run_level | null;
  date: number | null;
  players: GetGamePage_game_gameCategories_leaderboard_run_players[];
}

export interface GetGamePage_game_gameCategories_leaderboard {
  __typename: "LeaderboardRun";
  rank: number;
  isTied: boolean;
  tiedRank: number;
  run: GetGamePage_game_gameCategories_leaderboard_run;
}

export interface GetGamePage_game_gameCategories_progression_run_category {
  __typename: "Category";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com category ID
   */
  srcId: string;
  /**
   * name, in English if possible
   */
  name: string;
}

export interface GetGamePage_game_gameCategories_progression_run_level {
  __typename: "Level";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com level ID
   */
  srcId: string;
  /**
   * URL slug
   */
  slug: string;
  /**
   * name, in English if possible
   */
  name: string;
}

export interface GetGamePage_game_gameCategories_progression_run_players_user {
  __typename: "User";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com user ID
   */
  srcId: string;
  /**
   * URL slug
   */
  slug: string;
}

export interface GetGamePage_game_gameCategories_progression_run_players {
  __typename: "Player";
  name: string;
  isGuest: boolean;
  user: GetGamePage_game_gameCategories_progression_run_players_user | null;
}

export interface GetGamePage_game_gameCategories_progression_run {
  __typename: "Run";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com level ID
   */
  srcId: string;
  timeMs: number;
  videos: string[];
  category: GetGamePage_game_gameCategories_progression_run_category;
  level: GetGamePage_game_gameCategories_progression_run_level | null;
  date: number | null;
  players: GetGamePage_game_gameCategories_progression_run_players[];
}

export interface GetGamePage_game_gameCategories_progression_leaderboardRun_run_category {
  __typename: "Category";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com category ID
   */
  srcId: string;
  /**
   * name, in English if possible
   */
  name: string;
}

export interface GetGamePage_game_gameCategories_progression_leaderboardRun_run_level {
  __typename: "Level";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com level ID
   */
  srcId: string;
  /**
   * URL slug
   */
  slug: string;
  /**
   * name, in English if possible
   */
  name: string;
}

export interface GetGamePage_game_gameCategories_progression_leaderboardRun_run_players_user {
  __typename: "User";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com user ID
   */
  srcId: string;
  /**
   * URL slug
   */
  slug: string;
}

export interface GetGamePage_game_gameCategories_progression_leaderboardRun_run_players {
  __typename: "Player";
  name: string;
  isGuest: boolean;
  user: GetGamePage_game_gameCategories_progression_leaderboardRun_run_players_user | null;
}

export interface GetGamePage_game_gameCategories_progression_leaderboardRun_run {
  __typename: "Run";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com level ID
   */
  srcId: string;
  timeMs: number;
  videos: string[];
  category: GetGamePage_game_gameCategories_progression_leaderboardRun_run_category;
  level: GetGamePage_game_gameCategories_progression_leaderboardRun_run_level | null;
  date: number | null;
  players: GetGamePage_game_gameCategories_progression_leaderboardRun_run_players[];
}

export interface GetGamePage_game_gameCategories_progression_leaderboardRun {
  __typename: "LeaderboardRun";
  rank: number;
  isTied: boolean;
  tiedRank: number;
  run: GetGamePage_game_gameCategories_progression_leaderboardRun_run;
}

export interface GetGamePage_game_gameCategories_progression {
  __typename: "ProgressionRun";
  progressMs: number;
  run: GetGamePage_game_gameCategories_progression_run;
  leaderboardRun: GetGamePage_game_gameCategories_progression_leaderboardRun | null;
}

export interface GetGamePage_game_gameCategories {
  __typename: "Category";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com category ID
   */
  srcId: string;
  /**
   * URL slug
   */
  slug: string;
  /**
   * name, in English if possible
   */
  name: string;
  /**
   * leaderboards of ranked runs
   */
  leaderboard: GetGamePage_game_gameCategories_leaderboard[];
  /**
   * progress of record over time
   */
  progression: GetGamePage_game_gameCategories_progression[];
}

export interface GetGamePage_game_levelCategories_leaderboard_run_category {
  __typename: "Category";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com category ID
   */
  srcId: string;
  /**
   * name, in English if possible
   */
  name: string;
}

export interface GetGamePage_game_levelCategories_leaderboard_run_level {
  __typename: "Level";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com level ID
   */
  srcId: string;
  /**
   * URL slug
   */
  slug: string;
  /**
   * name, in English if possible
   */
  name: string;
}

export interface GetGamePage_game_levelCategories_leaderboard_run_players_user {
  __typename: "User";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com user ID
   */
  srcId: string;
  /**
   * URL slug
   */
  slug: string;
}

export interface GetGamePage_game_levelCategories_leaderboard_run_players {
  __typename: "Player";
  name: string;
  isGuest: boolean;
  user: GetGamePage_game_levelCategories_leaderboard_run_players_user | null;
}

export interface GetGamePage_game_levelCategories_leaderboard_run {
  __typename: "Run";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com level ID
   */
  srcId: string;
  timeMs: number;
  videos: string[];
  category: GetGamePage_game_levelCategories_leaderboard_run_category;
  level: GetGamePage_game_levelCategories_leaderboard_run_level | null;
  date: number | null;
  players: GetGamePage_game_levelCategories_leaderboard_run_players[];
}

export interface GetGamePage_game_levelCategories_leaderboard {
  __typename: "LeaderboardRun";
  rank: number;
  isTied: boolean;
  tiedRank: number;
  run: GetGamePage_game_levelCategories_leaderboard_run;
}

export interface GetGamePage_game_levelCategories_levels_level {
  __typename: "Level";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com level ID
   */
  srcId: string;
  /**
   * URL slug
   */
  slug: string;
  /**
   * name, in English if possible
   */
  name: string;
}

export interface GetGamePage_game_levelCategories_levels_leaderboard_run_category {
  __typename: "Category";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com category ID
   */
  srcId: string;
  /**
   * name, in English if possible
   */
  name: string;
}

export interface GetGamePage_game_levelCategories_levels_leaderboard_run_level {
  __typename: "Level";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com level ID
   */
  srcId: string;
  /**
   * URL slug
   */
  slug: string;
  /**
   * name, in English if possible
   */
  name: string;
}

export interface GetGamePage_game_levelCategories_levels_leaderboard_run_players_user {
  __typename: "User";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com user ID
   */
  srcId: string;
  /**
   * URL slug
   */
  slug: string;
}

export interface GetGamePage_game_levelCategories_levels_leaderboard_run_players {
  __typename: "Player";
  name: string;
  isGuest: boolean;
  user: GetGamePage_game_levelCategories_levels_leaderboard_run_players_user | null;
}

export interface GetGamePage_game_levelCategories_levels_leaderboard_run {
  __typename: "Run";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com level ID
   */
  srcId: string;
  timeMs: number;
  videos: string[];
  category: GetGamePage_game_levelCategories_levels_leaderboard_run_category;
  level: GetGamePage_game_levelCategories_levels_leaderboard_run_level | null;
  date: number | null;
  players: GetGamePage_game_levelCategories_levels_leaderboard_run_players[];
}

export interface GetGamePage_game_levelCategories_levels_leaderboard {
  __typename: "LeaderboardRun";
  rank: number;
  isTied: boolean;
  tiedRank: number;
  run: GetGamePage_game_levelCategories_levels_leaderboard_run;
}

export interface GetGamePage_game_levelCategories_levels_progression_run_category {
  __typename: "Category";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com category ID
   */
  srcId: string;
  /**
   * name, in English if possible
   */
  name: string;
}

export interface GetGamePage_game_levelCategories_levels_progression_run_level {
  __typename: "Level";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com level ID
   */
  srcId: string;
  /**
   * URL slug
   */
  slug: string;
  /**
   * name, in English if possible
   */
  name: string;
}

export interface GetGamePage_game_levelCategories_levels_progression_run_players_user {
  __typename: "User";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com user ID
   */
  srcId: string;
  /**
   * URL slug
   */
  slug: string;
}

export interface GetGamePage_game_levelCategories_levels_progression_run_players {
  __typename: "Player";
  name: string;
  isGuest: boolean;
  user: GetGamePage_game_levelCategories_levels_progression_run_players_user | null;
}

export interface GetGamePage_game_levelCategories_levels_progression_run {
  __typename: "Run";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com level ID
   */
  srcId: string;
  timeMs: number;
  videos: string[];
  category: GetGamePage_game_levelCategories_levels_progression_run_category;
  level: GetGamePage_game_levelCategories_levels_progression_run_level | null;
  date: number | null;
  players: GetGamePage_game_levelCategories_levels_progression_run_players[];
}

export interface GetGamePage_game_levelCategories_levels_progression_leaderboardRun_run_category {
  __typename: "Category";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com category ID
   */
  srcId: string;
  /**
   * name, in English if possible
   */
  name: string;
}

export interface GetGamePage_game_levelCategories_levels_progression_leaderboardRun_run_level {
  __typename: "Level";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com level ID
   */
  srcId: string;
  /**
   * URL slug
   */
  slug: string;
  /**
   * name, in English if possible
   */
  name: string;
}

export interface GetGamePage_game_levelCategories_levels_progression_leaderboardRun_run_players_user {
  __typename: "User";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com user ID
   */
  srcId: string;
  /**
   * URL slug
   */
  slug: string;
}

export interface GetGamePage_game_levelCategories_levels_progression_leaderboardRun_run_players {
  __typename: "Player";
  name: string;
  isGuest: boolean;
  user: GetGamePage_game_levelCategories_levels_progression_leaderboardRun_run_players_user | null;
}

export interface GetGamePage_game_levelCategories_levels_progression_leaderboardRun_run {
  __typename: "Run";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com level ID
   */
  srcId: string;
  timeMs: number;
  videos: string[];
  category: GetGamePage_game_levelCategories_levels_progression_leaderboardRun_run_category;
  level: GetGamePage_game_levelCategories_levels_progression_leaderboardRun_run_level | null;
  date: number | null;
  players: GetGamePage_game_levelCategories_levels_progression_leaderboardRun_run_players[];
}

export interface GetGamePage_game_levelCategories_levels_progression_leaderboardRun {
  __typename: "LeaderboardRun";
  rank: number;
  isTied: boolean;
  tiedRank: number;
  run: GetGamePage_game_levelCategories_levels_progression_leaderboardRun_run;
}

export interface GetGamePage_game_levelCategories_levels_progression {
  __typename: "ProgressionRun";
  progressMs: number;
  run: GetGamePage_game_levelCategories_levels_progression_run;
  leaderboardRun: GetGamePage_game_levelCategories_levels_progression_leaderboardRun | null;
}

export interface GetGamePage_game_levelCategories_levels {
  __typename: "CategoryLevel";
  /**
   * the level
   */
  level: GetGamePage_game_levelCategories_levels_level;
  /**
   * leaderboards of ranked runs
   */
  leaderboard: GetGamePage_game_levelCategories_levels_leaderboard[];
  /**
   * progress of record over time
   */
  progression: GetGamePage_game_levelCategories_levels_progression[];
}

export interface GetGamePage_game_levelCategories {
  __typename: "Category";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com category ID
   */
  srcId: string;
  /**
   * URL slug
   */
  slug: string;
  /**
   * name, in English if possible
   */
  name: string;
  /**
   * leaderboards of ranked runs
   */
  leaderboard: GetGamePage_game_levelCategories_leaderboard[];
  /**
   * category-level links
   */
  levels: GetGamePage_game_levelCategories_levels[];
}

export interface GetGamePage_game {
  __typename: "Game";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com game ID
   */
  srcId: string;
  /**
   * URL slug
   */
  slug: string;
  /**
   * name, in English if possible
   */
  name: string;
  /**
   * primary run timing method used for this game
   */
  timingMethod: TimingMethod;
  /**
   * full-game run categories
   */
  gameCategories: GetGamePage_game_gameCategories[];
  /**
   * individual level run categories
   */
  levelCategories: GetGamePage_game_levelCategories[];
}

export interface GetGamePage {
  game: GetGamePage_game | null;
}

export interface GetGamePageVariables {
  slug: string;
}

/* tslint:disable */
/* eslint-disable */
// @generated
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL query operation: GetRunPage
// ====================================================

export interface GetRunPage_game {
  __typename: "Game";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com game ID
   */
  srcId: string;
  /**
   * URL slug
   */
  slug: string;
  /**
   * name, in English if possible
   */
  name: string;
}

export interface GetRunPage_run_category {
  __typename: "Category";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com category ID
   */
  srcId: string;
  /**
   * name, in English if possible
   */
  name: string;
}

export interface GetRunPage_run_level {
  __typename: "Level";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com level ID
   */
  srcId: string;
  /**
   * URL slug
   */
  slug: string;
  /**
   * name, in English if possible
   */
  name: string;
}

export interface GetRunPage_run_players_user {
  __typename: "User";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com user ID
   */
  srcId: string;
  /**
   * URL slug
   */
  slug: string;
}

export interface GetRunPage_run_players {
  __typename: "Player";
  name: string;
  isGuest: boolean;
  user: GetRunPage_run_players_user | null;
}

export interface GetRunPage_run {
  __typename: "Run";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com level ID
   */
  srcId: string;
  timeMs: number;
  videos: string[];
  category: GetRunPage_run_category;
  level: GetRunPage_run_level | null;
  date: number | null;
  players: GetRunPage_run_players[];
}

export interface GetRunPage {
  game: GetRunPage_game | null;
  run: GetRunPage_run | null;
}

export interface GetRunPageVariables {
  gameSrcSlug: string;
  runSrcId: string;
}

/* tslint:disable */
/* eslint-disable */
// @generated
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL query operation: GetHomeStats
// ====================================================

export interface GetHomeStats_stats {
  __typename: "Stats";
  /**
   * timestamp of last update
   */
  lastUpdated: number;
  /**
   * number of runs
   */
  runs: number;
  /**
   * number of games
   */
  games: number;
  /**
   * the version of this server package handling this request
   */
  version: string;
}

export interface GetHomeStats {
  stats: GetHomeStats_stats;
  /**
   * a random value.
   */
  seed: number;
}

/* tslint:disable */
/* eslint-disable */
// @generated
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL query operation: GetGameIndex
// ====================================================

export interface GetGameIndex_games {
  __typename: "Game";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * URL slug
   */
  slug: string;
  /**
   * name, in English if possible
   */
  name: string;
}

export interface GetGameIndex {
  games: GetGameIndex_games[];
}

/* tslint:disable */
/* eslint-disable */
// @generated
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL query operation: GetNodePage
// ====================================================

export interface GetNodePage_node_Game {
  __typename: "Game";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com game ID
   */
  srcId: string;
  /**
   * URL slug
   */
  slug: string;
  /**
   * name, in English if possible
   */
  name: string;
}

export interface GetNodePage_node_User {
  __typename: "User";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com user ID
   */
  srcId: string;
  /**
   * URL slug
   */
  slug: string;
}

export interface GetNodePage_node_Level {
  __typename: "Level";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com level ID
   */
  srcId: string;
  /**
   * URL slug
   */
  slug: string;
  /**
   * name, in English if possible
   */
  name: string;
}

export interface GetNodePage_node_Category {
  __typename: "Category";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com category ID
   */
  srcId: string;
  /**
   * URL slug
   */
  slug: string;
  /**
   * name, in English if possible
   */
  name: string;
}

export interface GetNodePage_node_Run_category {
  __typename: "Category";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com category ID
   */
  srcId: string;
}

export interface GetNodePage_node_Run_level {
  __typename: "Level";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com level ID
   */
  srcId: string;
}

export interface GetNodePage_node_Run {
  __typename: "Run";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com level ID
   */
  srcId: string;
  date: number | null;
  category: GetNodePage_node_Run_category;
  level: GetNodePage_node_Run_level | null;
  timeMs: number;
}

export type GetNodePage_node =
  | GetNodePage_node_Game
  | GetNodePage_node_User
  | GetNodePage_node_Level
  | GetNodePage_node_Category
  | GetNodePage_node_Run;

export interface GetNodePage {
  node: GetNodePage_node | null;
}

export interface GetNodePageVariables {
  id: string;
}

/* tslint:disable */
/* eslint-disable */
// @generated
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL fragment: GameRun
// ====================================================

export interface GameRun_category {
  __typename: "Category";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com category ID
   */
  srcId: string;
  /**
   * name, in English if possible
   */
  name: string;
}

export interface GameRun_level {
  __typename: "Level";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com level ID
   */
  srcId: string;
  /**
   * URL slug
   */
  slug: string;
  /**
   * name, in English if possible
   */
  name: string;
}

export interface GameRun_players_user {
  __typename: "User";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com user ID
   */
  srcId: string;
  /**
   * URL slug
   */
  slug: string;
}

export interface GameRun_players {
  __typename: "Player";
  name: string;
  isGuest: boolean;
  user: GameRun_players_user | null;
}

export interface GameRun {
  __typename: "Run";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com level ID
   */
  srcId: string;
  timeMs: number;
  videos: string[];
  category: GameRun_category;
  level: GameRun_level | null;
  date: number | null;
  players: GameRun_players[];
}

/* tslint:disable */
/* eslint-disable */
// @generated
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL fragment: GameLeaderboardRun
// ====================================================

export interface GameLeaderboardRun_run_category {
  __typename: "Category";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com category ID
   */
  srcId: string;
  /**
   * name, in English if possible
   */
  name: string;
}

export interface GameLeaderboardRun_run_level {
  __typename: "Level";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com level ID
   */
  srcId: string;
  /**
   * URL slug
   */
  slug: string;
  /**
   * name, in English if possible
   */
  name: string;
}

export interface GameLeaderboardRun_run_players_user {
  __typename: "User";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com user ID
   */
  srcId: string;
  /**
   * URL slug
   */
  slug: string;
}

export interface GameLeaderboardRun_run_players {
  __typename: "Player";
  name: string;
  isGuest: boolean;
  user: GameLeaderboardRun_run_players_user | null;
}

export interface GameLeaderboardRun_run {
  __typename: "Run";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com level ID
   */
  srcId: string;
  timeMs: number;
  videos: string[];
  category: GameLeaderboardRun_run_category;
  level: GameLeaderboardRun_run_level | null;
  date: number | null;
  players: GameLeaderboardRun_run_players[];
}

export interface GameLeaderboardRun {
  __typename: "LeaderboardRun";
  rank: number;
  isTied: boolean;
  tiedRank: number;
  run: GameLeaderboardRun_run;
}

/* tslint:disable */
/* eslint-disable */
// @generated
// This file was automatically generated and should not be edited.

//==============================================================
// START Enums and Input Objects
//==============================================================

/**
 * A timing method that can be used to time a run.
 */
export enum TimingMethod {
  IGT = "IGT",
  RTA = "RTA",
  RTA_NL = "RTA_NL",
}

//==============================================================
// END Enums and Input Objects
//==============================================================
