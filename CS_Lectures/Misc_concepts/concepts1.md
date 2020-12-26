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
- Coroutines are very similar to threads. However, coroutines are cooperatively multitasked, whereas threads are typically preemptively multitasked. This means that coroutines provide concurrency but not parallelism.
- **Generators**, also known as semicoroutines, are a subset of coroutines. Specifically, while both can yield multiple times, suspending their execution and allowing re-entry at multiple entry points, they differ in coroutines' ability to control where execution continues immediately after they yield, while generators cannot, instead transferring control back to the generator's caller. That is, since generators are primarily used to simplify the writing of iterators, the yield statement in a generator does not specify a coroutine to jump to, but rather passes a value back to a parent routine.

Coroutines are useful to implement the following:

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
