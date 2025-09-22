mod cli;
mod keypair;
mod store;
mod util;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    store::create_base_dirs()?;
    cli::execute().await
}
