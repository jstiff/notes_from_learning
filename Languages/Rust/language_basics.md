### Rust

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
