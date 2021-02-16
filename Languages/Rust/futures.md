### Rust 'futures'.

- Like in JS ...with Promises a futures in Rust is a way to **express a value that is not yet ready**.
  - could be 'compute heavy' or 'io blocking'. Computing hashes that are taking a long time or waiting for a network response.
- Why have a multi threaded server to manage many connections???? Why not have multiple 'Promises' around one thread?
