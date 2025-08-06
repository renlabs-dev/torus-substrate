use std::{error::Error, path::PathBuf};

use torus_client_codegen::generate_interfaces;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let output_file = PathBuf::from(format!(
        "{}/interfaces.rs",
        std::env::var("OUT_DIR").unwrap()
    ));

    let devnet_url = if std::env::var("CARGO_FEATURE_DEVNET").is_ok() {
        Some(std::env::var("DEVNET_URL").unwrap_or("wss://localhost:9944".to_string()))
    } else {
        None
    };

    generate_interfaces(&output_file, devnet_url).await?;

    Ok(())
}
