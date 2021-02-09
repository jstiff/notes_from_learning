## Process

- the kernal provides and interface to the process abstraction.
- Most fundamental abstraction the Operating System implements.
  - They organize information about other abstractions and represent a 'single thing' that the computer
    is doing.
- Every application running on your computer or phone has it's own process.
- They are not tied to any hardware. It is a sort of 'container' around other abstractions that are indeed
  tied to hardware.
- act as a 'Protective boundry' from other processes on the system. You can do anything you want on a process and it will not effect other apps.

#### threads

- inside a process you can have many threads. Threads represent a line of execution...
- MOST everything in the process can be accessed by the threads. Threads share a lot of things such as address space.
- They get their own stack.

#### IPC

- If two diff apps want to talk....
- you can **set up a Socket...**
- Shared files, pipes, return codes, signals, shared memory....

1. **Return codes** are simplest way to do IPC.

   - When a process returns an int to the process that created it. 0 == SUCCESS and 1 == FAILURE
   - to get the exit code of the last running process enter .... echo $? ...usually just get 0.

   - 'ps aux'....will give you all the running processes on the machine. like a stripped down htop.

2. Pipes...

- take the output of some process and pass it to the input of another process.
- It's **one way**
- Has a read...and then a write file descriptor.
- OS manages a queue for each pipe to accomodate diff input/output rates.
- **pipe manipulates the file handles between the two processes**
  - ex) ps aux | grep jasonstiff...
    ps thinks it's writing to Stdout, but pipe manipulates it to write to stdin of grep.

3. Signals...
   - limited form of IPC.
   - When a signal is sent, the operating system interrupts the target process' normal flow of execution to deliver the signal.
   - Execution can be interrupted during any non-atomic instruction.
   - Signals are similar to interrupts, the difference being that interrupts are mediated by the processor and handled by the kernel while signals are mediated by the kernel (possibly via system calls) and handled by processes.

#### kill

- SIGTERM...'signal terminate'???? maybe...When you hit ctl-c...you are actually sending a SIGTERM to the current running process.
- if SIGTERM does not work you will need to pass the process SIGKILL.
- kill can do many things actually...
