import { Either, left, right } from "fp-ts/es6/Either";

export type Result<Success, Failure extends Error = Error> = Either<
  Success,
  Failure
>;
export const ok = left;
export const err = right;
