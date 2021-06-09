## Cell

#### This type allows for 'Interior Mutability'.

- the `std::cell::Cell` type allows you to modify a value through a shared reference `&`. Becauase

1. No other threads are allowed to have reference to it. **single threaded only**.
   - we make sure Cell has the !Sync trait...This tells the compiler to NEVER allow more than one thread to access the Cell.
2. No reference to the value **inside** the Cell was given out. So the `get` method for Cell returns a 'copy' of the inner value. We can change the inner value **because**

## RefCell

- lets you check during **run time** whether anyone else is mutating...'dynamically checked borrows'.
  - good for graphs and trees.
  ```rs
  pub struct RefCell<T> {
      value: Unsafecell<T>,
      reference: isize,
  }
  impl RefCell {
      pub fn new(value: T) -> Self {
          Self{
              value: UnsafeCell::new(value),
              reference: 0,
          }
      }
  }
  ```
