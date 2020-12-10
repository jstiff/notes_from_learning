## What is Category Theory?

- Lectures by Bartosz Milewski.

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

**Haskell** is based on Category Theory.

Category Theory is a higher level language ABOVE Haskell. It's not practical and cannot be written on a computer.
The goal is to get to the highest level of abstraction to then later translate it into a programing language.
From the highest level of Category theory....all programming langauges start to look the same...and even in Mathmatics...with all of it's diff branches and theories from the Catergorical perspective...the 'structure' of each of the domains of Mathmatics appeard to be the same or exhibit simularities.

You tweak the structure of a category and you suddenly get 'topology' or 'set theory' or 'algebra'...etc
There seems to be a Grand unification of all mathmatics...
Computational systems....cpu's, registers...etc can be described Mathmatically with Lambda Calculus.

Then there is 'Type theory' that describes all of the data structures we have like arrays, etc. There is also Type theory in Math...that attemps to solve parodoxes like 'you cannot construct a set of all sets'...

People realized that all of the distinct areas of Mathmatics are exactly the same. This is refered to as the
'Curry–Howard correspondence (also known as the Curry–Howard isomorphism)'. Essentially says that whatever you do in logic can be mapped to what you do in Type theory. It's not that they are simular...it's that they are **exactly the same**.
Computing -- Categories -- Types ....are all equivalent.

- All of our Human activity can be described by this one theory. As human creatures we evolved to solve problems by breaking up complexity into smaller managable problem sets and then bringing everything together. The way of doing things permeates so much of our existence that we do not notice it. This is how we do science and mathmatics...chop stuff into smaller pieces. From the higher vantage point all of these pieces are from the same structure.

Is the Entire universe choppable???? That is a debate. What is 'the elementary particle'?....a ball? ...that can be chopped further...a point? ... there are complications with infities when dealing with 'points'.....how about 'strings'? A string that is not divisable...it's not a point....Then Quantum theory says...take a group of particles as an object....the particles don't add up...they simply change the 'state'.???

Maybe it's just that our minds NEED to see structure everywhere or else it will give up...In this way Category theory is not about mathmatics...but more about our **minds**.
