/* tslint:disable */
/* eslint-disable */
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL query operation: GetHome
// ====================================================

export interface GetHome_war2_leaderboard_run_players_user {
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

export interface GetHome_war2_leaderboard_run_players {
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
  user: GetHome_war2_leaderboard_run_players_user | null;
}

export interface GetHome_war2_leaderboard_run {
  __typename: "Run";
  /**
   * The run's base36 ID from speedrun.com.
   */
  id: string;
  date: number | null;
  players: GetHome_war2_leaderboard_run_players[];
}

export interface GetHome_war2_leaderboard {
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
  run: GetHome_war2_leaderboard_run;
}

export interface GetHome_war2_levels_leaderboard_run_players_user {
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

export interface GetHome_war2_levels_leaderboard_run_players {
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
  user: GetHome_war2_levels_leaderboard_run_players_user | null;
}

export interface GetHome_war2_levels_leaderboard_run {
  __typename: "Run";
  /**
   * The run's base36 ID from speedrun.com.
   */
  id: string;
  date: number | null;
  players: GetHome_war2_levels_leaderboard_run_players[];
}

export interface GetHome_war2_levels_leaderboard {
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
  run: GetHome_war2_levels_leaderboard_run;
}

export interface GetHome_war2_levels {
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
  leaderboard: GetHome_war2_levels_leaderboard[];
}

export interface GetHome_war2 {
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
  leaderboard: GetHome_war2_leaderboard[];
  levels: GetHome_war2_levels[];
}

export interface GetHome_war2x_leaderboard_run_players_user {
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

export interface GetHome_war2x_leaderboard_run_players {
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
  user: GetHome_war2x_leaderboard_run_players_user | null;
}

export interface GetHome_war2x_leaderboard_run {
  __typename: "Run";
  /**
   * The run's base36 ID from speedrun.com.
   */
  id: string;
  date: number | null;
  players: GetHome_war2x_leaderboard_run_players[];
}

export interface GetHome_war2x_leaderboard {
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
  run: GetHome_war2x_leaderboard_run;
}

export interface GetHome_war2x_levels_leaderboard_run_players_user {
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

export interface GetHome_war2x_levels_leaderboard_run_players {
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
  user: GetHome_war2x_levels_leaderboard_run_players_user | null;
}

export interface GetHome_war2x_levels_leaderboard_run {
  __typename: "Run";
  /**
   * The run's base36 ID from speedrun.com.
   */
  id: string;
  date: number | null;
  players: GetHome_war2x_levels_leaderboard_run_players[];
}

export interface GetHome_war2x_levels_leaderboard {
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
  run: GetHome_war2x_levels_leaderboard_run;
}

export interface GetHome_war2x_levels {
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
  leaderboard: GetHome_war2x_levels_leaderboard[];
}

export interface GetHome_war2x {
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
  leaderboard: GetHome_war2x_leaderboard[];
  levels: GetHome_war2x_levels[];
}

export interface GetHome {
  /**
   * Get a game.
   */
  war2: GetHome_war2;
  /**
   * Get a game.
   */
  war2x: GetHome_war2x;
}

/* tslint:disable */
/* eslint-disable */
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL fragment: HomeRankedRun
// ====================================================

export interface HomeRankedRun_run_players_user {
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

export interface HomeRankedRun_run_players {
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
  user: HomeRankedRun_run_players_user | null;
}

export interface HomeRankedRun_run {
  __typename: "Run";
  /**
   * The run's base36 ID from speedrun.com.
   */
  id: string;
  date: number | null;
  players: HomeRankedRun_run_players[];
}

export interface HomeRankedRun {
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
  run: HomeRankedRun_run;
}

/* tslint:disable */
/* eslint-disable */
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL fragment: HomeDetails
// ====================================================

export interface HomeDetails_leaderboard_run_players_user {
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

export interface HomeDetails_leaderboard_run_players {
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
  user: HomeDetails_leaderboard_run_players_user | null;
}

export interface HomeDetails_leaderboard_run {
  __typename: "Run";
  /**
   * The run's base36 ID from speedrun.com.
   */
  id: string;
  date: number | null;
  players: HomeDetails_leaderboard_run_players[];
}

export interface HomeDetails_leaderboard {
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
  run: HomeDetails_leaderboard_run;
}

export interface HomeDetails_levels_leaderboard_run_players_user {
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

export interface HomeDetails_levels_leaderboard_run_players {
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
  user: HomeDetails_levels_leaderboard_run_players_user | null;
}

export interface HomeDetails_levels_leaderboard_run {
  __typename: "Run";
  /**
   * The run's base36 ID from speedrun.com.
   */
  id: string;
  date: number | null;
  players: HomeDetails_levels_leaderboard_run_players[];
}

export interface HomeDetails_levels_leaderboard {
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
  run: HomeDetails_levels_leaderboard_run;
}

export interface HomeDetails_levels {
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
  leaderboard: HomeDetails_levels_leaderboard[];
}

export interface HomeDetails {
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
  leaderboard: HomeDetails_leaderboard[];
  levels: HomeDetails_levels[];
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
