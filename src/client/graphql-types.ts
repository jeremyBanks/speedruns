/* tslint:disable */
/* eslint-disable */
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL query operation: GetMyGames
// ====================================================

export interface GetMyGames_war2_runs {
  __typename: "Run";
  /**
   * The run's base36 ID from speedrun.com.
   */
  id: string;
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

export interface GetMyGames_war2btdp_runs {
  __typename: "Run";
  /**
   * The run's base36 ID from speedrun.com.
   */
  id: string;
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
   * Get a game by id or slug.
   */
  war2: GetMyGames_war2;
  /**
   * Get a game by id or slug.
   */
  war2btdp: GetMyGames_war2btdp;
}

/* tslint:disable */
/* eslint-disable */
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL fragment: MyGameDetails
// ====================================================

export interface MyGameDetails_runs {
  __typename: "Run";
  /**
   * The run's base36 ID from speedrun.com.
   */
  id: string;
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
