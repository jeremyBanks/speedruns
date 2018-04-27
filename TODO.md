# to do

- refactor graph code, as it is currently very dense, hacky, and difficult to understand.

- add support for category/level paths (also forcing change of user paths), maybe like this:

      /smwext/world_1+world_2/@banks+@zpr
      
  where level/category slugs either be their slugs or their ids

- move link handling/client-side-reload into the "router" component.

- user pages?

      /@Banks
  
  Showing their personal bests in every level/category they've run.
  
  The API gives us the list of levels and categories:
  <https://www.speedrun.com/api/v1/users/Banks/personal-bests>


# brain storming

- reconsider top-level error handling -- since we now have a static render to fall back to (unless it has an error?)

- can we run our post-render hooks client-side, by walking up the DOM and associating tag names with classes? probably not, since we'd need to also pass in data to reconstruct the model props for the callback to use.

- embedded video player?

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
