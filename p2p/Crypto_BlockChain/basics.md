## Some terminology to get my feet wet..

#### Digital fingerprints.

- 'encryption' is the process of 'scrambling up a message in a predefined way beofre sending it off to a destination...the destination will need to know the same process to 'unscramble' the message to understand it.
  - The classic example is the 'Ceasar Cipher'...which is a 'symetric encryption'method.
    - problem with this is that two parties need to meet in private to agree on the key. Let's say Generals in a war will all agree on the key type and then will be able to pass around enrypted messages around without fear that the enemy will be able to read it. (unless the enemy had the same key or cipher.
  - Computers cannot do this. They use 'Asymetric encryption'. Based on a Public and Private key.
    - Public key is used to encrypt data and anyone can use it. But can only be decrypted to a computer who has access to the Private key. With a public key you can deliver messages, but not read them. I can give out my public key and many people can message me. But I will need to use my private key to read them.
- Security Protocols
  - SSL and TLS
  - Browsers use these to protect out data...HTTPS.
  - They use public key encryption to exchange data with the web site you are currently on.
- 'Asymmetric cryptography'....meaning it is comprised of generating a 'public hash key' and a 'private hash'. These two hash values are mathmatically related(derived). I mean they are computed together in order to provide another layer of cryptography. If someone steals your public key they will still need your private key to accomplish anything.
  - algorithmns that generate these public/private keys at the same time based on a 'randomly generated number'.
    - needs a very good way to generate **random number** for this computation.
    - some wallets that are easily hacked are due to poor/insecure random number generation.
- example...
  - I want to communicate with bob...
    1. I generate a public/private key pair.
    2. I send my public key to a server.
    3. Notifiy bob where he can get my public key.
    4. bob writes a message and uses my public key to encrypt it.
    5. bob sends the message back to me.
    6. I use my private key to decrypt the message.
  - in order to know that bob was really bob we need a...
    - 'Digital signature'
      1. Auth: this signature gives reciever reason to believe it is bob.
      2. Non-repudiation: seder cannot deny having sent the message later on.
      3. Interity: inusre that the message was not altered in transit.

---

- Bitcoin takes the Public key and hashes it twice...
  1. SHA256
     then into
  2. RIPEMD-160
- The 'Transaction format'
  - Has an **input** and a **output**
  - Input: Previous transaction ID
    Index (of merkle tree maybe???)
    Signature
  - Output: Value
    Public Key (bitcoin address)
  - the Previous transaction id basically uniquely itentifies money.
  - you send value through 'satoshis'...each coin has 10\*8 satoshis inside of it. ???

---

## Consensus

- every blockchain has it's own currency. Ethereum is 'Ether'...etc. What does this mean? how will the world agree on a single currency? I doubt there will be.

### Hard Fork!!

- right now the social consensus seems to be all in for Bitcoin...but once in awhile there is a 'hard fork' of the blockchain....'Bitcoin Cash' that maintains or matches the level of social consensus that the original currency has making it a legitamite store of value.



