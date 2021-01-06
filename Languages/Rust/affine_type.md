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
