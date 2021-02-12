### Single Threaded Model

- Single thread model is very similar to that of a scheduler, multiplexing multiple connections to a single flow of execution.
- There is a queue that buffers the incomming requests and an 'Event Loop' that yanks them from the queue to process on that single thread.
- Processing an event either **requires registered event handler** code for specific events, or it is based on the execution of a callback associated to the event in advance.
- The different states of the connections handled by a thread are organized in appropriate data structures-- either explicitly using finite state machines or implicitly via continuations or closures of callbacks.
- Instead of sequential operations, an event-driven program uses a cascade of asynchronous calls and callbacks that get executed on events. This notion often makes the flow of control less obvious and complicates debugging.
- The usage of event-driven server architectures has historically depended on the availability of asynchronous/non-blocking I/O operations on OS level and suitable, high performance event notification interfaces such as epoll and kqueue.

#### Non-blocking I/O Multiplexing Patterns

1. **Reactor Pattern**
   - targets synchronous, non-blocking I/O handling and relies on an event notification interface.
   - this pattern registers a set of resources (e.g. a socket) and events (e.g. a new connection) it is interested in.
   - For each resource event the application is interested in, an appropriate event handler must be provided--a callback or hook method.
     - The core component of the Reactor pattern is a **synchronous event demultiplexer**.
     - Whenever it receives an event (e.g. a new client connection), it notifies a dispatcher and awaits for the next event. The dispatcher processes the event by selecting the associated event handler and triggering the callback/hook execution.
     - Other variants of the Reactor pattern use a thread pool for the event handlers.
2. **Proactor Pattern**
   - leverages truly asynchronous, non-blocking I/O operations, as provided by interfaces such as POSIX AIO.
   - represents the main application thread and is responsible for initiating asynchronous I/O operations.
   - The execution of the asynchronous operation is governed by the asynchronous operation processor, an entity that is part of the OS in practice.
   - Has better multithreading support. The execution of completition handlers can easily be handed off to a dedicated thread pool.

#### Scalability Considerations

- Single threaded...gets rid of the overhead of excessive context switching and does not need a thread stack for each connection. This decreases the memory footprint under load and wastes less CPU time to context switching.
- Ideally, the CPU becomes the only apparent bottleneck of an event-driven network application.

#### Event Driven...

- Benifits...API systems don’t have to inherently wait for synchronous delivery or real time communication. This is hugely beneficial, as eliminating the need to constantly poll endpoints frees resources from otherwise wasteful purposes, reducing both general hardware requirements and call-specific overhead.

- like a control flow pattern where objects are passed around and detected. when preestablished patterns or objects are recognized then predetermined actions are executed...
- An event can be defined as "a significant change in state".
- An event-driven system typically consists of event emitters (or agents), event consumers (or sinks), and event channels.

1. Event Emmitter (event generator)
   - Emitters have the responsibility to detect, gather, and transfer events.
   - An Event Emitter does not know the consumers of the event, it does not even know if a consumer exists, and in case it exists, it does not know how the event is used or further processed.
2. Sinks
   - Sinks have the responsibility of applying a reaction as soon as an event is presented. The reaction might or might not be completely provided by the sink itself. For instance, the sink might just have the responsibility to filter, transform and forward the event to another component or it might provide a self-contained reaction to such event.
3. Channels
   - Event channels are conduits in which events are transmitted from event emitters to event consumers. The knowledge of the correct distribution of events is exclusively present within the event channel.
   - An event channel is a mechanism of propagating the information collected from an event generator to the event engine

- Building systems around an event-driven architecture simplifies horizontal scalability in distributed computing models and makes them more resilient to failure.

#### Types of Event Driven Protocols..

1. WebSockets
   - WebSocket is a protocol that provides full-duplex communication on a single TCP connection.
   - RFC 6455
   -
2. WebHooks
   - similar concept to the WebSocket.
   - primarily function using custom callbacks, or code that is passed as an argument to another chunk of code and executed at a specified point in time.
   - is a glorified system of **“if this, then do”**, allowing for users independent of the event firing to craft a custom response to that event within their own system.
   -
3. REST Hooks
   - REST Hooks is essentially “hooking” baked into REST itself.
   - This approach is a response to the practice of polling, in which a client constantly checks for changes to a resource.
   - Under the REST Hooks paradigm, the client instead waits for a change, and reacts to it. To put it simply, this is a WebHook in REST.
4. Pub-Sub (publish-subscribe)
   - where events are published to a class without knowledge of the client subscribing to the class.
   - a user will join one or more classes, and then will receive event updates without regard or knowledge to the event publisher.
   - the user specifies which class they wish to be part of and what events they are interested in receiving.
   - Like a "radio channel"...Record companies, or publishers, issue audio to the station, which then broadcasts this audio to listeners, or subscribers.
5. Server Sent Events
