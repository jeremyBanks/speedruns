# To Do

- add pagination support in API wrapper.

- fetch levels in parallel with level info, since it doesn't depend on it

- refactor everything since it's awful

- update data patcher 

- decide whether speedrun-patches.js should exist in client or server, then
  move it there properly. right now it's in a least-efficient limbo.

- persistent and timed server-side caching, instead of just random in-memory.
  - slightly less urgent once we're not hammering with excessive requests.
