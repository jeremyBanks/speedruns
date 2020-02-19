# A Simplified Async Web Service in Rust

As an exercise in understanding Rust async capabilities and concurrency
semantics I working through a simplified design of a web service with an
integrated database.

### Dependencies

For an async runtime and utilities I'll use `async_std` with the `attributes`
feature enabled so I can have an `async fn main`. I'll also include a few other
common crates I might need later.

```toml
[dependencies]
async-std = {  features = ["attributes"], version = "1.5.0" }
rand = "0.7.3"
```

```rust
use async_std::{prelude::*, task::sleep};
use rand::prelude::*;

#[async_std::main]
async fn main() {
    unimplemented!()
}
```

### Dummy Requests

I need types to represent the Requests and Responses to the server. I'll leave
them empty for now.

```rust
#[derive(Debug)]
struct Request {}

#[derive(Debug)]
struct Response {}
```

### The App

I'll define another initially-empty struct for the app itself. I need it to be
able to asynchronously produce responses to incoming requests, so I'll define
such a `handle()` method on `&self` that returns one of our empty Response
objects for now.

```rust
#[derive(Debug)]
struct WebApp {}

impl WebApp {
    fn new() -> WebApp {
        WebApp {}
    }

    async fn handle(&self,  request: Request) -> Response {
        Response {}
    }
}
```

### Event Loop with Fake Requests

Let's start defining the main event loop, to simulate incoming requests.

```rust
#[async_std::main]
async fn main() {
    let app = WebApp::new();

    loop {
        // For some variety, let's say that every one second...
        sleep(Duration::from_secs(1)).await;
        // ...I have a 40% probability...
        if 0.40 > rand::thread_rng().gen_range(0.0, 1.0) {
            // ...of simulating a request.
            let request = Request {};
            println!(" request: {:?}", request);

            // I ask the app to handle it and wait for the response.
            let response = app.handle(request).await;
            println!("response: {:?}", response);
        }
    }
}
```

This approach works, but it's not concurrent or really asynchronous: each time I
get a request, the event loop is blocked until I have a response to send. This
might be fine if our responses are generated almost instantly, but if I insert a
delay in the response handler to simulate a connection to a backend service, I
notice that the number of requests I can handle drops tremendously.

```
[t=   1.0s]  request: Request {}
[t=   3.5s] response: Response {}
[t=   4.5s]  request: Request {}
[t=   7.0s] response: Response {}
[t=   8.0s]  request: Request {}
[t=  10.5s] response: Response {}
[t=  11.5s]  request: Request {}
[t=  14.0s] response: Response {}
```

```rust
    async fn handle(&self,  request: Request) -> Response {
        // Pretend I'm waiting for a slow backend.
        sleep(Duration::from_secs(5)).await;
        Response {}
    }
```

If you come from JavaScript you might think "just get rid of the await, so the
request handler runs in the background and doesn't block your event loop".
However, in the case of Rust there isn't a single, default event loop in the
background that will automatically execute your async functions. Your code needs
to make sure the async function (or rather, the `impl Future` value it returns)
is eventually passed to an "executor" that is responsible for continuing to
execute it. (The compiler will usually warn you if you forget to, so this isn't
as much of a foot-gun as it might sound.)

Our async `main()` function is running on an executor provided by the
`async_std` library, which I've invoked with the `#[async_std::main]` macro.
This executor is capable of running code in parallel, but in the code above I
`.await` the `handle()` method which causes the current executor to run the
handler to completion before continuing to run our `main` loop, so we don't
actually have any paralellism.

I want it the handler to run on its own, without blocking our `main()` loop from
reading new incoming connections. The simplest way to do this with `async_std`
is to use its tasks feature. Tasks are a thread-like abstraction for independent
pieces of work, which `async_std` may schedule to run concurrently with other
tasks. There's a `spawn()` function that can be used to create a new task to run
a future/async function.

But when I try to move the handler into a task, Rust gives me a lifetime error:

```rust
            let request = Request {};
            println!(" request: {:?}", request);

            async_std::task::spawn(app.handle(request));
```

```
error[E0597]: `app` does not live long enough
   |
41 |             async_std::task::spawn(app.handle(request));
   |                                    ^^^----------------
   |                                    |
   |                                    borrowed value does not live long enough
   |                                    argument requires that `app` is borrowed for `'static`      ...
44 | }
   | - `app` dropped here while still borrowed
```

What is the problem?

The async `handle` method holds a reference to `&self`, which is the `WebApp`
we've defined in `main()`. The value will remain alive on the stack for as long
as our `main()` function keeps running. But now we're trying to run the handler
on its own task, outside of the `main()` function, so it can't know whether the
`WebApp` object will still be alive when task is run, potentially on a different
thread. (In our case, main() will probably still be running because it's an
infinite loop, but the language doesn't guarauntee that and even in our case an
unexpected `panic!()` could cause main to be cleaned up while the task is still
alive.)

How do we solve this?

We want to make sure that the `WebApp` value remains alive as long as _either_
(a) `main` is still running with it, or (b) there are any remaining handler
functions referring to it. We do this by moving it from the stack to the heap,
storing it inside of a `std::sync::Arc` Atomically Reference-Counted smart
pointer. This allows you to create multiple references to a value, and the value
will remain alive as long as any of those references remain.

_Aside: you should be aware that this reference counting adds some runtime
overhead, particularly since we're using the thread-safe Atomic version (there's
a much faster `std::rc::Rc` non-thread-safe version for cases where you don't
need that). Don't worry about this too much, but consider it before you replace
every reference in your program with an `Arc`._

We do this by wrapping `WebApp::new()` with `Arc::new()`...

```rust
async fn main() {
    let app = Arc::new(WebApp::new());
```

...calling `.clone()` to create a new reference for `.handle()`'s task...

```rust
            async_std::task::spawn(app.clone().handle(request));
```

...and updating the definition of `handle` to expect an `Arc` reference instead
of a bare `&` reference.

```rust
    async fn handle(self: Arc<Self>,  request: Request) -> Response {
```

(This currently requires `#![feature(arbitrary_self_types)]` and Rust nightly,
but the syntax will probably be stablized eventually and the alternative is
messier.)

With these changes, it works, and we can see that the slow responses aren't
blocking new requests:

```
[t=   1.0s]  request: Request {}
[t=   2.0s]  request: Request {}
[t=   3.0s]  request: Request {}
[t=   3.5s] response: Response {}
[t=   4.0s]  request: Request {}
[t=   4.5s] response: Response {}
[t=   5.0s]  request: Request {}
[t=   5.5s] response: Response {}
[t=   6.0s]  request: Request {}
[t=   6.5s] response: Response {}
[t=   7.0s]  request: Request {}
[t=   7.5s] response: Response {}
[t=   8.0s]  request: Request {}
```

Here's the full code we have to this point. (EDITOR: There's also a minor change
to add the response printing back, which I elided above because it would be
noisy.)

```rust
#![feature(arbitrary_self_types)]
#![allow(unused)]
use async_std::prelude::*;
use rand::prelude::*;
use std::{collections::BTreeMap, rc::Rc, sync::Arc, time::Duration};

#[derive(Debug)]
struct Request {}

#[derive(Debug)]
struct Response {}

#[derive(Debug)]
struct WebApp {}

impl WebApp {
    fn new() -> WebApp {
        WebApp {}
    }

    async fn handle(self: Arc<Self>,  request: Request) -> Response {
        // Pretend we're waiting for a slow backend.
        async_std::task::sleep(Duration::from_secs(5)).await;
        Response {}
    }
}

#[async_std::main]
async fn main() {
    let app = Arc::new(WebApp::new());

    loop {
        // For some variety, let's say that every one second...
        async_std::task::sleep(Duration::from_secs(1)).await;
        // ...we have a 20% probability...
        if 0.20 > rand::thread_rng().gen_range(0.0, 1.0) {
            // ...of simulating a request.
            let request = Request {};
            println!(" request: {:?}", request);

            let app_for_task = app.clone();
            async_std::task::spawn(async {
                let response = app_for_task.handle(request).await;
                println!("response: {:?}", response);
            });
        }
    }
}
```

# TODO

database updates! first as a batch, then per-row?

---

Context: I have a web server that's serving almost-static data from a `HashMap`. Ocassionally I need to replace the data, all at once, so I currently using an `RwLock<Arc<HashMap<_, _>>>`, where each request handler takes a read lock, and ocassionally a different thread takes a write lock to replace the `Arc<HashMap<_, _>>` with a different one. (I am _not_ mutating the existing HashMap, not mutating it.)

Question: It feels like using a lock here might be overkill, since I don't need to mutate the data in-place. I just need to replace reference with another, which seems like it should be able to use a single atomic swap instead of a full lock. (Followed by dropping) Is there an alternative to `RwLock` that would provide it? Or in this case, should an `RwLock` be as efficient as a atomic swap, anyway.

Ahh but that requires I have no outstanding readers....

Unless they're all cloning the Arc, which is more overhead than I'd be avoiding. Huh.
