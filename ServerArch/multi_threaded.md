### Multi-Threaded model

- Basically, there is a 'thread pool' with a pre-determined amount of threads inside of it, a main 'acceptor' thread and a queue.

  1. Thread Pool
  2. Acceptor Thread
  3. Queue

- All incomming connections first go through the Acceptor thread and are placed into the queue
  - the Thread Pool then picks off at the queue to process the requests.
  - If there are more threads then requests then those threads will remain 'idle' inside the pool.
