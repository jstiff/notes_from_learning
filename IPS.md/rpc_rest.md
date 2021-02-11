#### RPC vs REST

- these are two diff ways of building an API **over HTTP** ...which means their are simularities at the 'transfer' protocol layer which confuses people.
  - An RPC endpoint is useful for working with a narrow view of the data. This reduces the bandwidth you use on the network and simplifies the service. You can be as specific or as narrow as you want as long as it does what you need. What is nice about RPC is that you have a way of creating services that do one job well.
  - A REST endpoint treats the request like making a call to a resource. The resource becomes the domain that has the data. The resource does not concern itself with functionality at all. It is only a place where you contain data and do with it as you see fit.
- RPC is more procedure based while REST is resource based (oriented!)
  - RPC url will usually have the procedure it wants executed in it...communicating the intent right in the url.
