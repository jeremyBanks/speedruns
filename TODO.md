# 👌 to do

- limit API requests to exclude levels we don't need?

  - hmm, it may only be possible to limit requests to a single level/category, 
    not a list of them!
  
    maybe it's still worth requesting them individually, when we're not going
    to be using all of them (when we have some level filter specified).
    
    make sure they're rendered incrementally.

- some way to override real/in-game time settings
  (such as for Overwatch IL, which are IGT per rules but RTT per API, with inconsistent data)
  
- add embedded dom object support in HTML templating.

  - maybe assert that provided dom elements aren't attached the dom already,
    before moving them.

- (requires embedded dom support) construct nested components as elements instead of strings when on
  the client, so that their lifecycle methods can be fired appropraitely, and do so.

# 🤔 to consider
  
- (required nested components lifecycle methods) add embedded video player.

- refactor graph code, as it is currently very dense, hacky, and difficult to understand.

- move link handling/client-side-reload into the "router" component.

- be preactive and copy the general API?

  - add state and props, with appropriate methods, and maybe rename lifecycle callbacks.

- make client and server-side error handling consistent, possibly using a new common component.

# 🧠 brain train

- async iterators of props? not unless you have a motivating use case.
