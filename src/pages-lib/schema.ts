/* tslint:disable */
/* eslint-disable */
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
   * URL-safe slug, may differ from slug on speedrun.com
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
  category: GetGamePage_game_gameCategories_leaderboard_run_category;
  level: GetGamePage_game_gameCategories_leaderboard_run_level | null;
  date: number | null;
  players: GetGamePage_game_gameCategories_leaderboard_run_players[];
}

export interface GetGamePage_game_gameCategories_leaderboard {
  __typename: "LeaderboardRun";
  rank: number;
  timeMs: number;
  isTied: boolean;
  tiedRank: number;
  run: GetGamePage_game_gameCategories_leaderboard_run;
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
   * name, in English if possible
   */
  name: string;
  leaderboard: GetGamePage_game_gameCategories_leaderboard[];
}

export interface GetGamePage_game_levels_leaderboard_run_category {
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

export interface GetGamePage_game_levels_leaderboard_run_level {
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

export interface GetGamePage_game_levels_leaderboard_run_players_user {
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
   * URL-safe slug, may differ from slug on speedrun.com
   */
  slug: string;
}

export interface GetGamePage_game_levels_leaderboard_run_players {
  __typename: "Player";
  name: string;
  isGuest: boolean;
  user: GetGamePage_game_levels_leaderboard_run_players_user | null;
}

export interface GetGamePage_game_levels_leaderboard_run {
  __typename: "Run";
  /**
   * GraphQL node ID
   */
  id: string;
  /**
   * speedrun.com level ID
   */
  srcId: string;
  category: GetGamePage_game_levels_leaderboard_run_category;
  level: GetGamePage_game_levels_leaderboard_run_level | null;
  date: number | null;
  players: GetGamePage_game_levels_leaderboard_run_players[];
}

export interface GetGamePage_game_levels_leaderboard {
  __typename: "LeaderboardRun";
  rank: number;
  timeMs: number;
  isTied: boolean;
  tiedRank: number;
  run: GetGamePage_game_levels_leaderboard_run;
}

export interface GetGamePage_game_levels {
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
   * name, in English if possible
   */
  name: string;
  leaderboard: GetGamePage_game_levels_leaderboard[];
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
   * URL-safe slug, may differ from slug on speedrun.com
   */
  slug: string;
  /**
   * name, in English if possible
   */
  name: string;
  /**
   * full-game run categories
   */
  gameCategories: GetGamePage_game_gameCategories[];
  /**
   * individual levels
   */
  levels: GetGamePage_game_levels[];
}

export interface GetGamePage {
  game: GetGamePage_game | null;
}

export interface GetGamePageVariables {
  slug: string;
}

/* tslint:disable */
/* eslint-disable */
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
   * URL-safe slug, may differ from slug on speedrun.com
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
  category: GameLeaderboardRun_run_category;
  level: GameLeaderboardRun_run_level | null;
  date: number | null;
  players: GameLeaderboardRun_run_players[];
}

export interface GameLeaderboardRun {
  __typename: "LeaderboardRun";
  rank: number;
  timeMs: number;
  isTied: boolean;
  tiedRank: number;
  run: GameLeaderboardRun_run;
}

/* tslint:disable */
/* eslint-disable */
// This file was automatically generated and should not be edited.

//==============================================================
// START Enums and Input Objects
//==============================================================

//==============================================================
// END Enums and Input Objects
//==============================================================
