//! Utilities for working with Substrate metadata.

use std::{fs, io, path::Path};

/// Load metadata bytes from a file
pub fn load_metadata_from_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    fs::read(path)
}

/// Save metadata bytes to a file
pub fn save_metadata_to_file<P: AsRef<Path>>(path: P, metadata: &[u8]) -> io::Result<()> {
    fs::write(path, metadata)
}

/// Utility to convert a hex string to bytes
pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, hex::FromHexError> {
    let hex = hex.trim_start_matches("0x");
    hex::decode(hex)
}

/// Utility to convert bytes to a hex string
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    format!("0x{}", hex::encode(bytes))
}

const ONE_TORUS: u128 = 10u128.pow(18);

pub fn to_torus(tokens: f64) -> u128 {
    return (tokens.abs() * ONE_TORUS as f64) as u128;
}
