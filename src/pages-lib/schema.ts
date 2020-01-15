/* tslint:disable */
/* eslint-disable */
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL query operation: GetGamePage
// ====================================================

export interface GetGamePage_game {
  __typename: "Game";
  /**
   * The user's global base64 ID.
   */
  id: string;
  /**
   * The game's URL slug/abbreviation.
   */
  slug: string;
  /**
   * The game's name, in English if possible.
   */
  name: string;
}

export interface GetGamePage {
  /**
   * Get a Game by id or slug, or null if not found.
   *
   * Throws an error if both are specified but don't both match the same game.
   */
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
  /**
   * The run's base36 ID from speedrun.com.
   */
  id: string;
}

export interface GetRun {
  /**
   * Get a Run by id, or null if not found.
   */
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
  /**
   * The users's base36 ID from speedrun.com.
   */
  id: string;
  /**
   * The user's URL slug/abbreviation.
   */
  slug: string;
}

export interface GetUser {
  /**
   * Get a User by id or slug, or null if not found.
   *
   * Throws an error if both are specified but don't both match the same game.
   */
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
