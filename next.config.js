module.exports = [
  require("@zeit/next-css"),
  require("@zeit/next-source-maps"),
  require("@zeit/next-sass")
].reduce((config, plugin) => plugin(config), {
  cssModules: true,
  exportTrailingSlash: true,
  exportPathMap: function() {
    return {
      "/": { page: "/" },
      "/wc2": { page: "/[game]/", query: { game: "wc2" } },
      "/wc2btdp": { page: "/[game]/", query: { game: "wc2btdp" } }
    };
  }
});
