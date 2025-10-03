use std::fmt::Display;

use tabled::{
    settings::{object::FirstRow, Remove},
    Table,
};
use torus_client::{
    client::TorusClient,
    subxt::utils::{AccountId32, MultiAddress},
};

use crate::{
    action::{Action, ActionContext},
    keypair::Keypair,
    store::{get_account, get_key},
    util::format_torus,
};

pub struct CheckBalanceAction {
    account: AccountId32,
}

impl Action for CheckBalanceAction {
    type Params = String;

    type ResponseData = CheckBalanceActionResponse;

    async fn create(_ctx: &impl ActionContext, key: Self::Params) -> anyhow::Result<Self> {
        let account = get_account(&key)?;
        Ok(Self { account })
    }

    async fn execute(&self, ctx: &impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        let data = if ctx.is_testnet() {
            let client = TorusClient::for_testnet().await?;
            client
                .system()
                .storage()
                .account_get(&self.account)
                .await?
                .map(|info| (info.data.free, info.data.reserved, info.data.frozen))
        } else {
            let client = TorusClient::for_mainnet().await?;
            client
                .system()
                .storage()
                .account_get(&self.account)
                .await?
                .map(|info| (info.data.free, info.data.reserved, info.data.frozen))
        }
        .unwrap_or((0, 0, 0));

        Ok(CheckBalanceActionResponse {
            address: self.account.to_string(),
            free_balance: data.0,
            reserved_balance: data.1,
            frozen_balance: data.2,
        })
    }
}

#[derive(serde::Serialize)]
pub struct CheckBalanceActionResponse {
    pub address: String,
    pub free_balance: u128,
    pub reserved_balance: u128,
    pub frozen_balance: u128,
}

impl Display for CheckBalanceActionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new(vec![
            (("free".to_string()), format_torus(self.free_balance)),
            (
                ("reserved (stake + others)".to_string()),
                format_torus(self.reserved_balance + self.frozen_balance),
            ),
        ]);
        table.with(Remove::row(FirstRow));

        write!(f, "{table}")
    }
}

pub struct TransferBalanceAction {
    keypair: Keypair,
    target: AccountId32,
    amount: u128,
}

impl Action for TransferBalanceAction {
    type Params = (String, String, u128);

    type ResponseData = TransferBalanceActionResponse;

    async fn estimate_fee(&self, ctx: &impl ActionContext) -> anyhow::Result<u128> {
        let fee = if !ctx.is_testnet() {
            let client = TorusClient::for_mainnet().await?;
            client
                .balances()
                .calls()
                .transfer_keep_alive_fee(
                    MultiAddress::Id(self.target.clone()),
                    self.amount,
                    self.keypair.clone(),
                )
                .await?
        } else {
            let client = TorusClient::for_testnet().await?;
            client
                .balances()
                .calls()
                .transfer_keep_alive_fee(
                    MultiAddress::Id(self.target.clone()),
                    self.amount,
                    self.keypair.clone(),
                )
                .await?
        };

        Ok(fee)
    }

    async fn confirmation_phrase(
        &self,
        ctx: &impl ActionContext,
    ) -> anyhow::Result<Option<String>> {
        let fee = self.estimate_fee(ctx).await?;

        Ok(Some(format!(
            "Are you sure you want to transfer {} to {}? {}\n[y/N]",
            format_torus(self.amount),
            self.target,
            if fee != 0 {
                format!("(there will be a {} torus fee)", format_torus(fee))
            } else {
                "".to_string()
            }
        )))
    }

    async fn create(
        ctx: &impl ActionContext,
        (key, target, amount): Self::Params,
    ) -> anyhow::Result<Self> {
        let key = get_key(&key)?;
        let target = get_account(&target)?;

        let (_, keypair) = ctx.decrypt(&key)?;

        Ok(Self {
            keypair,
            target,
            amount,
        })
    }

    async fn execute(&self, ctx: &impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        if !ctx.is_testnet() {
            let client = TorusClient::for_mainnet().await?;
            client
                .balances()
                .calls()
                .transfer_keep_alive_wait(
                    MultiAddress::Id(self.target.clone()),
                    self.amount,
                    self.keypair.clone(),
                )
                .await?;
        } else {
            let client = TorusClient::for_testnet().await?;
            client
                .balances()
                .calls()
                .transfer_keep_alive_wait(
                    MultiAddress::Id(self.target.clone()),
                    self.amount,
                    self.keypair.clone(),
                )
                .await?;
        };

        Ok(TransferBalanceActionResponse {
            target: self.target.clone(),
            amount: self.amount,
        })
    }
}

#[derive(serde::Serialize)]
pub struct TransferBalanceActionResponse {
    target: AccountId32,
    amount: u128,
}

impl Display for TransferBalanceActionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Successfully transfered {} to {}!",
            format_torus(self.amount),
            self.target
        )
    }
}
