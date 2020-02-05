const path = require("path");

module.exports = [
  require("@zeit/next-css"),
  require("@zeit/next-source-maps"),
  require("@zeit/next-sass"),
].reduce((config, plugin) => plugin(config), {
  cssModules: true,
  exportPathMap() {
    return {
      "/": { page: "/" },
      "/graphql/voyager": { page: "/graphql/voyager" },
      "/wc2/run/yj4xokny": {
        page: "/[game]/run/[runSrcId]",
        query: { game: "wc2", runSrcId: "yj4xokny" },
      },
      "/wc2": { page: "/[game]", query: { game: "wc2" } },
      "/wc2btdp": { page: "/[game]", query: { game: "wc2btdp" } },
      "/node/gamekNdA__8": {
        page: "/node/[node]",
        query: { node: "gamekNdA__8" },
      },
      "/node/cat6f7V-VpE": {
        page: "/node/[node]",
        query: { node: "cat6f7V-VpE" },
      },
      "/node/gamdtuPrEpI": {
        page: "/node/[node]",
        query: { node: "gamdtuPrEpI" },
      },
      "/node/lvlIcodwALE": {
        page: "/node/[node]",
        query: { node: "lvlIcodwALE" },
      },
      "/node/runqi928yNY": {
        page: "/node/[node]",
        query: { node: "runqi928yNY" },
      },
      "/node/usrgcfqgH0Q": {
        page: "/node/[node]",
        query: { node: "usrgcfqgH0Q" },
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
    );
    config.resolve.alias["~"] = path.resolve("./src");
    return config;
  },
});
