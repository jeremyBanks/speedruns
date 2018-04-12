# To Do

- continue optimizing:
  - reduce number of speedrun api requests
    - since we're displaying all runs for a game, let's get them all in bulk
      and filter them ourselves, instead of making dozens of requests to fetch
      each level/category separately. this should also reduce total bandwidth
      by allowing better compression. this requires that we...
    - add pagination support in API.

- decide whether speedrun-patches.js should exist in client or server, then
  move it there properly. right now it's in a least-efficient limbo.

- persistent and timed server-side caching, instead of just random in-memory.
  - slightly less urgent once we're not hammering with excessive requests.
