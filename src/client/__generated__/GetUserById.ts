/* tslint:disable */
/* eslint-disable */
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL query operation: GetUserById
// ====================================================

export interface GetUserById_user {
  __typename: "User";
  id: string;
  name: string;
}

export interface GetUserById {
  /**
   * Get a user by id or slug. Throws an error if none are specified,
   * or no user matches all that are specified.
   */
  user: GetUserById_user;
}

export interface GetUserByIdVariables {
  id?: string | null;
}
