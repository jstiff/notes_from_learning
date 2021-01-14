### Optimizer phase of a compiler.

- Ideally, this step should be machine independent. (Machine dependent code done in 'backend' phase).
  It is only concerened with the code. Tries to optimize the code in an hardware/machine 'independent' way.
  This is harder said than done.

An example of an optimization is 'Common Subexpression Elimination' (CSE). Finding repetative computations in the code...'reusing' coputational values where possible. To 'reuse' a value means it must be stored somewhere. There needs to be enough memory and it must be fast enough to make it worth while.

This implies that if the machine hardware is not capable of storing a CSE effectivly then it might make more sense to simple re-compute on that machine.

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

- Also this optimization is an example of how this is not 'truely' independent from the hardware.
  It uses registers and caches to implement the optimization which could be effected by diff architectures.
