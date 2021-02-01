#### Based on blog....[blog](https://os.phil-opp.com/freestanding-rust-binary/)

- In a typical Rust binary that links the standard library, execution starts in a C runtime library called crt0 (“C runtime zero”), which sets up the environment for a C application. This includes creating a stack and placing the arguments in the right registers. The C runtime then invokes the entry point of the Rust runtime, which is marked by the start language item.
- Rust only has a very minimal runtime, which takes care of some small things such as setting up stack overflow guards or printing a backtrace on panic. The runtime then finally calls the main function.

## The Rust runtime entry point...which is marked by the 'start' language item.

    - this entry point is envoked by C...crt0 (“C runtime zero”)

```

#[lang = "start"]

fn lang_start(main: fn(), argc: isize, argv: *const *const u8) -> isize {
    use panic;
    use sys;
    use sys_common;
    use sys_common::thread_info;
    use thread::Thread;
    #[cfg(not(feature = "backtrace"))]
    use mem;

    sys::init();

    let failed = unsafe {
        let main_guard = sys::thread::guard::init();
        sys::stack_overflow::init();

        // Next, set up the current Thread with the guard information we just
        // created. Note that this isn't necessary in general for new threads,
        // but we just do this to name the main thread and to give it correct
        // info about the stack bounds.
        let thread = Thread::new(Some("main".to_owned()));
        thread_info::set(main_guard, thread);

        // Store our args if necessary in a squirreled away location
        sys::args::init(argc, argv);

        // Let's run some code!
        #[cfg(feature = "backtrace")]
        let res = panic::catch_unwind(|| {
            ::sys_common::backtrace::__rust_begin_short_backtrace(main)
        });
        #[cfg(not(feature = "backtrace"))]
        let res = panic::catch_unwind(mem::transmute::<_, fn()>(main));
        sys_common::cleanup();
        res.is_err()
    };

    if failed {
        101
    } else {
        0
    }
}



```

### VGA Text buffer.

- 0xb8000 is the address for
