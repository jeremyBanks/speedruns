module.exports = {
  client: {
    includes: ["src/**/*.ts", "src/**/*.tsx"],
    service: {
      name: "local",
      url: "http://localhost:8080/graphql"
    }
  }
};
