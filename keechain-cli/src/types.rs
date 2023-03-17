// Copyright (c) 2022-2023 Yuki Kishimoto
// Distributed under the MIT software license

use clap::ValueEnum;
use keechain_core::bitcoin::Network;
use keechain_core::types::{ElectrumSupportedScripts, WordCount};

#[derive(Debug, Clone, ValueEnum)]
pub enum CliNetwork {
    Bitcoin,
    Testnet,
    Signet,
    Regtest,
}

impl From<CliNetwork> for Network {
    fn from(value: CliNetwork) -> Self {
        match value {
            CliNetwork::Bitcoin => Self::Bitcoin,
            CliNetwork::Testnet => Self::Testnet,
            CliNetwork::Signet => Self::Signet,
            CliNetwork::Regtest => Self::Regtest,
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum CliElectrumSupportedScripts {
    Legacy,
    Segwit,
    NativeSegwit,
}

impl From<CliElectrumSupportedScripts> for ElectrumSupportedScripts {
    fn from(value: CliElectrumSupportedScripts) -> Self {
        match value {
            CliElectrumSupportedScripts::Legacy => Self::Legacy,
            CliElectrumSupportedScripts::Segwit => Self::Segwit,
            CliElectrumSupportedScripts::NativeSegwit => Self::NativeSegwit,
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum CliWordCount {
    #[clap(name = "12")]
    W12,
    #[clap(name = "18")]
    W18,
    #[clap(name = "24")]
    W24,
}

impl From<CliWordCount> for WordCount {
    fn from(value: CliWordCount) -> Self {
        match value {
            CliWordCount::W12 => Self::W12,
            CliWordCount::W18 => Self::W18,
            CliWordCount::W24 => Self::W24,
        }
    }
}
