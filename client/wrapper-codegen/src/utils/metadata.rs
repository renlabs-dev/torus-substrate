//! Utilities for working with Substrate metadata.

use codec::{Decode, Encode};
use std::{fs, path::Path};
use subxt::Metadata;

/// Load metadata from a file
pub fn load_metadata_from_file<P: AsRef<Path>>(
    path: P,
) -> Result<Metadata, Box<dyn std::error::Error>> {
    let bytes = fs::read(path)?;
    let metadata = Metadata::decode(&mut &bytes[..])?;
    Ok(metadata)
}

/// Save metadata to a file
pub fn save_metadata_to_file<P: AsRef<Path>>(
    path: P,
    metadata: &Metadata,
) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = metadata.encode();
    fs::write(path, bytes)?;
    Ok(())
}

/// Fetch metadata from a running node
pub async fn fetch_metadata_from_node(url: &str) -> Result<Metadata, Box<dyn std::error::Error>> {
    let client = subxt::OnlineClient::<subxt::PolkadotConfig>::from_url(url).await?;
    let metadata = client.metadata();
    Ok(metadata)
}
