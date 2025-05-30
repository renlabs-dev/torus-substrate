use clap::Parser;

use crate::sub_command::CliSubcommand;

pub fn parse() -> Cli {
    Cli::parse()
}

#[derive(clap::Parser)]
pub struct Cli {
    #[clap(short, long, default_value_t = 0)]
    pub debug: u8,

    #[clap(subcommand)]
    pub command: CliSubcommand,
}

impl Cli {
    pub fn extract_context(&self) -> CliCtx {
        todo!()
    }
}

pub struct CliCtx {}
