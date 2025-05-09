#![cfg_attr(feature = "no_std", no_std)]

// Common features for both master device and slave device
mod common;
pub use common::*;

/// Slave device relative features
#[cfg(feature = "slave")]
pub mod slave;
