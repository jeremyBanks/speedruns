import { useQuery } from "@apollo/react-hooks";
import { DocumentNode } from "graphql";

/// Wraps useQuery to always return the data if any is known (even if there are
/// partial errors), or else return a loading/error status component.
const useQueryWithStatus = <TData extends any = unknown>(
  query: DocumentNode,
  variables: { [key: string]: any } = {}
): { data: TData | undefined; status: JSX.Element } => {
  const { data, loading, error } = useQuery<TData>(query, { variables });

  return {
    data,
    status: <div>loading: {JSON.stringify(loading || error, null, 2)}</div>
  };
};

export default useQueryWithStatus;
