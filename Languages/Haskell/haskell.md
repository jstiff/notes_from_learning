## Haskell Language...

- Haskell describes functions or values defined in a typeclass as 'methods', just as traits describe OOP methods in the objects they enclose. However, Haskell deals with these differently, treating them as individual values rather than pinning them to an object as OOP would lead one to do. This is about the most obvious surface-level difference there is.

The one thing that Rust could not do for a while was higher-order typed traits, such as the infamous Functor and Monad typeclasses.

This means that Rust traits could only describe what's often called a 'concrete type', in other words, one without a generic argument. Haskell from the start could make higher-order typeclasses which use types similar to how higher-order functions use other functions: using one to describe another. For a period of time this was not possible in Rust, but since associated items have been implemented, such traits have become commonplace and idiomatic.

So if we ignore extensions, they are not exactly the same, but each can approximate what the other can do.

It is also mentionable, as said in the comments, that GHC (Haskell's principal compiler) supports further options for typeclasses, including multi-parameter (i.e. many types involved) typeclasses, and functional dependencies, a lovely option that allows for type-level computations, and leads on to type families. To my knowledge, Rust has neither funDeps or type families, though it may in the future.†

All in all, traits and typeclasses have fundamental differences, which due to the way they interact, make them act and seem quite similar in the end.
