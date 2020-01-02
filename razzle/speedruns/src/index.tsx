import http from "http";

let app = require("./server").default;

const server = http.createServer(app);

server.listen(process.env.PORT || 3000, () => {
  console.log("🚀 started");
});
