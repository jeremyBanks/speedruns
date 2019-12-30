/* tslint:disable */
/* eslint-disable */
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL query operation: GetGameBySlug
// ====================================================

export interface GetGameBySlug_game {
  __typename: "Game";
  id: string;
  name: string;
  slug: string;
}

export interface GetGameBySlug {
  /**
   * Get a game by id or slug.
   */
  game: GetGameBySlug_game;
}

export interface GetGameBySlugVariables {
  slug?: string | null;
}
