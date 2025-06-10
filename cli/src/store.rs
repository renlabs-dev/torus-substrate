use std::path::PathBuf;

use anyhow::anyhow;
use inquire::Password;

const VALID_FOLDERS: [&'static str; 2] = [
    CONFIG_FOLDER,
    ".commune", // Backwards compatibility
];

const CONFIG_FOLDER: &'static str = ".torus";
const CONFIG_FILENAME: &'static str = "config.json";

#[derive(serde::Deserialize, serde::Serialize)]
pub struct KeyEncryptionMedadata {
    kdf: String,
    cipher: String,
    cipher_text: String,
    nonce_size: u32,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Key {
    /// File version
    version: u32,
    /// If the sensitive data is encrypted and need a password
    encrypted: bool,
    /// The time the key was created at
    timestamp: u64,
    /// The keypair crypto type
    crypto_type: u32,
    /// Hex of the public key bytes
    public_key: String,
    /// Hex of the private key bytes, may be encrypted or not depending on the `encrypted` field
    private_key: String,
    /// Hex of the keypair seed if present, may be encrypted or not depending on the `encrypted` field
    seed_hex: Option<String>,
    /// Mnemonic of the keypair if present, may be encrypted or not depending on the `encrypted` field
    mnemonic: Option<String>,
    /// SS58 address format. Used to verify the address
    ss58_format: u32,
    /// SS58 address of the key
    ss58_address: String,
    /// ???
    path: String,
    /// ???
    derive_path: Option<String>,
    /// Data used to encrypt and decrypt the sensitive fields. Only present if `encrypted` field is true
    encryption_metadata: Option<KeyEncryptionMedadata>,
}

pub fn get_key(name: &str) -> anyhow::Result<Key> {
    for folder in VALID_FOLDERS {
        let key_path = base_dir()
            .join(folder)
            .join("key")
            .join(format!("{}.json", name));
        if !key_path.exists() {
            continue;
        }

        if !key_path.is_file() {
            return Err(anyhow!("{:?}'s key file is not a file", name));
        }

        let Ok(file_str) = std::fs::read_to_string(&key_path) else {
            return Err(anyhow!("could not read {:?}'s key file", name));
        };

        let Ok(key) = serde_json::from_str(&file_str) else {
            return Err(anyhow!("{:?}'s key file is corrupted", name));
        };

        if key.encrypted {
            let password = Password::new("Key password").prompt()?;
            decrypt_key(&mut key, &password);
        }

        return Ok(key);
    }

    return Err(anyhow!("key {:?} does not exist", name));
}

fn base_dir() -> PathBuf {
    return home::home_dir().expect("could not get the base cli directory");
}

fn decrypt_key(key: &mut Key, password: &str) {
    
}

fn decrypt_data(password: &str, data: &str) {
    let key = Blake2bVar
}