### Optimizer phase of a compilier.

- Ideally, this step should be machine independent. (Machine dependent code done in 'backend' phase).

An example of an optimization is 'Common Subexpression Elimination' (CSE).

```
     x = a + b + c;
         -----



     y = d / a + b;
             -----

     tmp = a + b;
     x = tmp + c;
     y = d/tmp;
```

Becuase a + b is a common Subexpression is can be put into a tmp. - It needs to be put into the cache or in a register otherwise it is not an optimization.
It's NOT a guaranteed that it will improve the code. If tmp is taking up resources when another
chunk of code would do better then that optimization needs to be re-thought.
