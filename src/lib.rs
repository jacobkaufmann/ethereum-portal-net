use std::ops::Deref;

/// A portal network (sub)protocol identifier.
pub struct ProtocolId([u8; 2]);

/// A portal network (sub)protocol.
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
