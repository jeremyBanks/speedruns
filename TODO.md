# To Do

## The Big One

Re-render without reload.

This shouldn't be hard, and our API caching should make this work pretty
efficiently with minimal effort.

## Completionism

import more playthrough runs into speedrun-patches.js:

- Cire2047 TOD Human:
  <https://www.youtube.com/playlist?list=PLC960334B12409C7E>
- ButcherT BTDP Human:
  <https://www.youtube.com/playlist?list=PLJJzjR7gApQnC1jBpubBvQ1i-nTh-DP3b>
- ButcherT BTDP Orc:
  <https://www.youtube.com/playlist?list=PLJJzjR7gApQnrV181ney7kYubE1spvLI9>

## Other Tech

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

- consider client-side offline fall-back caching with Service Worker Toolbox.
  this needs to handle the frequent case of the site being temporarily broken
  because I'm in the middle of editing it. however, consider whether this
  could affect the performance gains we got from disabling the browser's
  default caching for API requests.
  - easy solution: load interface from cache (update in background) every time,
    but always load scripts from server. start with that.
