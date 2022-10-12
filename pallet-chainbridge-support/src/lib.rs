#![cfg_attr(not(feature = "std"), no_std)]

pub mod traits;

pub type ChainId = u8;
pub type DepositNonce = u64;
pub type ResourceId = [u8; 32];
pub type EthAddress = [u8; 20];
