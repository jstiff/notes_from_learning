### Rust

- this language seems to be a nice blending of low level and high level programming. Great type system and can write in Functional Programming style. Or low level C style systems...

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

---

### i64

- this type is easily copied on the hardware so it can be accessed on stack with no problem..the ownership is acheived cheaply due to this copying......while the 'String' type is on the heap....there is no copy of it...it would be a copy of a pointer to the string on the heap...meaning there are immediate ownership issues. would have to 'clone()' it.
