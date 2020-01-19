/* tslint:disable */
/* eslint-disable */
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL query operation: GetGamePage
// ====================================================

export interface GetGamePage_game_categories_leaderboard_run_category {
  __typename: "Category";
  id: string;
  srcId: string;
}

export interface GetGamePage_game_categories_leaderboard_run_level {
  __typename: "Level";
  id: string;
  srcId: string;
}

export interface GetGamePage_game_categories_leaderboard_run_players_user {
  __typename: "User";
  id: string;
  srcId: string;
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
  srcId: string;
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
  srcId: string;
  name: string;
  leaderboard: GetGamePage_game_categories_leaderboard[];
}

export interface GetGamePage_game_levels_leaderboard_run_category {
  __typename: "Category";
  id: string;
  srcId: string;
}

export interface GetGamePage_game_levels_leaderboard_run_level {
  __typename: "Level";
  id: string;
  srcId: string;
}

export interface GetGamePage_game_levels_leaderboard_run_players_user {
  __typename: "User";
  id: string;
  srcId: string;
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
  id: string;
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
  id: string;
  srcId: string;
  name: string;
  leaderboard: GetGamePage_game_levels_leaderboard[];
}

export interface GetGamePage_game {
  __typename: "Game";
  id: string;
  srcId: string;
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
// GraphQL fragment: GameLeaderboardRun
// ====================================================

export interface GameLeaderboardRun_run_category {
  __typename: "Category";
  id: string;
  srcId: string;
}

export interface GameLeaderboardRun_run_level {
  __typename: "Level";
  id: string;
  srcId: string;
}

export interface GameLeaderboardRun_run_players_user {
  __typename: "User";
  id: string;
  srcId: string;
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
  id: string;
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
