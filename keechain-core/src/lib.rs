// Copyright (c) 2022-2023 Yuki Kishimoto
// Distributed under the MIT software license

#![doc = include_str!("../README.md")]

pub extern crate bdk;
pub extern crate bitcoin;

pub mod bips;
pub mod crypto;
pub mod slips;
pub mod types;
pub mod util;

pub use self::types::{KeeChain, Keychain};

pub type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;
