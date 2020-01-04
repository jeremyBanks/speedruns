/* tslint:disable */
/* eslint-disable */
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL query operation: GetMyGames
// ====================================================

export interface GetMyGames_war2_leaderboard_run_players_user {
  __typename: "User";
  /**
   * The users's base36 ID from speedrun.com.
   */
  id: string;
  /**
   * The user's URL slug/abbreviation.
   */
  slug: string;
}

export interface GetMyGames_war2_leaderboard_run_players {
  __typename: "Player";
  /**
   * The player's name, which may be a distinct username or a non-distinct guest
   * nickname.
   */
  name: string;
  /**
   * Whether this player is a guest instead of a user.
   */
  isGuest: boolean;
  /**
   * The associated user, if this is a user.
   */
  user: GetMyGames_war2_leaderboard_run_players_user | null;
}

export interface GetMyGames_war2_leaderboard_run {
  __typename: "Run";
  /**
   * The run's base36 ID from speedrun.com.
   */
  id: string;
  date: number | null;
  players: GetMyGames_war2_leaderboard_run_players[];
}

export interface GetMyGames_war2_leaderboard {
  __typename: "RankedRun";
  /**
   * This run's rank, with ties broken by date.
   */
  rank: number;
  /**
   * This run's rank, with ties unbroken.
   */
  tiedRank: number;
  /**
   * Whether this run is tied for this rank.
   */
  isTied: boolean;
  /**
   * The time of this run, as measured by this leaderboard's rules, in miliseconds.
   */
  timeMs: number;
  /**
   * The run.
   */
  run: GetMyGames_war2_leaderboard_run;
}

export interface GetMyGames_war2_levels_leaderboard_run_players_user {
  __typename: "User";
  /**
   * The users's base36 ID from speedrun.com.
   */
  id: string;
  /**
   * The user's URL slug/abbreviation.
   */
  slug: string;
}

export interface GetMyGames_war2_levels_leaderboard_run_players {
  __typename: "Player";
  /**
   * The player's name, which may be a distinct username or a non-distinct guest
   * nickname.
   */
  name: string;
  /**
   * Whether this player is a guest instead of a user.
   */
  isGuest: boolean;
  /**
   * The associated user, if this is a user.
   */
  user: GetMyGames_war2_levels_leaderboard_run_players_user | null;
}

export interface GetMyGames_war2_levels_leaderboard_run {
  __typename: "Run";
  /**
   * The run's base36 ID from speedrun.com.
   */
  id: string;
  date: number | null;
  players: GetMyGames_war2_levels_leaderboard_run_players[];
}

export interface GetMyGames_war2_levels_leaderboard {
  __typename: "RankedRun";
  /**
   * This run's rank, with ties broken by date.
   */
  rank: number;
  /**
   * This run's rank, with ties unbroken.
   */
  tiedRank: number;
  /**
   * Whether this run is tied for this rank.
   */
  isTied: boolean;
  /**
   * The time of this run, as measured by this leaderboard's rules, in miliseconds.
   */
  timeMs: number;
  /**
   * The run.
   */
  run: GetMyGames_war2_levels_leaderboard_run;
}

export interface GetMyGames_war2_levels {
  __typename: "Level";
  /**
   * The level's base36 ID from speedrun.com.
   */
  id: string;
  /**
   * The level's slug.
   */
  slug: string;
  /**
   * The level's name.
   */
  name: string;
  /**
   * Returns ordered ranked runs.
   */
  leaderboard: GetMyGames_war2_levels_leaderboard[];
}

export interface GetMyGames_war2 {
  __typename: "Game";
  /**
   * The game's base36 ID from speedrun.com.
   */
  id: string;
  /**
   * The game's name, in English if possible.
   */
  name: string;
  /**
   * The game's URL slug/abbreviation.
   */
  slug: string;
  /**
   * Returns the ordered ranked runs for a run in a category and optionally level.
   */
  leaderboard: GetMyGames_war2_leaderboard[];
  levels: GetMyGames_war2_levels[];
}

export interface GetMyGames_war2x_leaderboard_run_players_user {
  __typename: "User";
  /**
   * The users's base36 ID from speedrun.com.
   */
  id: string;
  /**
   * The user's URL slug/abbreviation.
   */
  slug: string;
}

export interface GetMyGames_war2x_leaderboard_run_players {
  __typename: "Player";
  /**
   * The player's name, which may be a distinct username or a non-distinct guest
   * nickname.
   */
  name: string;
  /**
   * Whether this player is a guest instead of a user.
   */
  isGuest: boolean;
  /**
   * The associated user, if this is a user.
   */
  user: GetMyGames_war2x_leaderboard_run_players_user | null;
}

export interface GetMyGames_war2x_leaderboard_run {
  __typename: "Run";
  /**
   * The run's base36 ID from speedrun.com.
   */
  id: string;
  date: number | null;
  players: GetMyGames_war2x_leaderboard_run_players[];
}

export interface GetMyGames_war2x_leaderboard {
  __typename: "RankedRun";
  /**
   * This run's rank, with ties broken by date.
   */
  rank: number;
  /**
   * This run's rank, with ties unbroken.
   */
  tiedRank: number;
  /**
   * Whether this run is tied for this rank.
   */
  isTied: boolean;
  /**
   * The time of this run, as measured by this leaderboard's rules, in miliseconds.
   */
  timeMs: number;
  /**
   * The run.
   */
  run: GetMyGames_war2x_leaderboard_run;
}

export interface GetMyGames_war2x_levels_leaderboard_run_players_user {
  __typename: "User";
  /**
   * The users's base36 ID from speedrun.com.
   */
  id: string;
  /**
   * The user's URL slug/abbreviation.
   */
  slug: string;
}

export interface GetMyGames_war2x_levels_leaderboard_run_players {
  __typename: "Player";
  /**
   * The player's name, which may be a distinct username or a non-distinct guest
   * nickname.
   */
  name: string;
  /**
   * Whether this player is a guest instead of a user.
   */
  isGuest: boolean;
  /**
   * The associated user, if this is a user.
   */
  user: GetMyGames_war2x_levels_leaderboard_run_players_user | null;
}

export interface GetMyGames_war2x_levels_leaderboard_run {
  __typename: "Run";
  /**
   * The run's base36 ID from speedrun.com.
   */
  id: string;
  date: number | null;
  players: GetMyGames_war2x_levels_leaderboard_run_players[];
}

export interface GetMyGames_war2x_levels_leaderboard {
  __typename: "RankedRun";
  /**
   * This run's rank, with ties broken by date.
   */
  rank: number;
  /**
   * This run's rank, with ties unbroken.
   */
  tiedRank: number;
  /**
   * Whether this run is tied for this rank.
   */
  isTied: boolean;
  /**
   * The time of this run, as measured by this leaderboard's rules, in miliseconds.
   */
  timeMs: number;
  /**
   * The run.
   */
  run: GetMyGames_war2x_levels_leaderboard_run;
}

export interface GetMyGames_war2x_levels {
  __typename: "Level";
  /**
   * The level's base36 ID from speedrun.com.
   */
  id: string;
  /**
   * The level's slug.
   */
  slug: string;
  /**
   * The level's name.
   */
  name: string;
  /**
   * Returns ordered ranked runs.
   */
  leaderboard: GetMyGames_war2x_levels_leaderboard[];
}

export interface GetMyGames_war2x {
  __typename: "Game";
  /**
   * The game's base36 ID from speedrun.com.
   */
  id: string;
  /**
   * The game's name, in English if possible.
   */
  name: string;
  /**
   * The game's URL slug/abbreviation.
   */
  slug: string;
  /**
   * Returns the ordered ranked runs for a run in a category and optionally level.
   */
  leaderboard: GetMyGames_war2x_leaderboard[];
  levels: GetMyGames_war2x_levels[];
}

export interface GetMyGames {
  /**
   * Get a game.
   */
  war2: GetMyGames_war2;
  /**
   * Get a game.
   */
  war2x: GetMyGames_war2x;
}

/* tslint:disable */
/* eslint-disable */
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL fragment: MyRankedRun
// ====================================================

export interface MyRankedRun_run_players_user {
  __typename: "User";
  /**
   * The users's base36 ID from speedrun.com.
   */
  id: string;
  /**
   * The user's URL slug/abbreviation.
   */
  slug: string;
}

export interface MyRankedRun_run_players {
  __typename: "Player";
  /**
   * The player's name, which may be a distinct username or a non-distinct guest
   * nickname.
   */
  name: string;
  /**
   * Whether this player is a guest instead of a user.
   */
  isGuest: boolean;
  /**
   * The associated user, if this is a user.
   */
  user: MyRankedRun_run_players_user | null;
}

export interface MyRankedRun_run {
  __typename: "Run";
  /**
   * The run's base36 ID from speedrun.com.
   */
  id: string;
  date: number | null;
  players: MyRankedRun_run_players[];
}

export interface MyRankedRun {
  __typename: "RankedRun";
  /**
   * This run's rank, with ties broken by date.
   */
  rank: number;
  /**
   * This run's rank, with ties unbroken.
   */
  tiedRank: number;
  /**
   * Whether this run is tied for this rank.
   */
  isTied: boolean;
  /**
   * The time of this run, as measured by this leaderboard's rules, in miliseconds.
   */
  timeMs: number;
  /**
   * The run.
   */
  run: MyRankedRun_run;
}

/* tslint:disable */
/* eslint-disable */
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL fragment: MyGameDetails
// ====================================================

export interface MyGameDetails_leaderboard_run_players_user {
  __typename: "User";
  /**
   * The users's base36 ID from speedrun.com.
   */
  id: string;
  /**
   * The user's URL slug/abbreviation.
   */
  slug: string;
}

export interface MyGameDetails_leaderboard_run_players {
  __typename: "Player";
  /**
   * The player's name, which may be a distinct username or a non-distinct guest
   * nickname.
   */
  name: string;
  /**
   * Whether this player is a guest instead of a user.
   */
  isGuest: boolean;
  /**
   * The associated user, if this is a user.
   */
  user: MyGameDetails_leaderboard_run_players_user | null;
}

export interface MyGameDetails_leaderboard_run {
  __typename: "Run";
  /**
   * The run's base36 ID from speedrun.com.
   */
  id: string;
  date: number | null;
  players: MyGameDetails_leaderboard_run_players[];
}

export interface MyGameDetails_leaderboard {
  __typename: "RankedRun";
  /**
   * This run's rank, with ties broken by date.
   */
  rank: number;
  /**
   * This run's rank, with ties unbroken.
   */
  tiedRank: number;
  /**
   * Whether this run is tied for this rank.
   */
  isTied: boolean;
  /**
   * The time of this run, as measured by this leaderboard's rules, in miliseconds.
   */
  timeMs: number;
  /**
   * The run.
   */
  run: MyGameDetails_leaderboard_run;
}

export interface MyGameDetails_levels_leaderboard_run_players_user {
  __typename: "User";
  /**
   * The users's base36 ID from speedrun.com.
   */
  id: string;
  /**
   * The user's URL slug/abbreviation.
   */
  slug: string;
}

export interface MyGameDetails_levels_leaderboard_run_players {
  __typename: "Player";
  /**
   * The player's name, which may be a distinct username or a non-distinct guest
   * nickname.
   */
  name: string;
  /**
   * Whether this player is a guest instead of a user.
   */
  isGuest: boolean;
  /**
   * The associated user, if this is a user.
   */
  user: MyGameDetails_levels_leaderboard_run_players_user | null;
}

export interface MyGameDetails_levels_leaderboard_run {
  __typename: "Run";
  /**
   * The run's base36 ID from speedrun.com.
   */
  id: string;
  date: number | null;
  players: MyGameDetails_levels_leaderboard_run_players[];
}

export interface MyGameDetails_levels_leaderboard {
  __typename: "RankedRun";
  /**
   * This run's rank, with ties broken by date.
   */
  rank: number;
  /**
   * This run's rank, with ties unbroken.
   */
  tiedRank: number;
  /**
   * Whether this run is tied for this rank.
   */
  isTied: boolean;
  /**
   * The time of this run, as measured by this leaderboard's rules, in miliseconds.
   */
  timeMs: number;
  /**
   * The run.
   */
  run: MyGameDetails_levels_leaderboard_run;
}

export interface MyGameDetails_levels {
  __typename: "Level";
  /**
   * The level's base36 ID from speedrun.com.
   */
  id: string;
  /**
   * The level's slug.
   */
  slug: string;
  /**
   * The level's name.
   */
  name: string;
  /**
   * Returns ordered ranked runs.
   */
  leaderboard: MyGameDetails_levels_leaderboard[];
}

export interface MyGameDetails {
  __typename: "Game";
  /**
   * The game's base36 ID from speedrun.com.
   */
  id: string;
  /**
   * The game's name, in English if possible.
   */
  name: string;
  /**
   * The game's URL slug/abbreviation.
   */
  slug: string;
  /**
   * Returns the ordered ranked runs for a run in a category and optionally level.
   */
  leaderboard: MyGameDetails_leaderboard[];
  levels: MyGameDetails_levels[];
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
