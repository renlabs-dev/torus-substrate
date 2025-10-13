use std::fmt::Display;

use crate::{keypair::Keypair, store::Key};

pub mod agent;
pub mod application;
pub mod balance;
pub mod key;
pub mod namespace;
pub mod network;
pub mod permission;
pub mod proposal;
pub mod stake;

pub trait ActionContext {
    fn is_json(&self) -> bool;
    fn is_testnet(&self) -> bool;
    fn is_mainnet(&self) -> bool;
    fn is_dry_run(&self) -> bool;

    fn confirm(&mut self, desc: &str) -> anyhow::Result<()>;

    fn decrypt(&mut self, key: &Key) -> anyhow::Result<(Key, Keypair)>;

    fn info(&mut self, message: impl AsRef<str>);
}

pub trait Action: Sized {
    type Params;
    type ResponseData: serde::Serialize + Display;

    async fn create(ctx: &mut impl ActionContext, params: Self::Params) -> anyhow::Result<Self>;

    async fn estimate_fee(&self, _ctx: &mut impl ActionContext) -> anyhow::Result<u128> {
        Ok(0)
    }

    async fn get_changes(&self, _ctx: &mut impl ActionContext) -> anyhow::Result<Option<Changes>> {
        Ok(None)
    }

    async fn execute(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Self::ResponseData>;
}

pub struct Changes {
    pub changes: Vec<String>,
    pub fee: Option<u128>,
}
