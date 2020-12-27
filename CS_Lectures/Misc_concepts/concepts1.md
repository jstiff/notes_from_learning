## Concepts I hear a lot and do not understand.

- **co-routines**: made up of two ideas...

1. co --> cooperative
2. routine --> can be thought of as 'functions' we use day to day.

Cooperative functions will share the execution....unlike normal functions that will sequentially finish off its own stuff..

```

typical scenario when a function calls another.

normalFunctionOne(){
    ...
    ...
    normalFunctionTwo();  // When called, it is popped onto the stack and executed line by line until finnished.
    ...  // normalFunctionOne will will resume executing here after functionTwo is completed and returned control.
    ...
    ...


Co-Routines....


 co-RoutineFucntionOne () {

     ...
     ...
     ...
     co-RoutineFunctionTwo();  --------------------> co-RoutineFunctionTwo(){  // is called
                                                                .....
                                                                .....
                                                                .....
                                <--------------------------   can pass control back
    ....
    ....                       ---------------------------->     get conrtol again...
                                                                .....
                                                                .....
                            < -----------------------------     pass it back.
    ....
    ....
    finished executing!


 }

}

Each function can also pass each other values during this process to use durring its execution.

Python and JavaScript uses the 'yield' syntax. This is from Generators I believe.

```

WIkipedia...

- also known as non-preemptive multitasking, is a style of computer multitasking in which the operating system never initiates a context switch from a running process to another process. Instead, processes voluntarily yield control periodically or when idle or logically blocked in order to enable multiple applications to be run concurrently.

- Coroutines are well-suited for implementing familiar program components such as cooperative tasks, exceptions, event loops, iterators, infinite lists and pipes.

- According to Donald Knuth, Melvin Conway coined the term coroutine in 1958 when he applied it to the construction of an assembly program. The first published explanation of the coroutine appeared later, in 1963.

- Subroutines are special cases of coroutines. When subroutines are invoked, execution begins at the start, and once a subroutine exits, it is finished; an instance of a subroutine only returns once, and does not hold state between invocations. By contrast, coroutines can exit by calling other coroutines, which may later return to the point where they were invoked in the original coroutine; from the coroutine's point of view, it is not exiting but calling another coroutine. Thus, a coroutine instance holds state, and varies between invocations; there can be multiple instances of a given coroutine at once. The difference between calling another coroutine by means of "yielding" to it and simply calling another routine (which then, also, would return to the original point), is that the relationship between two coroutines which yield to each other is not that of caller-callee, but instead symmetric.
- Any subroutine can be translated to a coroutine which does not call yield.
- Coroutines are very similar to threads. However, **coroutines are cooperatively multitasked, whereas threads are typically preemptively multitasked**. This means that coroutines provide concurrency but not parallelism.
- **Generators**, also known as semi-coroutines, are a subset of coroutines. Specifically, while both can yield multiple times, suspending their execution and allowing re-entry at multiple entry points, they differ in coroutines' ability to control where execution continues immediately after they yield, while generators cannot, instead transferring control back to the generator's caller. That is, since generators are primarily used to simplify the writing of iterators, the yield statement in a generator does not specify a coroutine to jump to, but rather passes a value back to a parent routine.
- because coroutines can only be rescheduled at specific points in the program and do not execute concurrently, programs using coroutines can often avoid locking (Mutexing) entirely...This property is also cited as a benefit of event-driven or asynchronous programming.

- In order to implement general-purpose coroutines, a second call stack must be obtained, which is a feature not directly supported by the C language. A reliable (albeit platform-specific) way to achieve this is to use a small amount of inline assembly to explicitly manipulate the stack pointer during initial creation of the coroutine. This is the approach recommended by Tom Duff in a discussion on its relative merits vs. the method used by Protothreads

- Back in the day systems would spawn more 'processes' and do expensive 'context switching' between them to run many tasks concurrently. Eventually a new layer of 'indirection' was created called 'threads' which was sort of a process with its own call stack and resembled a set of instructions...but all existed within a single process...each thread shares the resources of its parent process.
- **Cooperative Schedulaling**...(non-preemptive multitasking) the Operating System NEVER initiates context switching from one process to another. The actual programes must 'cooperate' with each other...(???). This must be programed through the 'yield' model. (for example). In this scheme, the process scheduler of an operating system is known as a cooperative scheduler, having its role reduced down to starting the processes and letting them return control back to it voluntarily. As a cooperatively multitasked system relies on each process regularly giving up time to other processes on the system, one poorly designed program can consume all of the CPU time for itself, either by performing extensive calculations or by busy waiting; both would cause the whole system to hang. Cooperative multitasking is used with await in languages with a single-threaded event-loop in their runtime, like JavaScript or Python.

- **Preemptive scheduling** is initiated by the Operating system. Uses interupts to activate a schedualer to assign differnt processes to the cpu. Preemptive multitasking allows the computer system to more reliably guarantee each process a regular "slice" of operating time. It also allows the system to **rapidly deal** with important external events like incoming data, which might require the immediate attention of one or another process.
- processes can be grouped into two categories: those that are waiting for input or output (called "I/O bound"), and those that are fully utilizing the CPU ("CPU bound")
- In early systems, processes would often "poll", or "busy-wait" while waiting for requested input (such as disk, keyboard or network input). During this time, the process was not performing useful work, but still maintained complete control of the CPU. With the advent of interrupts and preemptive multitasking, these I/O bound processes could be "blocked", or put on hold, pending the arrival of the necessary data, allowing other processes to utilize the CPU. As the arrival of the requested data would generate an interrupt, blocked processes could be guaranteed a timely return to execution. Although multitasking techniques were originally developed to allow multiple users to share a single machine, it soon became apparent that multitasking was useful regardless of the number of users.
- The period of time for which a process is allowed to run in a preemptive multitasking system is generally called the time slice or quantum. The scheduler is run once every time slice to choose the next process to run. The length of each time slice can be critical to balancing system performance vs process responsiveness - if the time slice is too short then the scheduler will consume too much processing time, but if the time slice is too long, processes will take longer to respond to input.
- Today, nearly all operating systems support preemptive multitasking, including the current versions of Windows, macOS, Linux (including Android) and iOS.

- **'Stackless'** coroutines

  - ...utilze the **heap** in order to store data before 'context switching' of functions...
    or they can also use an 'activation record'(equivalent to a stack frame).

    - otherwise the local variables of the stackless coroutine would be overwritten by invocations of ordinary functions after suspending the stackless coroutine
    - **suspending from a deep call stack is only possible if all functions in between are stackless coroutines too (viral; otherwise you would get a corrupted stack)**
    - stackless coroutines do need memory to store local variables too, especially if the coroutine gets suspended the local variables need to be preserved

    - each stackless coroutines requires its own activation record -> called in a deep call chain a lot activation records have to be created/allocated.

- **'Stackful'** coroutines
  - in its essence a stackful coroutine simply switches stack and instruction pointer.
  - allocate a side-stack that works like a ordinary stack (storing local variables, advancing the stack pointer for called functions)
  - stackful coroutines allow to suspend from a deep call chain while the functions in between can be ordinary functions (not viral)
  - a stackful coroutine can outlive its caller/creator
  - the industry is heading towards stackless ...blocking I/O is a dangerous illusion. Any time you leave your address space you are fundamentally engaged in an asynchronous exchange. APIs that hide this from people -- even in the name of "ease of use" -- are simply inviting all sorts of unexpected behavior or worse, deadlocks. Stackful coroutines hide their blocking nature from the caller and this makes it very difficult to reason about their behavior.
- The stackful vs stackless effectively refers to whether the implementation uses the same stack discipline for calls that can yield vs those that cannot. In either case you're always going to have to construct some kind of stack to support nested function invocation, the question is whether you're going to duplicate all that infrastructure.

- All coroutine implementations share a common algorithm, called the coroutine transformation (henceforth termed "transformation"). This algorithm takes ordinary code annotated with suspend points and inserts additional code allowing the thread of execution to be paused at each suspend point and later resumed. At a minimum, the transformation must store sufficient state to identify the last suspend point from which the coroutine should be resumed. An implementation may also choose to preserve the values of some or all local variables and function arguments.

### Coroutines are useful to implement the following:

1. State machines within a single subroutine, where the state is determined by the current entry/exit point of the procedure; this can result in more readable code compared to use of goto, and may also be implemented via mutual recursion with tail calls.
2. Actor model of concurrency, for instance in video games. Each actor has its own procedures (this again logically separates the code), but they voluntarily give up control to central scheduler, which executes them sequentially (this is a form of cooperative multitasking).
3. Generators, and these are useful for streams – particularly input/output – and for generic traversal of data structures.
4. Communicating sequential processes where each sub-process is a coroutine. Channel inputs/outputs and blocking operations yield coroutines and a scheduler unblocks them on completion events. Alternatively, each sub-process may be the parent of the one following it in the data pipeline (or preceding it, in which case the pattern can be expressed as nested generators).
5. Reverse communication, commonly used in mathematical software, wherein a procedure such as a solver, integral evaluator, ... needs the using process to make a computation, such as evaluating an equation or integrand.

#### Javascript

Since ECMAScript 2015, stackless coroutine functionality through "generators" and yield expressions is provided.
Javascript implements 'stackless' co-routines.

The key distinction is not whether a coroutine ever has a stack, but rather whether a coroutine needs a stack while it is suspended. If you look closely, a Javascript async function doesn’t need a stack while suspended.
This is because you can only call await within the body of f itself - we can never suspend while g is running.
we only need a stack while the coroutine is running. When the coroutine suspends, it can serialize it’s local variables into a fixed-size structure, then use the current call stack for executing the next coroutine.

```
function g() { return 0; };

async function f() {
  let val = g();
  await sleep_async(10);
  return val;
}
```

#### Rust

from [Sam Sartor Blog](https://samsartor.com/)

Rust doesn't have a heavy runtime so coroutines have to be assembled somehow out of ordinary functions and structs. an anonymous struct is exactly what async produces.
The coroutine is executed by calling a resume function which can later pause execution by saving variables into the struct and returning back to the caller. This is why they are called "stackless coroutines". It also means that any coroutine syntax in Rust (e.g. async/await) is just a fancy way of creating a state machine! For example, consider the following async code:

```
let my_coroutine: Future<Output=u32> = async {
    some_function().await + 3
};
```

Stackful coroutines are actually a perfect fit for Rust's ownership model, and would simplify much of the work, or obviate it altogether. But for other reasons--C compatibility, poor OS constructs for minimizing memory use, and an unfortunate early conflation of coroutines with async I/O--Rust has chosen the path of stackless coroutines a la async/await as the official model.

1. Python Async is "stackless coroutine"
2.

## Threads - Fibers - Coroutines - Closures
