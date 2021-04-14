### Rust 'futures'.

- Like in JS ...with Promises a future in Rust is a way to **express a value that is not yet ready**.
  - could be 'compute heavy' or 'io blocking'. Computing hashes that are taking a long time or waiting for a network response.
- Why have a multi threaded server to manage many connections???? Why not have multiple 'Promises' around one thread?

---

- Let's say you have a server with 1000 connections...instead of spinning up 1000 seperate threads to handle their own connection....We can have 1 thread (or say 10 threads)... that have a 'future' representing each unique connection....handling the connections that are ready at the time.

- all the magic of futures occures in the 'poll' function.
- Futures can be advanced by calling the poll function, which will drive the future as far towards completion as possible.
- 'Futures' are the stepping stone to async/await.
- the std::futures::....crate is how to define the infrustructure for futures, but it does NOT implmenent it. It's the interface for dealing with asynchronous code. Doesn't give you an 'executer'...**only defines the interfaces**.
  'tokio' is the actual implementation. It handles the tasks and the 'waking up' of futures. It implements the infrustructure needed.
