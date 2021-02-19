### Warp

- there are a few patterns that are common when using Hyper.
  - the main trait is called 'Service'
    - The Service trait is a simplified interface making it easy to write network applications in a modular and reusable way, decoupled from the underlying protocol. It is one of Tower's fundamental abstractions.
  - A new Response will by default have a 200 OK status code, and the Body is able to tell that it is made from a static string, and is able to add a Content-Length header for us automatically.
