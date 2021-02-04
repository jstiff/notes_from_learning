## Walk through a libp2p repository example

- [Link to the libp2p repo](https://github.com/libp2p/rust-libp2p)

#### examples/chat.rs

- uses module called **'identity'**.
- **Keypair** is an enum which represents the 'identiry keypair of a node'.

#### let local_key = identity::Keypair::generate_ed25519();

```
pub enum Keypair {
    Ed25519(Keypair),
    Rsa(Keypair),
    Secp256k1(Keypair),
}

pub fn generate_ed25519() -> Keypair {
        Keypair::Ed25519(ed25519::Keypair::generate())
    }

```

#### let local_peer_id = PeerId::from(local_key.public());

```
PeerId is a struct

pub struct PeerId {
    multihash: Multihash,
}

** Not sure this is correct! the "from" might be from core::convert::from

impl From<PublicKey> for PeerId {
    fn from(key: PublicKey) -> PeerId {
        PeerId::from_public_key(key)
    }
}

pub fn public(&self) -> PublicKey {
        use Keypair::*;
        match self {
            Ed25519(pair) => PublicKey::Ed25519(pair.public()),
            #[cfg(not(target_arch = "wasm32"))]
            Rsa(pair) => PublicKey::Rsa(pair.public()),
            #[cfg(feature = "secp256k1")]
            Secp256k1(pair) => PublicKey::Secp256k1(pair.public().clone()),
        }
    }
```

#### let transport = libp2p::build_development_transport(local_key)?;

- Set up a an encrypted DNS-enabled TCP Transport over the Mplex and Yamux protocols.

```
pub fn build_development_transport(keypair: identity::Keypair)
    -> std::io::Result<core::transport::Boxed<(PeerId, core::muxing::StreamMuxerBox)>>
{
     build_tcp_ws_noise_mplex_yamux(keypair)
}


pub fn build_tcp_ws_noise_mplex_yamux(keypair: identity::Keypair)
    -> std::io::Result<core::transport::Boxed<(PeerId, core::muxing::StreamMuxerBox)>>
{
    let transport = {
        #[cfg(feature = "tcp-async-io")]
        let tcp = tcp::TcpConfig::new().nodelay(true);
        #[cfg(feature = "tcp-tokio")]
        let tcp = tcp::TokioTcpConfig::new().nodelay(true);
        let transport = dns::DnsConfig::new(tcp)?;
        let trans_clone = transport.clone();
        transport.or_transport(websocket::WsConfig::new(trans_clone))
    };

```

```
#### struct MyBehaviour {

        floodsub: Floodsub,
        mdns: Mdns,

        // Struct fields which do not implement NetworkBehaviour need to be ignored
        #[behaviour(ignore)]
        #[allow(dead_code)]
        ignored_member: bool,
    }

```

mDns

pub struct Mdns {
/// The inner service.
service: MdnsBusyWrapper,

    /// List of nodes that we have discovered, the address, and when their TTL expires.
    ///
    /// Each combination of `PeerId` and `Multiaddr` can only appear once, but the same `PeerId`
    /// can appear multiple times.
    discovered_nodes: SmallVec<[(PeerId, Multiaddr, Instant); 8]>,

    /// Future that fires when the TTL of at least one node in `discovered_nodes` expires.
    ///
    /// `None` if `discovered_nodes` is empty.
    closest_expiration: Option<Timer>,

}

enum MdnsBusyWrapper {
Free(MdnsService),
Busy(Pin<Box<dyn Future<Output = (MdnsService, MdnsPacket)> + Send>>),
Poisoned,
}

```

```
