use std::fmt::Display;

use crate::{keypair::Keypair, store::Key};

pub mod agent;
pub mod balance;
pub mod key;
pub mod namespace;
pub mod stake;

pub trait ActionContext {
    fn is_json(&self) -> bool;
    fn is_testnet(&self) -> bool;
    fn is_dry_run(&self) -> bool;

    fn confirm(&self, desc: &str) -> anyhow::Result<()>;

    fn decrypt(&self, key: &Key) -> anyhow::Result<(Key, Keypair)>;

    fn info(&self, message: impl AsRef<str>);
}

pub trait Action: Sized {
    type Params;
    type ResponseData: serde::Serialize + Display;

    async fn create(ctx: &impl ActionContext, params: Self::Params) -> anyhow::Result<Self>;

    async fn estimate_fee(&self, _ctx: &impl ActionContext) -> anyhow::Result<u128> {
        Ok(0)
    }

    async fn confirmation_phrase(
        &self,
        _ctx: &impl ActionContext,
    ) -> anyhow::Result<Option<String>> {
        Ok(None)
    }

    async fn execute(&self, ctx: &impl ActionContext) -> anyhow::Result<Self::ResponseData>;
}
