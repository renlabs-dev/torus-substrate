#![allow(clippy::result_large_err)]
//! The Torus node implementation.

mod chain_spec;
mod cli;
mod command;
mod rpc;
mod service;

fn main() -> polkadot_sdk::sc_cli::Result<()> {
    command::run()
}
