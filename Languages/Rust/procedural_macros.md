## Procedural Macros

Macros expand out at compile time to a complete implemention.

```
    To write out a macro you would do something like the fallowing...

            macro_rules! vec {
                ($($e:expr), *) => {{               First write pattern matching rules for the arguments to macro.
                    let mut v =  Vec::new();
                    $(v.push)$e);)*
                    v
                }}
            }

           From line 9.. Anything that is an expression seperated by a comma 0 or more times....
           expand the body out and execute what ever is in parenthesis (line 11) for each number of commas found in arguments....

    So for example..
            vec![1,2,3]; would expand out to roughly

            {
                let mut v = Vec::new();
                v.push(1);
                v.push(2);
                v.push(3);
                v
            }

```

- from the serde crate...let's say you need to do serialization and de-serialization.
  this is an example of a **'derive' macro**

```
    #[derive(Serilize, Deserialize)]
    struct foo {
        bar: usize,
    }

    this will generate something like...

    impl Serialize for foo {
        ...
    }
```

- An **attribute macro**

```
    The web framework Rocket uses attribute macros for routing...

        #[route(get, '/')]
        fn foo () {
            ...
        }

        foo will execute everytime a GET request comes in for '/'
```

- These are all **Procedural** macros....at compile time they will be given a stream of tokens and they will replace or add to that original stream. (TokenStream) -> TokenStream. I assume this happens in between the lexing state and the parsing for the AST.

- Use the crate 'cargo expand' <https://crates.io/crates/cargo-expand> to see macro expansions when compiled.
