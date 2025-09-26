//! Network-related primitives for consensus.

use crate::Hash;

/// Network identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkId {
    Mainnet,
    Testnet,
    Devnet,
    Simnet,
}

impl NetworkId {
    /// Returns the magic bytes for the network.
    pub fn magic(&self) -> [u8; 4] {
        match self {
            NetworkId::Mainnet => [0xAB, 0xCD, 0xEF, 0x12],
            NetworkId::Testnet => [0xBA, 0xDC, 0xFE, 0x21],
            NetworkId::Devnet => [0xCA, 0xED, 0xFA, 0x31],
            NetworkId::Simnet => [0xDA, 0xEC, 0xFB, 0x41],
        }
    }
}

/// Peer address representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeerAddress {
    pub ip: std::net::IpAddr,
    pub port: u16,
}

impl PeerAddress {
    pub fn new(ip: std::net::IpAddr, port: u16) -> Self {
        Self { ip, port }
    }
}

/// Contextual network address (stub).
#[derive(Debug, Clone, Default)]
pub struct ContextualNetAddress {
    pub address: String,
}

impl ContextualNetAddress {
    pub fn unspecified() -> Self {
        Self { address: "0.0.0.0".to_string() }
    }
}

impl std::str::FromStr for ContextualNetAddress {
    type Err = std::net::AddrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Stub: just store the string
        Ok(Self { address: s.to_string() })
    }
}

/// Network address (stub).
#[derive(Debug, Clone, Default)]
pub struct NetAddress {
    pub address: String,
}

impl std::str::FromStr for NetAddress {
    type Err = std::net::AddrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Stub: just store the string
        Ok(Self { address: s.to_string() })
    }
}

/// Network message types.
#[derive(Debug, Clone)]
pub enum NetworkMessage {
    Ping,
    Pong,
    GetBlocks { hashes: Vec<Hash> },
    Blocks { blocks: Vec<Hash> }, // Placeholder
    Inv { hashes: Vec<Hash> },
    GetData { hashes: Vec<Hash> },
    Tx { transaction: Hash }, // Placeholder
}

/// Default network ID.
pub const DEFAULT_NETWORK: NetworkId = NetworkId::Mainnet;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_magic() {
        assert_eq!(NetworkId::Mainnet.magic(), [0xAB, 0xCD, 0xEF, 0x12]);
    }

    #[test]
    fn test_peer_address() {
        let addr = PeerAddress::new("127.0.0.1".parse().unwrap(), 8333);
        assert_eq!(addr.port, 8333);
    }
}
