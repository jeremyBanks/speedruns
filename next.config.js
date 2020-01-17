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
      "/user/zpr": { page: "/user/[user]", query: { user: "zpr" } },
      "/user/banks": { page: "/user/[user]", query: { user: "banks" } },
      "/user/szwagier": { page: "/user/[user]", query: { user: "szwagier" } },
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
