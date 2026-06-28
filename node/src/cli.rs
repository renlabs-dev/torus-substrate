// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use polkadot_sdk::{sc_cli::RunCmd, *};

pub mod eth;

#[derive(Clone, Copy, Debug, Default)]
pub enum Consensus {
    #[default]
    Aura,
    ManualSeal(u64),
    InstantSeal,
}

impl std::str::FromStr for Consensus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s == "aura" {
            Consensus::Aura
        } else if s == "instant-seal" {
            Consensus::InstantSeal
        } else if let Some(block_time) = s.strip_prefix("manual-seal-") {
            Consensus::ManualSeal(block_time.parse().map_err(|_| "invalid block time")?)
        } else {
            return Err("incorrect consensus identifier".into());
        })
    }
}

#[derive(Debug, clap::Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Option<Subcommand>,

    #[arg(long, default_value = "aura")]
    pub consensus: Consensus,

    /// Keep session keys only in memory.
    ///
    /// This disables the on-disk keystore entirely. Keys must be injected at
    /// runtime through RPC and are lost on restart.
    #[arg(
        long,
        conflicts_with_all = [
            "alice",
            "bob",
            "charlie",
            "dave",
            "dev",
            "eve",
            "ferdie",
            "keystore_path",
            "one",
            "password",
            "password_filename",
            "password_interactive",
            "two"
        ]
    )]
    pub keystore_in_memory: bool,

    #[clap(flatten)]
    pub run: RunCmd,

    #[command(flatten)]
    pub eth: eth::EthConfiguration,
}

#[derive(Debug, clap::Subcommand)]
#[allow(clippy::large_enum_variant)]
pub enum Subcommand {
    /// Key management cli utilities
    #[command(subcommand)]
    Key(sc_cli::KeySubcommand),

    /// Build a chain specification.
    BuildSpec(sc_cli::BuildSpecCmd),

    /// Validate blocks.
    CheckBlock(sc_cli::CheckBlockCmd),

    /// Export blocks.
    ExportBlocks(sc_cli::ExportBlocksCmd),

    /// Export the state of a given block into a chain spec.
    ExportState(sc_cli::ExportStateCmd),

    /// Import blocks.
    ImportBlocks(sc_cli::ImportBlocksCmd),

    /// Remove the whole chain.
    PurgeChain(sc_cli::PurgeChainCmd),

    /// Revert the chain to a previous state.
    Revert(sc_cli::RevertCmd),

    /// Db meta columns information.
    ChainInfo(sc_cli::ChainInfoCmd),

    /// Sub-commands concerned with benchmarking.
    #[command(subcommand)]
    Benchmark(frame_benchmarking_cli::BenchmarkCmd),
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{Parser, error::ErrorKind};

    #[test]
    fn parses_keystore_in_memory_flag() {
        let cli = Cli::try_parse_from([
            "torus-node",
            "--chain",
            "dev",
            "--validator",
            "--keystore-in-memory",
        ])
        .expect("valid CLI arguments");

        assert!(cli.keystore_in_memory);
    }

    #[test]
    fn rejects_keystore_in_memory_with_on_disk_keystore_path() {
        let error = Cli::try_parse_from([
            "torus-node",
            "--keystore-in-memory",
            "--keystore-path",
            "/tmp/torus-keystore",
        ])
        .expect_err("conflicting keystore options should fail");

        assert_eq!(error.kind(), ErrorKind::ArgumentConflict);
    }

    #[test]
    fn rejects_keystore_in_memory_with_dev_key_shortcuts() {
        let error = Cli::try_parse_from(["torus-node", "--keystore-in-memory", "--alice"])
            .expect_err("dev key shortcuts should fail");

        assert_eq!(error.kind(), ErrorKind::ArgumentConflict);
    }

    #[test]
    fn rejects_keystore_in_memory_with_dev_mode() {
        let error = Cli::try_parse_from(["torus-node", "--keystore-in-memory", "--dev"])
            .expect_err("dev mode should fail");

        assert_eq!(error.kind(), ErrorKind::ArgumentConflict);
    }
}
