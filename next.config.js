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
      "/wc2": { page: "/[game]", query: { game: "wc2" } },
      "/celeste": { page: "/[game]", query: { game: "celeste" } },
      "/wc2btdp": { page: "/[game]", query: { game: "wc2btdp" } },
      "/user/banks": { page: "/user/[user]", query: { user: "banks" } },
      "/user/zpr": { page: "/user/[user]", query: { user: "zpr" } },
      "/user/szwagier": { page: "/user/[user]", query: { user: "szwagier" } }
    };
  }
});
