#### closures

From a stackOverflow post. [here](https://stackoverflow.com/questions/45935100/how-do-rust-closures-work-and-how-does-it-execute-a-closure)

---

- closures can capture values from the **scope** in which they’re **defined**.
- in a closure you can use the variables defined outside the closure body but accessible in its scope.

```
fn main() {
    let a = 6;
    let closure = |b| {
        println!("product is: {}", a * b);
    };
    closure(7);
}

```

will essentially de-sugar into..

```
fn main() {
    let a = 6;
    let closure = {
        struct Closure<'a> {
            a: &'a i32,
        }
        impl<'a> Fn<(i32,)> for Closure<'a> {
            extern "rust-call" fn call(&self, (b,): (i32,)) {
                println!("product is: {}", (*self.a) * b);
            }
        }
        impl<'a> FnMut<(i32,)> for Closure<'a> {
            extern "rust-call" fn call_mut(&mut self, args: (i32,)) {
                self.call(args)
            }
        }
        impl<'a> FnOnce<(i32,)> for Closure<'a> {
            type Output = ();
            extern "rust-call" fn call_once(self, args: (i32,)) {
                self.call(args)
            }
        }
        Closure {
            a: &a,
        }
    };
    FnOnce::call_once(closure, (7,));
}


```

...nothing magical is happening. They boil down to a regular function call with an extra initial "context" argument.

---

- closure traits...Fn(), MutFn() and fnOnce(). Are special traits. ???

---

The 'move' keyword....closures are cleaver in that they capture the variables in their scope by reference (&...compilier converts it. everytime? idk)...
in a situation like multi threads...Channels...we have a 'Reciever' that literally needs to be passed between all of the theads. wrapping the reciever in a Mutex is not enouph. What is really going on is that the closure is capturing &Mutex\<reciever>. The move keyword is reposible for literally moving ownership into that thread. Something like that...???

from 'Rust Reference'..."Without the move keyword, the closure expression infers how it captures each variable from its environment, **preferring to capture by shared reference**, effectively borrowing all outer variables mentioned inside the closure's body."

- common pattern seen a lot in Rust is the 'Arc::new(Mutex::new(thing))'.
  - Mutex for exclusive access.
  - Arc for having multiple owners safely.

from [Guide to Rustc Development](https://rustc-dev-guide.rust-lang.org/closure.html)

- Closures in Rust are effectively "desugared" into structs that contain the values they use (or references to the values they use) from their creator's stack frame.
- has to figure out which of the closure traits (Fn, FnMut, or FnOnce) a closure is capable of implementing.
