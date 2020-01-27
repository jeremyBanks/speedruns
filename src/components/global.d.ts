declare module "*.scss" {
  export const content: { [className: string]: string };
  export default content;
}

declare module "graphiql" {
  const GraphiQL: React.FC<{
    fetcher: (query: unknown) => Promise<any>;
  }>;
  export default GraphiQL;
}
declare module "graphql-docs" {
  export const GraphQLDocs: React.FC<{
    fetcher: (query: unknown) => Promise<any>;
  }>;
}

declare module "color-space" {
  const space: {
    lab: {
      rgb(lab: [number, number, number]): [number, number, number];
    };
    rgb: {
      lab(rgb: [number, number, number]): [number, number, number];
    };
  };
  export default space;
}
