### Generics

- the compilier will generate copies of each type used for a declared generic. If a function is declared that 'impl AsStr' and on two diff instances it is passed an &str and String....two versions of the function will be created by compilier that handels each type.
  - this is called **Monomorphization**.
  - This is also why it is difficult to send compiled binaries as libraries. Because if the consumer of the library provides a function types that are differant than what has been given the compilier it will not work. The consumer of a library will need to compile the library within the context of their own code. Hence the need to ship --lib rather than --bin.
