use std::{cell::LazyCell, fmt::format, path::PathBuf};

use anyhow::anyhow;
use torus_client::interfaces::mainnet::api::sudo::storage::types::key;

const VALID_FOLDERS: [&'static str; 2] = [
    CONFIG_FOLDER,
    ".commune", // Backwards compatibility
];

const CONFIG_FOLDER: &'static str = ".torus";
const CONFIG_FILENAME: &'static str = "config.json";

const CLI_CONFIG: LazyCell<CliConfig> = LazyCell::new(CliConfig::load);

#[derive(serde::Deserialize, serde::Serialize)]
pub struct CliConfig {
    default_key: Option<String>,
}

impl Default for CliConfig {
    fn default() -> Self {
        Self { default_key: None }
    }
}

impl CliConfig {
    pub fn load() -> CliConfig {
        let config_path = base_dir().join(CONFIG_FOLDER).join(CONFIG_FILENAME);
        if !config_path.exists() {
            return CliConfig::default();
        }

        if !config_path.is_file() {
            tracing::warn!("corrupted config, using the default one.");
            return CliConfig::default();
        }

        let Ok(config_str) = std::fs::read_to_string(&config_path) else {
            tracing::warn!("could not read the config file, using the default one.");
            return CliConfig::default();
        };

        let Ok(cli_config) = serde_json::from_str(&config_str) else {
            tracing::warn!("corrupted config, using the default one.");
            return CliConfig::default();
        };

        cli_config
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Key {
    crypto_type: u32,
}

pub fn get_default_key() -> anyhow::Result<Key> {
    match &CLI_CONFIG.default_key {
        Some(name) => get_key(name),
        None => Err(anyhow!("no default key is set")),
    }
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

        return Ok(key);
    }

    return Err(anyhow!("key {:?} does not exist", name));
}

fn base_dir() -> PathBuf {
    return home::home_dir().expect("could not get the base cli directory");
}
