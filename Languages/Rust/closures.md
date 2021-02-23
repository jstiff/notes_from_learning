#### closures

From a stackOverflow post. [here](https://stackoverflow.com/questions/45935100/how-do-rust-closures-work-and-how-does-it-execute-a-closure)

---

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
