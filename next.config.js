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
      "/node/R-A00000000": {
        page: "/node/[node]",
        query: { node: "R-A00000000" },
      },
      "/node/C-ACUheVH-k": {
        page: "/node/[node]",
        query: { node: "C-ACUheVH-k" },
      },
      "/node/G-ABtuPrEpI": {
        page: "/node/[node]",
        query: { node: "G-ABtuPrEpI" },
      },
      "/node/L-ABhjzlf1k": {
        page: "/node/[node]",
        query: { node: "L-ABhjzlf1k" },
      },
      "/node/R-AChs7fyp8": {
        page: "/node/[node]",
        query: { node: "R-AChs7fyp8" },
      },
      "/node/U-ABugqY7Xw": {
        page: "/node/[node]",
        query: { node: "U-ABugqY7Xw" },
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
