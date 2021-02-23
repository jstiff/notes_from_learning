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
