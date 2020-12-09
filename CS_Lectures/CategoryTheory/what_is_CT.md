## What is Category Theory?

#### Tools we use.

- Abstraction (hide the details)
- Composition
- Identity

We want to forget about the machine code and all the unnecessary details. At some point we need to un-learn
imperative programing.

imperative --> procedural programming --> object Oriented programming --> Functional programming

Once 'Concurrent software' started to be written it had become clear that OOP was not up to the challenge.
Objects hide or abstract away implementation details, but the hide the exact wrong things when it comes to concurrentcy. This makes them NOT composable.

Objects hide 2 important things.

1. Mutations (They mutate state inside the objects).
2. Share Pointers ( They share data between each other).

These two properties lend themselves to **'Data Races'**...

The Java language provides a 'lock' for each object.

When you are abstracting away implementation details you need to be careful to understand exactly what it is you are 'subtracting' away and what consequences will potentially fallow such decisions.
