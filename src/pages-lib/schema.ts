/* tslint:disable */
/* eslint-disable */
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL query operation: GetGamePage
// ====================================================

export interface GetGamePage_game_categories_leaderboard_run_category {
  __typename: "Category";
  id: string;
}

export interface GetGamePage_game_categories_leaderboard_run_level {
  __typename: "Level";
  id: string;
}

export interface GetGamePage_game_categories_leaderboard_run_players_user {
  __typename: "User";
  id: string;
  slug: string;
}

export interface GetGamePage_game_categories_leaderboard_run_players {
  __typename: "Player";
  name: string;
  isGuest: boolean;
  user: GetGamePage_game_categories_leaderboard_run_players_user | null;
}

export interface GetGamePage_game_categories_leaderboard_run {
  __typename: "Run";
  id: string;
  category: GetGamePage_game_categories_leaderboard_run_category;
  level: GetGamePage_game_categories_leaderboard_run_level | null;
  date: number | null;
  players: GetGamePage_game_categories_leaderboard_run_players[];
}

export interface GetGamePage_game_categories_leaderboard {
  __typename: "LeaderboardRun";
  rank: number;
  timeMs: number;
  isTied: boolean;
  tiedRank: number;
  run: GetGamePage_game_categories_leaderboard_run;
}

export interface GetGamePage_game_categories {
  __typename: "Category";
  id: string;
  name: string;
  leaderboard: GetGamePage_game_categories_leaderboard[];
}

export interface GetGamePage_game_levels {
  __typename: "Level";
  id: string;
  name: string;
}

export interface GetGamePage_game {
  __typename: "Game";
  id: string;
  slug: string;
  name: string;
  categories: GetGamePage_game_categories[];
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
// GraphQL query operation: GetRun
// ====================================================

export interface GetRun_run {
  __typename: "Run";
  id: string;
}

export interface GetRun {
  run: GetRun_run | null;
}

export interface GetRunVariables {
  id: string;
}

/* tslint:disable */
/* eslint-disable */
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL query operation: GetUser
// ====================================================

export interface GetUser_user {
  __typename: "User";
  id: string;
  slug: string;
}

export interface GetUser {
  user: GetUser_user | null;
}

export interface GetUserVariables {
  slug: string;
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
