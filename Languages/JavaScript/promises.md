## Promises

- Promises represent an 'idea' that is implemented in Js code.
- This idea has become a 'standard' which everyone agrees on how and why this should be implemented.
- It represents a future value which we may not have at the moment which is returned after some work we need has been completed elsewhere.

```
   This is basically pseudo-code...based on Tony Alicea's video on Promises.

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

       function resolve(result){
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

[Link to Tony Alicea Promises video](https://www.youtube.com/watch?v=fyGSyqEX2dw)

- Creating a new Promise will immedietly create the Promise object and fire off the executor() function which in turn passes in the resolve and reject functions that were just declared within that scope (Defined by spec).
- By calling the executer immedietley ...the executor function 'we' declared will be stuffed with the 'resolve/reject' functions that come with a 'Promise' impl...we also determine the 'result' value that will be passed into that resolve function. It can be the return value of an API call for example.

Once the result comes in (if not Error)...then resolve function updates the state to 'FULLFILLED' and sets the returned result to the Promise states 'value'. And with that value it will pass it on to the 'then' handler functions declared within the Promise. If there are multpiple handlers in the array they will be executed sequentially. I believe each handler also returns a Promise with the new 'value' (???? could be wrong...)

This means that the executor function will have closure over the state of the Promise.(??). I guess because a function was declared within another function the js Engine will have to allocate memory on the heap for the relevent state that was declared in the Promomise function. The garbadge collector will have to keep track of what has a pointer to the heap and destroy it when necessary. GC must have a ref counter to Promise.

- Chaining '.then' to a Promise may look like they are chained to the original Promise, but that is not the case. If a second .then is declared it is attached to the Promise that its predecessor returned. (??) The Js Promise implementation wraps all results into a new Promise to aid in this chaining feature.

- **IF** you created a new Promise inside a .then handler(which automatically returns a Promise) it will be syncronized with the original .then and still work. (?????) Look more into why!

There are diff flavors of implementing Promises of diff 'recipes'.

1. Parallel Promises
2. Serial Promises
3. Promise.resolve
4. Promise.race
5. Await Promise.all
