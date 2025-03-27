#![cfg_attr(feature = "slave", no_std)]

pub mod common;

#[cfg(feature = "slave")]
pub mod slave;