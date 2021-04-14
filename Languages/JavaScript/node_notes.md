### Notes about node.js that would be nice to remember, but I know I will forget!

- Every file executed in the node runtime is 'wrapped' inside an IIFE. Every file is 'treated as a module'.
  - This IIFE generates an object called 'arguments' that is globally available data that helps aid in building apps.
  - **arguments** has a length of 5.
    1.
