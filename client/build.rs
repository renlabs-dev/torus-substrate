use std::{error::Error, path::PathBuf};

use torus_client_codegen::generate_interfaces;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let output_file = PathBuf::from(format!(
        "{}/interfaces.rs",
        std::env::var("OUT_DIR").unwrap()
    ));

    generate_interfaces(&output_file).await?;

    Ok(())
}
