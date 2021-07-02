### Lifetime Annotations.

- The first thing one needs to realize about lifetimes is that they are all about references, and nothing else.

- at first glance it seems that lifetimes are ways of telling the compilier that you want to extend a reference to a piece of data that will not live long enough for that particular reference. ??? This might be wrong though. **Lifetime annotations don’t change how long any of the references involved live. In the same way that functions can accept any type when the signature specifies a generic type parameter, functions can accept references with any lifetime when the signature specifies a generic lifetime parameter. What lifetime annotations do is relate the lifetimes of multiple references to each other**.

  - By specifying the lifetime parameters for say, a function signature, we are **not changing the lifetimes of any values** passed in or returned, but we are saying that any values that do not adhere to this **contract should be rejected by the borrow checker**.

- **every reference** in Rust has a lifetime, which is the scope for which that reference is valid. Most of the time lifetimes are implicit and inferred, just like most of the time types are inferred.

- The main aim of lifetimes is to **prevent dangling references**.

[paper](http://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch10-03-lifetime-syntax.html)

```
This errors...
{
    let r;                // -------+-- 'a
                          //        |
    {                     //        |
        let x = 5;        // -+-----+-- 'b
        r = &x;           //  |     |
    }                     // -+     |
                          //        |
    println!("r: {}", r); //        |
}                         // -------+

This compiles...


{
    let x = 5;            // -----+-- 'b
                          //      |
    let r = &x;           // --+--+-- 'a
                          //   |  |
    println!("r: {}", r); //   |  |
                          // --+  |
}                         // -----+


```

---

[notes from this blog](https://blog.logrocket.com/understanding-lifetimes-in-rust/)
lifetimes are commonly used for...

1. when returning references from functions and when creating structs with references.

- When writing functions that accept references as arguments, the compiler can infer the correct lifetimes in many cases, saving you the trouble of writing them out by hand. When lifetime annotations are implicit, we call this **lifetime elision**.
  - three rules to figure out whether lifetime annotations can be elided or not.
    1. The function doesn’t return a reference
    2. There is exactly one reference input parameter
    3. The function is a method, taking &self or &mut self as the first parameter
- When a function accepts multiple references, they’re each given their own
  lifetime

#### structs

- there are certain cases where structs with references are exactly what you want — in particular, if you want to create a view into something else. Using structs with references is a great way to organize some data into a package that’s easier to handle without moving or copying data. This means that the original data source can still be referenced elsewhere and you’re spared the hassle of cloning the data.

#### Lifetime Elision

- The reason this function compiles without lifetime annotations is historical: in early versions of pre-1.0 Rust, this indeed wouldn’t have compiled. Every reference needed an explicit lifetime. At that time, the function signature would have been written like this:

```
fn first_word<'a>(s: &'a str) -> &'a str {

```

- After writing a lot of Rust code, the Rust team found that Rust programmers were typing the same lifetime annotations over and over in particular situations. These situations were predictable and followed a few deterministic patterns.The Rust team then programmed these patterns into the Rust compiler’s code so that the borrow checker can infer the lifetimes in these situations without forcing the programmer to explicitly add the annotations.

#### lifetime elision rules

- Lifetimes on function or method parameters are called **input lifetimes**, and lifetimes on return values are called **output lifetimes**.

  - rules that the compiler uses: (The first rule applies to input lifetimes, and the second two rules apply to output lifetimes)

    1. Each parameter that is a reference gets its own lifetime parameter. In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32), a function with two arguments gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32), and so on.

    2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.

    3. If there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, then the lifetime of self is assigned to all output lifetime parameters. This makes writing methods much nicer.

- If the compiler gets to the end of the three rules and there are still references that it can’t figure out lifetimes for, the compiler will stop with an error.

#### Dangling Pointers...

- this occurs when you have a pointer that refers to an allocated section of memory on the heap...you use that memory for whatever purposes...are done with it and free it before exiting the scope of that code. If later in the code you try to use that same pointer(which is just a vaiable with the address) it will point to a location on the heap that is either empty or has been reassigned to a differnt value. You ran free() to tell the operating system to clean up that memory address, but you **did not destroy the value inside the pointer variable**...the pointer variable is still stored in the processes execution space with the number of the old memory location.

```

int someFunc () {
    int myPtr* = malloc(10);
    ...run computation...
    free(myPtr);
return someValue;
}

let anotherValue = &myPtr;                     // 'anotherValue' is trying to de-allocated the contents of a
                                               // memory location that has already been freed!!!


```
