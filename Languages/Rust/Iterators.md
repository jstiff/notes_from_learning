### Iterators

for loops will de-sugar into this .....

```rs

let values = vec![1, 2, 3, 4, 5];
{
    let result = match IntoIterator::into_iter(values) {
        mut iter => loop {
            let next;
            match iter.next() {
                Some(val) => next = val,
                None => break,
            };
            let x = next;
            let () = { println!("{}", x); };
        },
    };
    result
}
```

- std iterators return &references to the data it's interating over. NOT the data itself. (read-only)
