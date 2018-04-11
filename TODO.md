# To Do

- continue optimizing:
  - reduce number of speedrun api requests
    - since we're displaying all runs for a game, let's get them all in bulk
      and filter them ourselves, instead of making dozens of requests to fetch
      each level/category separately. this requires that we...
    - add pagination support in API.

- import more playthrough runs into speedrun-patches.js:
  - Cire2047 TOD Orc:
    <https://www.youtube.com/playlist?list=PLumTHdkN2x_nwV_rcgadktukTaBZiubYc>
  - Cire2047 TOD Human:
    <https://www.youtube.com/playlist?list=PLC960334B12409C7E>
  - ButcherT BTDP Human:
    <https://www.youtube.com/playlist?list=PLJJzjR7gApQnC1jBpubBvQ1i-nTh-DP3b>
  - ButcherT BTDP Orc:
    <https://www.youtube.com/playlist?list=PLJJzjR7gApQnrV181ney7kYubE1spvLI9>

- decide whether speedrun-patches.js should exist in client or server, then
  move it there properly. right now it's in a least-efficient limbo.

- persistent and timed server-side caching, instead of just random in-memory.
- client-side offline fall-back caching with Service Worker Toolbox.
