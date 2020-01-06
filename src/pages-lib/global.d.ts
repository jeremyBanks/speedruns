declare module "*.scss" {
  export const content: { [className: string]: string };
  export default content;
}

declare module "graphiql" {
  export const GraphiQL: React.FC<{
    fetcher: (query: unknown) => Promise<any>;
  }>;
  export default GraphiQL;
}
