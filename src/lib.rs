use std::ops::Deref;

/// A portal network protocol identifier.
pub struct ProtocolId([u8; 2]);

/// A portal network protocol.
pub enum Protocol {
    /// Execution history.
    History,
    /// Execution state.
    State,
}

impl Deref for ProtocolId {
    type Target = [u8; 2];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Protocol {
    /// Returns the protocol identifier.
    pub fn id(&self) -> ProtocolId {
        match self {
            Self::State => ProtocolId([0x50, 0x0a]),
            Self::History => ProtocolId([0x50, 0x0b]),
        }
    }
}

/// A portal network content identifier.
///
/// A `ContentId` is the lookup key for a content item on a portal network protocol. The value is
/// derived from a `ContentKey`.
pub struct ContentId([u8; 32]);

impl Deref for ContentId {
    type Target = [u8; 32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A portal network content key.
///
/// A `ContentKey` uniquely identifies a content item on a portal network protocol. Each
/// `ContentKey` defines a derivation function to map to a fixed-size `ContentId` which is the key
/// in the portal network DHT key-space. Each `ContentKey` defines an encoding so that keys can be
/// transmitted over the wire.
pub trait ContentKey<E: std::error::Error>: Sized {
    /// Returns the `ContentId` for the `ContentKey`.
    fn id(&self) -> ContentId;
    /// Returns the encoded `ContentKey`.
    fn encode(&self) -> Vec<u8>;
    /// Returns the successfully decoded `ContentKey`, or an error `E` upon failure.
    fn decode(value: &[u8]) -> Result<Self, E>;
}
