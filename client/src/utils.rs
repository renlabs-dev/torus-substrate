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

/// Utility to convert a floating number representing tokens to the chain-ready fixed u128
pub fn to_torus(tokens: f64) -> u128 {
    (tokens.abs() * ONE_TORUS as f64) as u128
}
