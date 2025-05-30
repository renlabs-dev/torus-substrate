pub mod key;

#[derive(clap::Subcommand)]
pub enum CliSubcommand {
    Key {},
}
