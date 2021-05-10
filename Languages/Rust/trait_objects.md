### "Dynamic dispatch"....Trait Objects...???

- because the stack needs to know the size of parameters at compile time ...passing in a pointer to a vtable will suffice. &dyn someType, ..this is 'indirection'.
- there will only be one copy of the function, but it goes through this indeirection at runtime...where as static dispatch is much faster...but can bloat the binary size if there are too many copies made (monomorphization)
- You have trait objects when you have a pointer to a trait. Box, Arc, Rc.
- They allow for "dynamic polymorphism and heterogeneous uses of types". (???)
- "Trait objects" are **Rust's take on dynamic dispatch**.
  - dynamic dispatch is the process of selecting which implementation of a polymorphic operation (method or function) to call at run time.
    - polymorphism is the provision of a single interface to entities of different types or the use of a single symbol to represent multiple different types.
      - subtype polymorphism: subtype is a datatype that is related to another datatype.
      - If S is a subtype of T, the subtyping relation is often written **S <: T**, to mean that any term of type S can be safely used in a context where a term of type T is expected.
      - Due to the subtyping relation, a term may belong to more than one type. Subtyping is therefore a form of type polymorphism.
- Traits are abstract ways to define 'behavior'...this behavior can be shared with other types.
  - Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose. Implementing a trait on a type is similar to implementing regular methods.
- a 'trait object' is a trait that is wrapped in a Box, Arc or Rc...which simply means it is **placed on the heap**.
- Dynamic dispatch is calling these traits from the heap...using a 'virtual table'.
- The purpose of dynamic dispatch is to defer the selection of an appropriate implementation until the run time type of a parameter (or multiple parameters) is known.
- to call methods in the trait object you **go through a V-Table**...

  - vtable (dispatch table, virtual method table, etc):

  ### Box

  - The canonical way to create trait objects is Box<Trait>, but the majority of code can get away with &mut Trait, which also has dynamic dispatch but saves an allocation. If you absolutely need ownership then use a Box, but most use-cases can use an &Trait or &mut Trait.

  - For concerns with optimization..Even better is to avoid using a trait object all together.

  ### Static Dispatch..

  - used when the types are known at compile time....the stack needs to know exactly how much memory to allocate for the function parameters. If a trait is implemented for a given function...then the compilier can infer what type each will have and can make copies to accomodate that.

  - the compilier will generate copies of each type used for a declared generic. If a function is declared that 'impl AsStr' and on two diff instances it is passed an &str and String....two versions of the function will be created by compilier that handels each type.
  - this is called **Monomorphization**.
  - This is also why it is difficult to send compiled binaries as libraries. Because if the consumer of the library provides a function types that are differant than what has been given the compilier it will not work. The consumer of a library will need to compile the library within the context of their own code. Hence the need to ship --lib rather than --bin.
  - This also means that Rust binaries are bigger than others.
  - Why are they bigger than 'C' binaries
    - more staticly compiled code.
    - Monomorphization
    - More stuff from stdLib that gets compiled in
    - Rust builds in 'Debug' symbols...if you don't strip them from the Binary it will be larger.

### Object 'safe'

- Object safety.

  - A trait is object safe if all the methods defined in the trait have the following properties:

    - The return type **isn’t Self**.
    - There are **no generic type parameters**.

  - Trait objects must be object safe because once you’ve used a trait object, Rust no longer knows the concrete type that’s implementing that trait.

- "Non-dispatchale functions"
  A Trait can have some methods in it that are 'object safe' and some that are not. A trick is to use trait bounds on the methods you want to leave out of a vTable....by saying 'where Self: Sized'. This some how makes the whole triat object safe while still having non-safe functions within it. ????

- vTables are constructed from the 'union of all the supertraits'....????
- EVERY trait object has a 'drop' function included with it in the vTable. A pointer to the 'drop' function. It also has some other information on the concrete type needed for the allocator.
  - vTable for ANY trait object includes.
    1. pointer to the 'drop' function. (for concrete type).
    2. the methods for the Trait.
    3. Size and alignment...???

### Types that are not Sized....(known at compile time)

1. dyn Trait ===> (*mut data, *mut vTable)
2. [u8] ===> (\*mut data, usize length)
3. str ===> (\*mut data, usize length)
