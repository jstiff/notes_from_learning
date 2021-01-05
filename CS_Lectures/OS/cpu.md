## CPU stuff...

#### I/O

Cpu's are 'bound'...**i/o bound** or affected by the time it takes for i/o requests to finish. They are not blocked, but bound...

- as for as I/O is concerned. The CPU **initiates** all input/output requests....but, when the CPU issues an I/O request to the hard disk, the hard disk has its own specialized chip called a device (or hardware) controller designed solely for processing commands from the CPU, such as reading from the disk.
- Modern hardware controllers are basically their own microprocessors with firmware and everything, so they are capable of very complex operations without the main CPU's help. While the hard drive's controller is busy performing the request, the main CPU is free to do whatever it wishes.
- The controller is able to read and write directly to and from system RAM using what is called a Direct Memory Access (DMA) controller, a special unit that transfers data from the hardware controller to main RAM without the CPU needing to do anything.
- When the hard drive is done with the request and the relevant data has been loaded into RAM through DMA, it issues an interrupt request which informs the CPU that the data has been loaded into RAM. At this point the CPU can transfer control back to an originating process.
- CPU does not need to micromanage all tasks involved with I/O. At one time this used to be the case, but these tricks (interrupts, DMA, special controllers) were invented in order to improve CPU performance and make things more efficient.

#### Threads...

- A thread is the smallest possible segment of sequenced instructions that can be independently managed by a CPU scheduler... threads give a program parallelism – if a program has multiple threads, each of those threads could run independently on multiple CPU cores, making the program exponentially faster.
- Threads and processes are different but similar. A process **defines the address space, text, resources**, and so on needed to complete some unit of work for a computer complete. A thread defines a **single sequential execution stream within a process**. So a thread can be thought of as a unit within a process (assuming that the programmer has designated threads). There can be many threads within a process, and many processes within a computer. **Threads are bound to a single process and cannot go outside the process**. Within each process, the address space is shared and **no system calls are required** to cooperate amongst threads.

#### Kernal Threads

- threads within the kernel, also known as **lightweight processes**.
- Switching between kernel threads of the same process requires a smaller context switch than if the OS were switching between process. This is because only the value of the registers, program counter, and stack pointer must be changed, the memory management does not need changing because the threads share address space. For kernel processes, the more threads it creates, the more time is allocated to it by the CPU scheduling algorithm.
-

#### User-Level Threads.

- OS does **not actually know about threads** that reside within the user level. The OS only sees the process, not the thread(s) within it. Therefore, the OS schedules the process, but not the threads within the process. Programmers use a thread library to manage threads (create and delete them, synchronize them, and schedule them).
- Remember, user-level threads do not require system-calls to create them or context switches to move between them. The result of this is that user-level threads are typically much faster than kernel threads.
- Disadvantage of User-level threads... is that there is **no true parallelism with user-level threads**. Multiple threads within the same process **do not access separate cores** simultaneously (concurrently), because sees only the process, not the individual threads.
- the OS may make poor scheduling decisions due to its lack of knowledge of the user-level threads...example if a user-level thread is waiting for I/O, the entire process will wait. The way to solve this problem is communication between the kernel and the user-level thread manager.
- Sometimes User-level threads are referred to as 'green threads', 'fibers', **'coroutines'**...These threads only exist inside your language and not in your OS.
- Green threads are much simpler for the programmer, but their performance varies: If you have a LOT of threads, green threads can be better for both CPU and RAM. On the other hand, **most green thread languages can't take advantage of multiple cores**. (You can't even buy a single-core computer or phone anymore!). And a bad library can halt the entire language by doing a blocking OS call.
- User threads and Kernel threads are exactly the same. (You can see by looking in /proc/ and see that the kernel threads are there too.

  - A User thread is one that executes user-space code. But it can call into kernel space at any time. It's still considered a "User" thread, even though it's executing kernel code at elevated security levels.

  - A Kernel thread is one that only runs kernel code and isn't associated with a user-space process. These are like "UNIX daemons", except they are kernel-only daemons. So you could say that the kernel is a multi-threaded program. For example, there is a kernel thread for swap. This forces all swap issues to get "serialized" into a single stream.
  - all threads start off in kernel space, because the clone() operation happens in kernel space. (And there's lots of kernel accounting to do before you can 'return' to a new process in user space.

    - User-level threads are threads that the OS is not aware of. They exist entirely within a process, and are scheduled to run within that process's timeslice.
    - The OS is aware of kernel-level threads. Kernel threads are scheduled by the OS's scheduling algorithm, and require a "lightweight" context switch to switch between (that is, registers, PC, and SP must be changed, but the memory context remains the same among kernel threads in the same process).
    - User-level threads are much faster to switch between, as there is no context switch; further, a problem-domain-dependent algorithm can be used to schedule among them. CPU-bound tasks with interdependent computations, or a task that will switch among threads often, might best be handled by user-level threads.
    - Kernel-level threads are scheduled by the OS, and each thread can be granted its own timeslices by the scheduling algorithm. The kernel scheduler can thus make intelligent decisions among threads, and avoid scheduling processes which consist of entirely idle threads (or I/O bound threads). A task that has multiple threads that are I/O bound, or that has many threads (and thus will benefit from the additional timeslices that kernel threads will receive) might best be handled by kernel threads.

  #### User-Level Threads are atually implemented by being mapped to Kernal Threads....

- Each user thread **can't actually run on its own**, and the only way for a user thread to run is if a kernel thread is actually told to execute the code contained in a user thread.
- A reason that user threads have to be associated with kernel threads is that by itself a user thread is just a bunch of data in a user program. Kernel threads are the real threads in the system, so for a user thread to make progress the user program has to have its scheduler take a user thread and then run it on a kernel thread.

  - There are four models for mapping to Kernal threads..
    1. Many-to-one
    2. One-to-one
    3. Many-to-many
    4. Two-level

- **'Many user to One'** Kernel....Because a single kernel thread can operate only on a single CPU, the many-to-one model does not allow individual processes to be split across multiple CPUs. Very few systems use this model these days.
- **One to One**...creates a separate kernel thread to handle each user thread...the overhead of managing the one-to-one model is more significant. Most implementations of this model place a limit on how many threads can be created.
- **Many-to-Many** model multiplexes any number of user threads onto an equal or smaller number of kernel threads

#### Rust uses a 'User Mode Scheduler' for threading...

- when you call ...thread::spawn()...this will generate a C based sys call that the Operating system can do.

#### Thread Pools

- Creating new threads every time one is needed and then deleting it when it is done can be inefficient, and can also lead to a very large ( unlimited ) number of threads being created.
- An alternative solution is to create a number of threads when the process first starts, and put those threads into a thread pool.
  - Threads are allocated from the pool as needed, and returned to the pool when no longer needed.
  - When no threads are available in the pool, the process may have to wait until one becomes available.
- The ( maximum ) number of threads available in a thread pool may be determined by adjustable parameters, possibly dynamically in response to changing system loads.
