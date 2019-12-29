/* tslint:disable */
/* eslint-disable */
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL query operation: TypeNames
// ====================================================

export interface TypeNames___schema_types {
  __typename: "__Type";
  name: string | null;
}

export interface TypeNames___schema {
  __typename: "__Schema";
  /**
   * A list of all types supported by this server.
   */
  types: TypeNames___schema_types[];
}

export interface TypeNames {
  __schema: TypeNames___schema;
}
