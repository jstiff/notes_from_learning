### JSON Web Tokens.

- Tokens are coprised of three parts....

  1. Header
  2. Body (payload)
  3. Signature

- generating a 'Header'....which is json data about the user needs to be encoded in 'base64Url' to more effectively ship over the wire. There would be a lot of characters within the JSON object that might cause problems.
  - the payload will also need to be 'encoded' and then these will be packaged along with a 'secret' and a hashing alogorithm type. These base64 encodings are not for security purposes...that will be added later in the process.
