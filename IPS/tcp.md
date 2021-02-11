### TCP/IP

- When HTTP wants to transmit a message, it **streams** the contents of the message data, in order, through an open TCP connection. TCP takes the stream of data, chops up the data stream into chunks called segments, and transports the segments across the Internet inside envelopes called IP packets (see Figure 4-4). This is all handled by the TCP/IP software; the HTTP programmer sees none of it.

- HTTP streams --> TCP segments --> IP packets

#### History

- The Internet protocol suite resulted from research and development conducted by the Defense Advanced Research Projects Agency (DARPA) in the late 1960's.
- Initially, the Transmission Control Program managed both datagram transmissions and routing, but as experience with the protocol grew, collaborators recommended division of functionality into layers of distinct protocols.
- Encapsulation of different mechanisms was intended to create an environment where the upper layers could access only what was needed from the lower layers. A monolithic design would be inflexible and lead to scalability issues.
- the Transmission Control Program was split into two distinct protocols, the Internet Protocol as connectionless layer and the Transmission Control Protocol as a reliable connection-oriented service.
