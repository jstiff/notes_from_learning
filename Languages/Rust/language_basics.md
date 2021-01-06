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

- All behavior of data in Rust is implemented within **Traits**...there are two very important traits to know.
  1. Copy --> will implicitly make copies if you break Rust normal rules...'there can only be one owner'. Primitives are able to break the rule, because they are safe to duplicate. For custom types you have the 'Clone' trait.
  2. Clone -->
- Traits are a set of function definitions for a given Type. Be it custom type of built in type. Think of traits as 'base classes'. You need to bring them into scope when you need them because they are the declarations...
- Everything is immutable by default. Read-only access.
- VERY strongly 'typed'.

- Numeric types...the usize and isize are what the Cpu use natively ...the width and a pointer in the cpu..(???).

  - Floating point --> f32 , f64
  - signed Int --> i8, i16, i32, i64, isize
  - un_signed --> u8, u16, u32, u64, usize

  Numbers have methods....in Rust we can give methods to **any** type. Taits are a way to define methods on types.

  - almost everything in Rust returns a value...the return statment is implied. Even things that do not return a value actually return a value...it's known as (), pronounced unit...it evalutates to the **'unit type'**. Its purpose is to be useless. Found in Haskell as well.

  a Result<(), String> can be used as return type for a function that either completes successfully or fails for a variety of reasons.

  you can think of unit as being like void in C. Type theorists will point out that unit is not like void, because unit has exactly 1 value whereas void has 0 values.

  You can store it in a variable, struct, collection, or anywhere else you could store a value. You can pass as an argument or return it as a result. You can create a reference to it. Etc.

  It's **'an expression based language'** ....

## Ownership

- The concept of ownership has nothing to do with 'property' in the Lockean sense of the word. Owners have no ability to restrict access...the "Owner" of a value simply means that it is responsible for destroying the value when the 'owner' itself has gone 'out of scope'.

**Borrowing**....anything can borrow any value at any point....it is 'read only'....

- & --> read only
- mut& --> has read/write accesss. only one mutable reference can exist for a value within a given scope.

## Datatypes...

- tuples are usually implemented behind the scenes. We don't see them directly...usually used as intermidiate placeholders. They're used for function return values. So you can also desructure tuples...
  - let's say you have a function that returns three values...
    let (x,y,z) = function_example();

## Const vs Let

- Compilier will make a copy of the 'Const' value wherever it is used and 'Let' value is placed on the stack inside the stack frame. It can optimize for 'Const' at compile time...const takes an 'assignment binding'.
- const is not for variables; it's for constant values which may not be stored anywhere; they're effectively an alias for a literal value.
- The type of a constant **must** be declared, whereas the type of a variable may be declared.
- A const does not represent a memory location but a value. const values are directly inlined at usage location
- Constants can only be set to a constant expression, not to the result of a function call or anything that could only be determined at runtime.
- constants may not be of a type that requires allocation on the heap, since they’re not known at compile time
- The naming convention for const is SCREAMING_SNAKE_CASE.
  The naming convention for let is snake_case.

- Let values represent a memory location. Immutability for let binding is a compiler enforced thing can be changed with mut modifier. It is runtime construct. Always locally scopped. Their types can be inferred by the compiler.
- Non-mut let declares an actual variable which is created at runtime, can be moved (and no longer accessible), and even have interior mutability (if it contains Cell members, for example)

## Result and Option

- These are both very helpful 'enums'
  - **Result** provides you as a library writer....the opportunity to be informed that something went wrong that can probobly be recovered from. It at least gives you the opportunity to recover before BLOWING UP! In i/o...almost every method will return a 'Result'. A wrapper around that provides an 'Ok' with the value and 'Error' that will be a custom error handling. Nearly all i/o will return something...even if it fails. I think the .unwrap()...will 'cascade up' the call stack to point of failure and let you handle it....(???)
  - **Options** are simular, but different. An 'optional' parameter for a function is something that may or may not be **present**, but that is different that something being **wrong**.
