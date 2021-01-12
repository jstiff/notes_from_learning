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

### Shell

- The shell is your interface to the operating system. It acts as a command interpreter.
- It is named a shell because it is the outermost layer around the operating system.
- A terminal is an interface that runs a 'shell' program.
  - the shell is a way to access the Kernal.
- ex) command “ls” is a file, that when it is executed, the system creates a process corresponding to an instance of a running program and that has an ID or PID, which is not more than a unique number, used by several functions or system calls (syscalls) to manipulate these processes.
  - the "ls" is a child process that is spawned by the parent process(Shell itself).
- Every command starts up a **process** that is provided a default stdin, stdout,stderr to write or read too. YOu can direct or redirect data to other locations... > , < , | ...

### Terminal vs Shell

- The terminal emulator (often just called terminal) is “just the window”, yes. It runs a text based program, which by default is your login shell (which is bash in Ubuntu). When you type characters in the window, the terminal draws these characters in the window in addition to sending it to the shell’s (or other program’s) stdin. The characters the shell outputs to stdout and stderr get sent to the terminal, which in turn draws these characters in the window.

### Fun Fact about the 'cd' command..

- 'cd' is a type of program that is run differently in the shell...there are certain commands that the shell cannot simply dispatch to another process. These are things which affect something internal to the shell, and thus must be implemented by the shell itself
  - Dennis Ritchie:
    - In the midst of our jubilation, it was discovered that the chdir (change current directory) command had stopped working. There was much reading of code and anxious introspection about how the addition of fork could have broken the chdir call. Finally the truth dawned: in the old system chdir was an ordinary command; it adjusted the current directory of the (unique) process attached to the terminal. Under the new system, the chdir command correctly changed the current directory of the process created to execute it, but this process promptly terminated and had no effect whatsoever on its parent shell! It was necessary to make chdir a special command, executed internally within the shell. It turns out that several command-like functions have the same property, for example login.
