### Types

- a type system is a logical system comprising a set of rules that assigns a property called a type to the various constructs of a computer program, such as variables, expressions, functions or modules.
  - The main purpose of a type system is to reduce possibilities for bugs in computer programs by defining interfaces between different parts of a computer program, and then checking that the parts have been connected in a consistent way.
  - checking can happen statically (at compile time), dynamically (at run time).
  - The more type restrictions that are imposed by the compiler, the more strongly typed a programming language is
- The hardware of a general purpose computer is unable to discriminate between for example a memory address and an instruction code, or between a character, an integer, or a floating-point number, because it makes no intrinsic distinction between any of the possible values that a sequence of bits might mean.

- Associating a sequence of bits with a type conveys that meaning to the programmable hardware to form a symbolic system composed of that hardware and some program.

#### Advantages provided by programmer-specified type systems.

1. Abstraction: Types enable programmers to think at a higher level than the bit or byte, not bothering with low-level implementation. - think of a string as a set of character values instead of as a mere array of bytes.
2. Documentation: In more expressive type systems, types can serve as a form of documentation clarifying the intent of the programmer.
