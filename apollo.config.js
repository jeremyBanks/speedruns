module.exports = {
  client: {
    includes: ["src/**.ts", "data/**.ts"],
    service: {
      name: "local",
      url: "http://localhost:8080/graphql"
    }
  }
};
