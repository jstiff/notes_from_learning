### Rust

As a language, Rust really blends strong academic theory with Engineering.

#### Rust uses an 'Affine Type System (Substructual)'

- This is academic speak for the 'Ownership' type system.

- Affine Logic is a version of 'Linear Logic'...with weakening. ???
- Rust type system is designed to insure...

  - An 'object' can either be:

  1. Mutable

     or

  2. Can have multiple immutable active aliases.

     - **cannot have both though**

  - Linear logic is sort of a 'resource sensitive' logic....meaning you **can't reuse stuff**.
  - This logic takes into account the emphemeral realities...if you have a dollar you have it until you use it, then it is gone forever!!!! Resources are consumed!!! yes yes?
  - Conjunction and Disjunction now become ambiguous...need more information...like what 'kind'...so as a consequence Linear logic produces a **duplication** of stuff depending on how we are allocating and using our resources.

- 'What does logic have to do with programming languages?'...
- A type system is there to **constrain** behavior...we are explicit in exactly what we want/need as program to do.
- **"Types are the interface between logic and programing"**.....????
- Logicians care about 'propositions' and programmers care about program behavior.

  - a proposition is the non-linguistic bearer of truth or falsity which makes any sentence that expresses it either true or false. The term is often used very broadly and can also refer to various related concepts.
    It can generally be used to refer to some or all of the following:
    - The primary bearers of truth values (such as "true" and "false");
    - the objects of belief and other propositional attitudes (i.e. what is believed, doubted, etc.).
    - the referents of "that"-clauses (e.g. "It is true that the sky is blue" and "I believe that the sky is blue" both involve the proposition the sky is blue).
    - and the meanings of declarative sentences.

```
            Programming                 Logic
            ----------                 -------
            Functions      ---->        Implication (if)
            Product types  ---->        Conjunction (and)
            Some Types     ---->        Disjunction (or)

            STLC           ---->        Intuitionistic Logic              "Simply Typed Lambda Calculus"
            Polymorphism   ---->        'For All' (2nd ord Logic)
            Control Flow   ---->        Classical Logic                 CL principles are control flow primitives
            Combinator calc  -->        Hilbert Logic (???)

```

- #### Slogans to ponder.

  - 'Propositions are types...'
  - 'Proofs are programs...'

- The Curry-Howard correspondence states that formulating a mathematical proof and writing a (typed, functional) program are basically the same thing. It isn't just an analogy, it's an isomorphism, meaning they act exactly the same.
- 'Curry Howard Correspondence'...is the direct relationship between computer programs and mathematical proofs.
- Affine types are a version of linear types allowing to discard (i.e. not use) a resource, corresponding to affine logic. An affine resource can be used at most once, while a linear one must be used exactly once.
- Types that are not Copy, and are thus moved, are Affine Types: you may use them either once or never, nothing else.
- Rust qualifies this as a transfer of ownership in its Ownership-centric view of the world. Haskell which exposes the math-y/cs-y concepts, Rust tends to expose more pragmatic concepts.
- ***

---

### Mutual Exclusion in Rust.

```
How does Rust support mutating alliased references...?

ex)

    Pointer based data structures like 'Doublely-linked list'.
    Synchronization mechanisims...'Locks, channels , semaphores'.
    Memory managment...'Ref counting'.

- collaberation between two diff API's...Arc and Mutex provides us with a way to implement 'aliased mutable state'...

let data = Arc::new(Mutex::new(x));



```

- Mutex: provides synchronized access to some object through a 'lock'. If you have an Object<T> you can **wrap** it in a Mutex and in order to get access to Object<T> you need to aquire the 'lock'.
- Arc: allows dynamic sharing of that Mutex by implementing 'reference counting'. It 'Wraps' the mutex in a reference counting object...

#### Rust is actually implemented with 'unsafe' code.

- In order to implemented shared mutablility there must be unsafe code...???
- What Rust provides are these API's that are technically implmeneted in a way that makes being a client of the Api safe to use. So, it's built on a confidently designed abstraction layer that we can be 'safe' in Rust.

### Enter stage left....'RustBelt'.

- Goal of 'rustbelt' is to establish the 1st logical foundations for the language.

  - Use this logic to verify the safety of the Rust core type system and std libraries.
  - Give developers the tools to saftly evolve the language.

- What are they trying to prove? What is 'safety'?
  - The 'standard' is what is called **'syntactic safety'**. This is taught a lot and used because it is very simple, but it will **not** work for Rust. As soon as you have any unsafe code it does not work.
  - What RustBelt tries to do is 'generalize to **semantic safety**'.
    - Sort of lift up the idea of safety to mean it is safe 'if no well-typed code using it can have undifined behavior'
    - What exactly is 'undefined behavior'?????
      - Basically, a standard notion of **not accessing memory outside the regions that are allocated**
      - or not haveing data races based on some 'operational definition' of what a data race is....no unsynchronized access to memory from diff threads.
      - or means...there is an 'operational semantics' they give for a language called **Lambda Rust**...which is a sort of 'core Rust Lambda Calculus'...that is modled after the 'MIR' (Middle Intermediate Representation) of Rust...that has an 'operational semantics' that has the ability to 'get stuck'...get into some bad state. And the prove that 'well typed' Rust will never get into these states.
- There is a problem with the 'syntactic safety' approach in that it doesn't allow for expanding the primitives of the language...for instance in building API's. Without having to go back and revise the existing proofs.

#### Semantic Safety

-

```
 Library interface  ->  Semantic Model --> Safety Contract ---> Logical Satisfaction  ==> Library Implementation.
```

- So the standard libraries have been proved and are 'safe by **construction**'...(???). While anything implemented internally that uses unsafe code needs to be 'manually verified'. It needs to be checked against the Safety Contract....(???). Arc, Mutex, RefCell, channel...need to be manually verified.
- RustBelt uses 'Coq' (interactive theorem prover) to assist in updating the model.
- Rest of talk RustBelt[POPL'18] paper
  - The key challenge is choosing what logic to use...
    - They use a derivative of **'Seperation Logic'. An extension of Hoare logic**..for reasoning about pointer-manipulating programs.
    - It's a perfect fit for Rust becuase it models 'Ownership' nicely.
