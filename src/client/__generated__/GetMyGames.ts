/* tslint:disable */
/* eslint-disable */
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL query operation: GetMyGames
// ====================================================

export interface GetMyGames_war2 {
  __typename: "Game";
  id: string;
  name: string;
  slug: string;
}

export interface GetMyGames_war2x {
  __typename: "Game";
  id: string;
  name: string;
  slug: string;
}

export interface GetMyGames {
  /**
   * Get a game by id or slug.
   */
  war2: GetMyGames_war2;
  /**
   * Get a game by id or slug.
   */
  war2x: GetMyGames_war2x;
}
