module.exports = [
  require("@zeit/next-css"),
  require("@zeit/next-source-maps"),
  require("@zeit/next-sass"),
].reduce((config, plugin) => plugin(config), {
  cssModules: true,
  exportTrailingSlash: false,
  exportPathMap() {
    return {
      "/": { page: "/" },
      "/graphql/voyager": { page: "/graphql/voyager" },
      "/wc2": { page: "/[game]", query: { game: "wc2" } },
      "/wc2btdp": { page: "/[game]", query: { game: "wc2btdp" } },
      "/node/cat00000000": {
        page: "/node/[node]",
        query: { node: "cat00000000" },
      },
      "/node/catCUheVH-k": {
        page: "/node/[node]",
        query: { node: "catCUheVH-k" },
      },
      "/node/gamBtuPrEpI": {
        page: "/node/[node]",
        query: { node: "gamBtuPrEpI" },
      },
      "/node/lvlBhjzlf1k": {
        page: "/node/[node]",
        query: { node: "lvlBhjzlf1k" },
      },
      "/node/runBk0OGMPY": {
        page: "/node/[node]",
        query: { node: "runBk0OGMPY" },
      },
      "/node/usrAFq39djE": {
        page: "/node/[node]",
        query: { node: "usrAFq39djE" },
      },
    };
  },
  webpack(config) {
    config.module.rules.push(
      {
        test: /\.(flow)$/,
        use: ["file-loader"],
      },
      {
        test: /\.(png|svg|jpg|gif|jpeg)$/,
        use: ["file-loader"],
      },
      {
        test: /\.toml$/,
        use: ["@lcdev/toml-loader"],
      },
    );
    return config;
  },
});
