module.exports = {
  client: {
    includes: ["src/**/*.ts", "src/**/*.tsx"],
    service: {
      name: "local",
      url: "http://localhost:3001/graphql",
    },
  },
};
