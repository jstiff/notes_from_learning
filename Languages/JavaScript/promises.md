## Promises

- Promises represent an 'idea' that is implemented in Js code.
- This idea has become a 'standard' which everyone agrees on how and why this should be implemented.
- It represents a future value which we may not have at the moment which is returned after some work we need has been completed elsewhere.

```
   This is pseudo-code...

// a promise is always in one of three States.

   const PENDING = 0;
   const FULLFILLED = 1;
   const REJECTED = 2;

   // create a Promise Object...pass in an executor function. This is the function that will do the work like
   // make API calls or DB query or simply Set Timeout. We are wrapping the async function in this 'Promise'.

   function Promise (executor) {
       let state = PENDING;
       let value = null;
       let handlers = [];              //array of handler functions
       let catches = [];

       function reslove(result){
           if (state !== PENDING) return;
           state = FULLFILLED;
           value = result;

           handlers.forEach((h) => h(value))
       }

       function reject(err) {
           if (state !== PENDING) return;
           state = REJECTED;
           value = err;

           catches.forEach((e) => e(err));
       }

       this.then = function(callback){
           if(state == FULLFILLED){
               callback(value);
           } else{
               handlers.push(callback)
           }
       }

       executor(resolve, reject)
   }

   When we make a 'new Promise' the executor function will be called right away with the resolve and reject functions passed into it.
```
