#!/usr/bin/env node
"use strict";

const { createServer } = require("http");
const { parse } = require("url");
const path = require("path");

process.chdir(path.dirname(path.dirname(path.dirname(__filename))));

const next = require("next");
const argv = require("yargs").argv;

const port = Number(argv.port || 3000);

const dev = process.env.NODE_ENV !== "production";
const app = next({ dev });
const handle = app.getRequestHandler();

app.prepare().then(() => {
  createServer((req, res) => {
    const parsedUrl = parse(req.url, true);
    handle(req, res, parsedUrl);
  }).listen(port, err => {
    if (err) {
      throw err;
    }
    // tslint:disable-next-line:no-console
    console.log(`http://localhost:${port}`);
  });
});
