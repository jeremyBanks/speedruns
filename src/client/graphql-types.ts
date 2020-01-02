/* tslint:disable */
/* eslint-disable */
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL query operation: GetMyGames
// ====================================================

export interface GetMyGames_banks {
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

export interface GetMyGames_war2_runs_level {
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
  /**
   * The level associated with this run, or null.
   */
  level: GetMyGames_war2_runs_level | null;
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
   * All of the runs submitted for this game.
   */
  runs: GetMyGames_war2_runs[];
}

export interface GetMyGames_war2btdp_runs_category {
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

export interface GetMyGames_war2btdp_runs_level {
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
}

export interface GetMyGames_war2btdp_runs {
  __typename: "Run";
  /**
   * The run's base36 ID from speedrun.com.
   */
  id: string;
  /**
   * The category associated with this run.
   */
  category: GetMyGames_war2btdp_runs_category;
  /**
   * The level associated with this run, or null.
   */
  level: GetMyGames_war2btdp_runs_level | null;
}

export interface GetMyGames_war2btdp {
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
   * All of the runs submitted for this game.
   */
  runs: GetMyGames_war2btdp_runs[];
}

export interface GetMyGames {
  /**
   * Get a user.
   */
  banks: GetMyGames_banks;
  /**
   * Get a game.
   */
  war2: GetMyGames_war2;
  /**
   * Get a game.
   */
  war2btdp: GetMyGames_war2btdp;
}

/* tslint:disable */
/* eslint-disable */
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL fragment: MyGameDetails
// ====================================================

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

export interface MyGameDetails_runs_level {
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
  /**
   * The level associated with this run, or null.
   */
  level: MyGameDetails_runs_level | null;
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
