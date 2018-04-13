# To Do

- add pagination support in API wrapper.  
  currently blocking: https://bests.glitch.me/smb  
  Continue to enforce some sanity limit like 6 pages (1200 items).

- refactor everything since it's awful
  hey chris is helping with this!

- update data patcher to live in API classes

- update speedrun-patches.js to create normal speedrun.Run objects, and update
  speedrun.Run to patch them in itself, instead of messy with the API response data.

## Ideas?

- embedded video player?

- user pages?

- events? hard to do that without a dom...
  - it probably wouldn't actually be that hard to allow DOM content to be
    embedded, but it could ruin the immutability. Maybe if we grab the
    outerHTML immediately for the purpose of text output, so you're only
    affected if you're using DOM output? nb: immediately detach for more
    deteriminism.
    this could be reasonable -- components would be the breaking point
    within which rendering occurs.

