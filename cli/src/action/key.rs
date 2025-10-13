use std::{fmt::Display, path::PathBuf};

use anyhow::bail;
use bip39::Mnemonic;
use inquire::Password;
use tabled::Table;

use crate::{
    action::{Action, ActionContext, Changes},
    keypair::generate_sr25519_keypair,
    store::{delete_key, get_all_keys, get_key, key_exists, store_new_key, Key},
};

pub struct ListKeysAction;

impl Action for ListKeysAction {
    type Params = ();
    type ResponseData = ListKeysActionResponse;

    async fn create(_ctx: &mut impl ActionContext, _params: Self::Params) -> anyhow::Result<Self> {
        Ok(ListKeysAction)
    }

    async fn execute(&self, _ctx: &mut impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        let keys = get_all_keys()?
            .into_iter()
            .map(|key| KeyEntry {
                name: PathBuf::from(key.path)
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
                    .replace(".json", ""),
                address: key.ss58_address,
            })
            .collect();

        Ok(ListKeysActionResponse { keys })
    }
}

#[derive(serde::Serialize, tabled::Tabled)]
pub struct KeyEntry {
    name: String,
    address: String,
}

#[derive(serde::Serialize)]
pub struct ListKeysActionResponse {
    #[serde(flatten)]
    keys: Vec<KeyEntry>,
}

impl Display for ListKeysActionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let table = Table::new(&self.keys);
        write!(f, "{table}")
    }
}

pub struct CreateKeyAction {
    name: String,
    no_password: bool,
    mnemonic: bool,
}

impl Action for CreateKeyAction {
    type Params = (String, bool, bool);
    type ResponseData = CreateKeyActionResponse;

    async fn create(
        _ctx: &mut impl ActionContext,
        (name, no_password, mnemonic): Self::Params,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            name,
            no_password,
            mnemonic,
        })
    }

    async fn execute(&self, _ctx: &mut impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        let password = if self.no_password {
            None
        } else {
            Some(
                Password::new("Password: ")
                    .without_confirmation()
                    .prompt()?,
            )
        };

        let mnemonic = if self.mnemonic {
            let mnemonic = Password::new("Mnemonic: ")
                .without_confirmation()
                .prompt()?;

            Mnemonic::parse(mnemonic)?
        } else {
            Mnemonic::generate(12)?
        };

        let (keypair, seed) = generate_sr25519_keypair(&mnemonic)?;

        store_new_key(&self.name, &mnemonic, &seed, &keypair, password.as_deref())?;

        Ok(CreateKeyActionResponse {
            name: self.name.clone(),
            public_key: keypair.hex_public_key(),
            address: keypair.ss58_address(),
        })
    }
}

#[derive(serde::Serialize)]
pub struct CreateKeyActionResponse {
    name: String,
    public_key: String,
    address: String,
}

impl Display for CreateKeyActionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Created key `{}` with address `{}` and public key `{}`.",
            &self.name, &self.address, &self.public_key
        )
    }
}

pub struct DeleteKeyAction {
    key: String,
}

impl Action for DeleteKeyAction {
    type Params = String;
    type ResponseData = DeleteKeyActionResponse;

    async fn create(_ctx: &mut impl ActionContext, key: Self::Params) -> anyhow::Result<Self> {
        if !key_exists(&key) {
            bail!("No keys found with this name.");
        }

        Ok(Self { key })
    }

    async fn get_changes(&self, _ctx: &mut impl ActionContext) -> anyhow::Result<Option<Changes>> {
        Ok(Some(Changes {
            changes: vec![format!("Delete the key `{}`", self.key,)],
            fee: None,
        }))
    }

    async fn execute(&self, _ctx: &mut impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        delete_key(&self.key)?;

        Ok(DeleteKeyActionResponse)
    }
}

#[derive(serde::Serialize)]
pub struct DeleteKeyActionResponse;

impl Display for DeleteKeyActionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Key successfully deleted!")
    }
}

pub struct KeyInfoAction {
    key: Key,
}

impl Action for KeyInfoAction {
    type Params = String;
    type ResponseData = KeyInfoActionResponse;

    async fn create(ctx: &mut impl ActionContext, key: Self::Params) -> anyhow::Result<Self> {
        let key = get_key(&key)?;
        let (key, _) = ctx.decrypt(&key)?;
        Ok(Self { key })
    }

    async fn execute(&self, _ctx: &mut impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        Ok(KeyInfoActionResponse {
            name: self.key.name(),
            address: self.key.ss58_address.clone(),
            public_key: self.key.public_key.clone(),
            private_key: self.key.private_key.clone(),
            mnemonic: self.key.mnemonic.clone().unwrap_or("null".to_string()),
            seed_hex: self.key.seed_hex.clone().unwrap_or("null".to_string()),
        })
    }
}

#[derive(serde::Serialize, tabled::Tabled)]
pub struct KeyInfoActionResponse {
    name: String,
    address: String,
    private_key: String,
    public_key: String,
    seed_hex: String,
    mnemonic: String,
}

impl Display for KeyInfoActionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let table = Table::kv(std::iter::once(self));
        write!(f, "{table}")
    }
}
