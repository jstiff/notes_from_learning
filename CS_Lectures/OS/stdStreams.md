### StdStreams....stdin, stdout, stderr

- standard streams are interconnected input and output communication channels between a computer program and its environment when it begins execution.
- Originally I/O happened via a physically connected system console (input via keyboard, output via monitor), but standard streams abstract this.
-
- One of Unix's several groundbreaking advances was abstract devices.
  - In most operating systems predating Unix, programs had to explicitly connect to the appropriate input and output devices.

#### Redirection

- a function common to most command-line interpreters, including the various Unix shells that can redirect standard streams to user-specified locations.
- Redirection is a form of interprocess communication. Some forms of IPC include:
  - File:
  - Socket:
  - Message Queue:
  - Pipe:
  - Message passing:
  - etc...
