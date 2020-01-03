/* tslint:disable */
/* eslint-disable */
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL query operation: GetMyGames
// ====================================================

export interface GetMyGames_war2_leaderboard_run {
  __typename: "Run";
  /**
   * The run's base36 ID from speedrun.com.
   */
  id: string;
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

export interface GetMyGames_war2_runs_category {
  __typename: "Category";
  /**
   * The category's base36 ID from speedrun.com.
   */
  id: string;
  /**
   * The category's slug.
   */
  slug: string;
  /**
   * The category's name.
   */
  name: string;
}

export interface GetMyGames_war2_runs {
  __typename: "Run";
  /**
   * The run's base36 ID from speedrun.com.
   */
  id: string;
  /**
   * The category associated with this run.
   */
  category: GetMyGames_war2_runs_category;
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
  /**
   * All of the runs submitted for this game.
   */
  runs: GetMyGames_war2_runs[];
}

export interface GetMyGames_war2x_leaderboard_run {
  __typename: "Run";
  /**
   * The run's base36 ID from speedrun.com.
   */
  id: string;
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

export interface GetMyGames_war2x_runs_category {
  __typename: "Category";
  /**
   * The category's base36 ID from speedrun.com.
   */
  id: string;
  /**
   * The category's slug.
   */
  slug: string;
  /**
   * The category's name.
   */
  name: string;
}

export interface GetMyGames_war2x_runs {
  __typename: "Run";
  /**
   * The run's base36 ID from speedrun.com.
   */
  id: string;
  /**
   * The category associated with this run.
   */
  category: GetMyGames_war2x_runs_category;
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
  /**
   * All of the runs submitted for this game.
   */
  runs: GetMyGames_war2x_runs[];
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

export interface MyRankedRun_run {
  __typename: "Run";
  /**
   * The run's base36 ID from speedrun.com.
   */
  id: string;
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

export interface MyGameDetails_leaderboard_run {
  __typename: "Run";
  /**
   * The run's base36 ID from speedrun.com.
   */
  id: string;
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

export interface MyGameDetails_runs_category {
  __typename: "Category";
  /**
   * The category's base36 ID from speedrun.com.
   */
  id: string;
  /**
   * The category's slug.
   */
  slug: string;
  /**
   * The category's name.
   */
  name: string;
}

export interface MyGameDetails_runs {
  __typename: "Run";
  /**
   * The run's base36 ID from speedrun.com.
   */
  id: string;
  /**
   * The category associated with this run.
   */
  category: MyGameDetails_runs_category;
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
  /**
   * All of the runs submitted for this game.
   */
  runs: MyGameDetails_runs[];
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
