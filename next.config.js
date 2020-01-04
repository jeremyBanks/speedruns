module.exports = require("@zeit/next-css")(
  require("@zeit/next-source-maps")(
    require("@zeit/next-sass")({
      cssModules: true
    })
  )
);
