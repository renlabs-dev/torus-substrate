use std::{
    path::PathBuf,
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::anyhow;
use base64::{prelude::BASE64_STANDARD, Engine};
use bip39::Mnemonic;
use blake2::{digest::consts::U32, Digest};
use nacl::secret_box::NONCE_LENGTH;
use rand::Rng;
use sp_core::{
    bytes::from_hex,
    crypto::{default_ss58_version},
};
use torus_client::subxt::utils::AccountId32;

use crate::keypair::Keypair;

const CONFIG_FOLDER: &str = ".torus";

const KEY_FILE_VERSION: u32 = 1;

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct KeyEncryptionMedadata {
    kdf: String,
    cipher: String,
    cipher_text: String,
    nonce_size: u32,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct Key {
    /// File version
    pub version: u32,
    /// If the sensitive data is encrypted and need a password
    pub encrypted: bool,
    /// The time the key was created at
    pub timestamp: u128,
    /// The keypair crypto type
    pub crypto_type: u8,
    /// Hex of the public key bytes
    pub public_key: String,
    /// Hex of the private key bytes, may be encrypted or not depending on the `encrypted` field
    pub private_key: String,
    /// Hex of the keypair seed if present, may be encrypted or not depending on the `encrypted` field
    pub seed_hex: Option<String>,
    /// Mnemonic of the keypair if present, may be encrypted or not depending on the `encrypted` field
    pub mnemonic: Option<String>,
    /// SS58 address format. Used to verify the address
    pub ss58_format: u32,
    /// SS58 address of the key
    pub ss58_address: String,
    /// Path of the json key file (???)
    pub path: String,
    /// ???
    pub derive_path: Option<String>,
    /// Data used to encrypt and decrypt the sensitive fields. Only present if `encrypted` field is true
    pub encryption_metadata: Option<KeyEncryptionMedadata>,
}

pub fn key_exists(name: &str) -> bool {
    let key_path = base_dir()
        .join(CONFIG_FOLDER)
        .join("keys")
        .join(format!("{name}.json"));

    key_path.exists()
}

pub fn get_account(val: &str) -> anyhow::Result<AccountId32> {
    if let Ok(account_id) = AccountId32::from_str(val) {
        return Ok(account_id);
    }

    let key = get_key(val)?;
    Ok(AccountId32::from_str(&key.ss58_address).map_err(|err| anyhow!("{err}"))?)
}

pub fn get_key(name: &str) -> anyhow::Result<Key> {
    let key_path = key_path(name);
    if !key_path.exists() {
        return Err(anyhow!("key {:?} does not exist", name));
    }

    if !key_path.is_file() {
        return Err(anyhow!("{:?}'s key file is not a file", name));
    }

    let Ok(file_str) = std::fs::read_to_string(&key_path) else {
        return Err(anyhow!("could not read {:?}'s key file", name));
    };

    let Ok(key) = serde_json::from_str::<'_, Key>(&file_str) else {
        return Err(anyhow!("{:?}'s key file is corrupted", name));
    };

    Ok(key)
}

fn key_path(name: &str) -> PathBuf {
    base_dir()
        .join(CONFIG_FOLDER)
        .join("keys")
        .join(format!("{name}.json"))
}

#[allow(unused)]
pub fn get_all_key_names() -> anyhow::Result<Vec<String>> {
    let mut key_names = Vec::new();

    let key_folder_path = base_dir().join(CONFIG_FOLDER).join("keys");

    for entry in std::fs::read_dir(key_folder_path)? {
        let entry = entry?;

        if entry.file_type()?.is_dir() {
            continue;
        }

        let file_name = entry.file_name().to_str().unwrap().to_string();
        if !file_name.ends_with(".json") || file_name.starts_with(".") {
            continue;
        }

        key_names.push(file_name.replace(".json", ""));
    }

    Ok(key_names)
}

pub fn get_all_keys() -> anyhow::Result<Vec<Key>> {
    let mut keys = Vec::new();

    let key_folder_path = base_dir().join(CONFIG_FOLDER).join("keys");

    for entry in std::fs::read_dir(key_folder_path)? {
        let entry = entry?;

        if entry.file_type()?.is_dir() {
            continue;
        }

        let file_name = entry.file_name().to_str().unwrap().to_string();
        if !file_name.ends_with(".json") || file_name.starts_with(".") {
            continue;
        }

        let content = std::fs::read_to_string(entry.path())?;
        let key: Key = match serde_json::from_str(&content) {
            Ok(key) => key,
            Err(err) => {
                return Err(err.into());
            }
        };

        keys.push(key);
    }

    Ok(keys)
}

pub fn create_base_dirs() -> anyhow::Result<()> {
    let config_folder_path = base_dir().join(CONFIG_FOLDER);

    if !config_folder_path.exists() {
        std::fs::create_dir_all(&config_folder_path)?;
    }

    let key_folder_path = config_folder_path.join("keys");

    if !key_folder_path.exists() {
        std::fs::create_dir_all(&key_folder_path)?;
    }

    Ok(())
}

pub fn store_new_key(
    name: &str,
    mnemonic: &Mnemonic,
    seed_hex: &str,
    keypair: &Keypair,
    password: Option<&str>,
) -> anyhow::Result<()> {
    let path = key_path(name).to_str().unwrap().to_string();
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
    let crypto_type = keypair.crypto_type();
    let public_key = keypair.hex_public_key();
    let private_key = keypair.hex_private_key();
    let seed_hex = Some(seed_hex.to_string());
    let mnemonic = Some(mnemonic.to_string());
    let ss58_format = default_ss58_version().prefix() as u32;
    let ss58_address = keypair.ss58_address();

    let mut key = Key {
        version: KEY_FILE_VERSION,
        encrypted: false,
        timestamp,
        crypto_type,
        public_key,
        private_key,
        seed_hex,
        mnemonic,
        ss58_format,
        ss58_address,
        path,
        derive_path: None,
        encryption_metadata: None,
    };

    if let Some(password) = password {
        encrypt_key(&mut key, password)?;
    }

    store_key(&key)?;

    Ok(())
}

fn store_key(key: &Key) -> anyhow::Result<()> {
    let path = key.path.clone();

    if std::fs::exists(&path)? {
        return Err(anyhow!("Key file already exists!"));
    }

    let content = serde_json::to_string_pretty(key)?;
    std::fs::write(&path, content)?;

    Ok(())
}

pub fn delete_key(name: &str) -> anyhow::Result<()> {
    let path = key_path(name);

    if !std::fs::exists(&path)? {
        return Err(anyhow!("Key file doesn't exist!"));
    }

    std::fs::remove_file(path)?;

    Ok(())
}

fn derive_key(password: &str) -> Vec<u8> {
    let mut hasher = blake2::Blake2b::<U32>::new();
    hasher.update(password.as_bytes());
    let res = hasher.finalize();
    res.to_vec()
}

fn encrypt_data(password: &str, data: impl AsRef<[u8]>) -> anyhow::Result<Vec<u8>> {
    let key = derive_key(password);
    let mut nonce = [0u8; NONCE_LENGTH];
    rand::rng().fill(&mut nonce);

    nacl::secret_box::format_wn::pack(data.as_ref(), &nonce[..], &key[..])
        .map_err(|err| anyhow!("nacl error: {err:?}"))
}

fn decrypt_data(password: &str, data: impl AsRef<[u8]>) -> anyhow::Result<Vec<u8>> {
    let key = derive_key(password);
    nacl::secret_box::format_wn::open(data.as_ref(), &key[..])
        .map_err(|err| anyhow!("nacl error: {err:?}"))
}

pub fn decrypt_key(key: &mut Key, password: &str) -> anyhow::Result<()> {
    key.private_key = BASE64_STANDARD
        .encode(&decrypt_data(password, BASE64_STANDARD.decode(&key.private_key)?)?[..]);
    key.mnemonic = key
        .mnemonic
        .as_ref()
        .map(
            |mnemonic| match decrypt_data(password, from_hex(mnemonic)?) {
                Ok(data) => Ok(String::from_utf8_lossy(&data[..]).to_string()),
                Err(err) => Err(anyhow!("nacl error: {err:?}")),
            },
        )
        .transpose()?;
    key.seed_hex = key
        .seed_hex
        .as_ref()
        .map(
            |seed_hex| match decrypt_data(password, BASE64_STANDARD.decode(seed_hex)?) {
                Ok(data) => Ok(BASE64_STANDARD.encode(data)),
                Err(err) => Err(anyhow!("nacl error: {err:?}")),
            },
        )
        .transpose()?;
    key.encrypted = false;
    key.encryption_metadata = None;
    Ok(())
}

pub fn encrypt_key(key: &mut Key, password: &str) -> anyhow::Result<()> {
    key.private_key = BASE64_STANDARD
        .encode(&encrypt_data(password, BASE64_STANDARD.decode(&key.private_key)?)?[..]);

    key.mnemonic = key
        .mnemonic
        .as_ref()
        .map(
            |mnemonic| match encrypt_data(password, mnemonic.as_bytes()) {
                Ok(data) => Ok(BASE64_STANDARD.encode(data)),
                Err(err) => Err(anyhow!("nacl error: {err:?}")),
            },
        )
        .transpose()?;

    key.seed_hex = key
        .seed_hex
        .as_ref()
        .map(
            |seed_hex| match encrypt_data(password, BASE64_STANDARD.decode(seed_hex)?) {
                Ok(data) => Ok(BASE64_STANDARD.encode(data)),
                Err(err) => Err(anyhow!("nacl error: {err:?}")),
            },
        )
        .transpose()?;

    key.encryption_metadata = Some(KeyEncryptionMedadata {
        kdf: "blake2b".to_string(),
        cipher: "xsalsa20-poly1305".to_string(),
        cipher_text: "base64".to_string(),
        nonce_size: nacl::secret_box::NONCE_LENGTH as u32,
    });

    key.encrypted = true;

    Ok(())
}

fn base_dir() -> PathBuf {
    home::home_dir().expect("could not get the base cli directory")
}
