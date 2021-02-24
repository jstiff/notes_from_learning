### Trait Objects...???

- You have trait objects when you have a pointer to a trait. Box, Arc, Rc.
- They allow for "dynamic polymorphism and heterogeneous uses of types". (???)
- "Trait objects" are Rust's take on dynamic dispatch.
  - dynamic dispatch is the process of selecting which implementation of a polymorphic operation (method or function) to call at run time.
    - polymorphism is the provision of a single interface to entities of different types or the use of a single symbol to represent multiple different types.
      - subtype polymorphism: subtype is a datatype that is related to another datatype.
      - If S is a subtype of T, the subtyping relation is often written **S <: T**, to mean that any term of type S can be safely used in a context where a term of type T is expected.
      - Due to the subtyping relation, a term may belong to more than one type. Subtyping is therefore a form of type polymorphism.
      - ex) Bird<T> ... with subtypes duck<a>, ostrich<b>, cukoo<c>
- Traits are abstract ways to define 'behavior'...this behavior can be shared with other types.
  - Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose. Implementing a trait on a type is similar to implementing regular methods.
- a 'trait object' is a trait that is wrapped in a Box, Arc or Rc...which simply means it is **placed on the heap**.
- Dynamic dispatch is calling these traits from the heap...using a 'virtual table'.
- The purpose of dynamic dispatch is to defer the selection of an appropriate implementation until the run time type of a parameter (or multiple parameters) is known.
