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
    affected if you're using DOM output? or get both, and check if they change?
    probably awful. nb: immediately detach for more deteriminism.
    parsing/rendering at component boundaries might be reasonable.
    do we have a post-render bind-events step? seems abusable.
    componentDidUpdate()?

maybe come up with a few actual uses before engineering in the abstract.
that's part of the point here: feel out actual use patterns.

