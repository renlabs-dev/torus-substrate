//! Utilities for working with Substrate metadata.

use std::{fs, io, path::Path};
use subxt::metadata::Metadata;

/// Load metadata from a file
pub fn load_metadata_from_file<P: AsRef<Path>>(path: P) -> io::Result<Metadata> {
    let bytes = fs::read(path)?;
    let metadata = Metadata::decode(&mut &bytes[..])?;
    Ok(metadata)
}

/// Save metadata to a file
pub fn save_metadata_to_file<P: AsRef<Path>>(path: P, metadata: &Metadata) -> io::Result<()> {
    let bytes = metadata.encode();
    fs::write(path, bytes)
}

/// Fetch metadata from a running node
pub async fn fetch_metadata_from_node(url: &str) -> Result<Metadata, subxt::Error> {
    let client = subxt::backend::rpc::RpcClient::from_url(url).await?;
    let metadata_bytes = client
        .request("state_getMetadata", subxt::backend::rpc::RpcParams::new())
        .await?;
    let metadata = Metadata::decode(&mut &metadata_bytes[..])?;
    Ok(metadata)
}