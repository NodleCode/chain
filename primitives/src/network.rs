#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::RuntimeDebug;

/// Network type for parallel.
#[derive(Clone, Copy, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum NetworkType {
    Parallel,
    Heiko,
}

impl NetworkType {
    /// Return ss58 address prefix from network type.
    pub fn ss58_addr_format_id(&self) -> u8 {
        match self {
            NetworkType::Heiko => HEIKO_PREFIX,
            NetworkType::Parallel => PARALLEL_PREFIX,
        }
    }
}

pub const HEIKO_PREFIX: u8 = 110;
pub const PARALLEL_PREFIX: u8 = 172;
