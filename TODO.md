# To Do or maybe just ideas

- add support for categories-slugs, maybe like:

      /wc2+wc2btdp-/banks

- add pagination support in API wrapper.  
  currently blocking: https://bests.glitch.me/smb  
  Continue to enforce some sanity limit like 6 pages (1200 items).

- move URL parsing login to a "router" component or something so the server can reuse.

- some amount of feature detection or better error handling, so we don't clobber the server-side render in safari where the client-side render fails.

- can we run our post-render hooks client-side, by walking up the DOM and associating tag names with classes? probably not, since we'd need to also pass in data to reconstruct the model props for the callback to use.

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
    
okay did something:
  - onElementCreated() will be called immediate for event binding on the root element.
  - onRendered() will be called deferred for event binding on any child elements.


maybe come up with a few actual uses before engineering in the abstract.
that's part of the point here: feel out actual use patterns.

- async iterators of props? probably really stupid idea.

- Test assumption that current approach will release the model from memory post-render in common case.
