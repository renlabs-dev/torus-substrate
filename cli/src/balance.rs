use crate::{cli::CliCtx, store::get_key};

#[derive(clap::Subcommand)]
pub enum BalanceCliCommand {
    FreeBalance { key: String },
}

pub async fn free_balance(ctx: CliCtx, key: String) -> anyhow::Result<()> {
    let key = get_key(&key)?;
    
    Ok(())
}
