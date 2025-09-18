use std::path::PathBuf;

use bip39::Mnemonic;
use inquire::Password;
use tabled::{Table, Tabled};

use crate::{
    cli::CliCtx,
    keypair::generate_sr25519_keypair,
    store::{delete_key, get_all_keys, get_key, key_exists, store_new_key},
};

#[derive(clap::Parser)]
pub struct KeyCliCommand {
    #[command(subcommand)]
    pub sub_command: KeyCliSubCommand,
}

#[derive(clap::Subcommand)]
pub enum KeyCliSubCommand {
    List,
    Create {
        name: String,

        #[arg(long)]
        password: bool,
    },
    Delete {
        name: String,
    },
    Info {
        name: String,
    },
}

#[derive(Tabled)]
struct KeyTableRow {
    name: String,
    address: String,
}

#[derive(Tabled)]
struct KeyInfo {
    name: String,
    address: String,
    private_key: String,
    public_key: String,
    seed_hex: String,
    mnemonic: String,
}

pub(super) fn list(_ctx: &CliCtx) -> anyhow::Result<()> {
    let keys = get_all_keys()?.into_iter().map(|key| KeyTableRow {
        name: PathBuf::from(key.path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
            .replace(".json", ""),
        address: key.ss58_address,
    });
    let table = Table::new(keys);
    println!("{table}");
    Ok(())
}

pub(super) fn create(_ctx: &CliCtx, name: String, password: bool) -> anyhow::Result<()> {
    if key_exists(&name) {
        println!("A key with this name already exists.");
        return Ok(());
    }

    let password = if password {
        Some(
            Password::new("Password: ")
                .without_confirmation()
                .prompt()?,
        )
    } else {
        None
    };

    let mnemonic = Mnemonic::generate(12)?;
    let (keypair, seed) = generate_sr25519_keypair(&mnemonic, password.as_deref())?;

    store_new_key(&name, &mnemonic, &seed, &keypair, password.as_deref())?;

    print!(
        "Created key `{}` with public key `{}`.",
        keypair.ss58_address(),
        keypair.hex_public_key()
    );

    Ok(())
}

pub(super) fn delete(_ctx: &CliCtx, name: String) -> anyhow::Result<()> {
    if !key_exists(&name) {
        println!("No keys found with this name.");
        return Ok(());
    }

    delete_key(&name)?;

    Ok(())
}

pub(super) fn info(ctx: &CliCtx, name: String) -> anyhow::Result<()> {
    let key = get_key(&name)?;
    let (key, _) = ctx.decrypt(&key)?;

    let table = Table::kv(std::iter::once(KeyInfo {
        name,
        address: key.ss58_address,
        public_key: key.public_key,
        private_key: key.private_key,
        mnemonic: key.mnemonic.unwrap_or("null".to_string()),
        seed_hex: key.seed_hex.unwrap_or("null".to_string()),
    }));

    println!("{table}");

    Ok(())
}
