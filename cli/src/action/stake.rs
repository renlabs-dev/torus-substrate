use std::fmt::Display;

use tabled::Table;
use torus_client::{
    client::TorusClient,
    subxt::{ext::futures::TryStreamExt, utils::AccountId32},
};

use crate::{
    action::{Action, ActionContext},
    keypair::Keypair,
    store::{get_account, get_key},
    util::format_torus,
};

pub struct GivenStakeAction {
    account: AccountId32,
}

impl Action for GivenStakeAction {
    type Params = String;
    type ResponseData = GivenStakeActionResponse;

    async fn create(_ctx: &impl ActionContext, account: Self::Params) -> anyhow::Result<Self> {
        let account = get_account(&account)?;
        Ok(Self { account })
    }

    async fn execute(&self, ctx: &impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        ctx.info("Fetching given stake...");

        let staking = if ctx.is_testnet() {
            let client = TorusClient::for_testnet().await?;
            client
                .torus0()
                .storage()
                .staking_to_iter1(&self.account)
                .await?
                .try_collect::<Vec<_>>()
                .await?
        } else {
            let client = TorusClient::for_mainnet().await?;
            client
                .torus0()
                .storage()
                .staking_to_iter1(&self.account)
                .await?
                .try_collect::<Vec<_>>()
                .await?
        };

        let mut entries = staking
            .iter()
            .map(|((_, target), amount)| GivenStakeEntry {
                target: target.to_string(),
                amount: format_torus(*amount),
            })
            .collect::<Vec<_>>();

        let sum = staking.iter().map(|(_, amount)| *amount).sum::<u128>();

        entries.push(GivenStakeEntry {
            target: "".to_string(),
            amount: format_torus(sum),
        });

        Ok(GivenStakeActionResponse { entries })
    }
}

#[derive(serde::Serialize, tabled::Tabled)]
struct GivenStakeEntry {
    target: String,
    amount: String,
}

#[derive(serde::Serialize)]
pub struct GivenStakeActionResponse {
    entries: Vec<GivenStakeEntry>,
}

impl Display for GivenStakeActionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let table = Table::new(&self.entries);
        write!(f, "{table}")
    }
}

pub struct ReceivedStakeAction {
    account: AccountId32,
}

impl Action for ReceivedStakeAction {
    type Params = String;
    type ResponseData = ReceivedStakeActionResponse;

    async fn create(_ctx: &impl ActionContext, account: Self::Params) -> anyhow::Result<Self> {
        let account = get_account(&account)?;
        Ok(Self { account })
    }

    async fn execute(&self, ctx: &impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        ctx.info("Fetching given stake...");

        let staking = if ctx.is_testnet() {
            let client = TorusClient::for_testnet().await?;
            client
                .torus0()
                .storage()
                .staking_to_iter1(&self.account)
                .await?
                .try_collect::<Vec<_>>()
                .await?
        } else {
            let client = TorusClient::for_mainnet().await?;
            client
                .torus0()
                .storage()
                .staking_to_iter1(&self.account)
                .await?
                .try_collect::<Vec<_>>()
                .await?
        };

        let mut entries = staking
            .iter()
            .map(|((_, target), amount)| ReceivedStakeEntry {
                source: target.to_string(),
                amount: format_torus(*amount),
            })
            .collect::<Vec<_>>();

        let sum = staking.iter().map(|(_, amount)| *amount).sum::<u128>();

        entries.push(ReceivedStakeEntry {
            source: "".to_string(),
            amount: format_torus(sum),
        });

        Ok(ReceivedStakeActionResponse { entries })
    }
}

#[derive(serde::Serialize, tabled::Tabled)]
struct ReceivedStakeEntry {
    source: String,
    amount: String,
}

#[derive(serde::Serialize)]
pub struct ReceivedStakeActionResponse {
    entries: Vec<ReceivedStakeEntry>,
}

impl Display for ReceivedStakeActionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let table = Table::new(&self.entries);
        write!(f, "{table}")
    }
}

pub struct AddStakeAction {
    keypair: Keypair,
    target: AccountId32,
    amount: u128,
}

impl Action for AddStakeAction {
    type Params = (String, String, u128);
    type ResponseData = AddStakeActionResponse;

    async fn create(
        ctx: &impl ActionContext,
        (key, target, amount): Self::Params,
    ) -> anyhow::Result<Self> {
        let key = get_key(&key)?;
        let (_, keypair) = ctx.decrypt(&key)?;

        let target = get_account(&target)?;

        Ok(Self {
            keypair,
            target,
            amount,
        })
    }

    async fn estimate_fee(&self, ctx: &impl ActionContext) -> anyhow::Result<u128> {
        let fee = if !ctx.is_testnet() {
            let client = TorusClient::for_mainnet().await?;
            client
                .torus0()
                .calls()
                .add_stake_fee(self.target.clone(), self.amount, self.keypair.clone())
                .await?
        } else {
            let client = TorusClient::for_testnet().await?;
            client
                .torus0()
                .calls()
                .add_stake_fee(self.target.clone(), self.amount, self.keypair.clone())
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
            "Are you sure you want to add {} stake to `{}`? {}\n[y/N]",
            format_torus(self.amount),
            self.target,
            if fee != 0 {
                format!("(there will be a {} torus fee)", format_torus(fee))
            } else {
                "".to_string()
            }
        )))
    }

    async fn execute(&self, ctx: &impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        if !ctx.is_testnet() {
            let client = TorusClient::for_mainnet().await?;
            client
                .torus0()
                .calls()
                .add_stake(self.target.clone(), self.amount, self.keypair.clone())
                .await?
        } else {
            let client = TorusClient::for_testnet().await?;
            client
                .torus0()
                .calls()
                .add_stake(self.target.clone(), self.amount, self.keypair.clone())
                .await?
        };

        Ok(AddStakeActionResponse)
    }
}

#[derive(serde::Serialize)]
pub struct AddStakeActionResponse;

impl Display for AddStakeActionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Successfully added stake!")
    }
}

pub struct RemoveStakeAction {
    keypair: Keypair,
    target: AccountId32,
    amount: u128,
}

impl Action for RemoveStakeAction {
    type Params = (String, String, u128);
    type ResponseData = RemoveStakeActionResponse;

    async fn create(
        ctx: &impl ActionContext,
        (key, target, amount): Self::Params,
    ) -> anyhow::Result<Self> {
        let key = get_key(&key)?;
        let (_, keypair) = ctx.decrypt(&key)?;

        let target = get_account(&target)?;

        Ok(Self {
            keypair,
            target,
            amount,
        })
    }

    async fn estimate_fee(&self, ctx: &impl ActionContext) -> anyhow::Result<u128> {
        let fee = if !ctx.is_testnet() {
            let client = TorusClient::for_mainnet().await?;
            client
                .torus0()
                .calls()
                .remove_stake_fee(self.target.clone(), self.amount, self.keypair.clone())
                .await?
        } else {
            let client = TorusClient::for_testnet().await?;
            client
                .torus0()
                .calls()
                .remove_stake_fee(self.target.clone(), self.amount, self.keypair.clone())
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
            "Are you sure you want to remove {} stake from `{}`? {}\n[y/N]",
            format_torus(self.amount),
            self.target,
            if fee != 0 {
                format!("(there will be a {} torus fee)", format_torus(fee))
            } else {
                "".to_string()
            }
        )))
    }

    async fn execute(&self, ctx: &impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        if !ctx.is_testnet() {
            let client = TorusClient::for_mainnet().await?;
            client
                .torus0()
                .calls()
                .add_stake(self.target.clone(), self.amount, self.keypair.clone())
                .await?
        } else {
            let client = TorusClient::for_testnet().await?;
            client
                .torus0()
                .calls()
                .add_stake(self.target.clone(), self.amount, self.keypair.clone())
                .await?
        };

        Ok(RemoveStakeActionResponse)
    }
}

#[derive(serde::Serialize)]
pub struct RemoveStakeActionResponse;

impl Display for RemoveStakeActionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Successfully removed stake!")
    }
}

pub struct TransferStakeAction {
    keypair: Keypair,
    source: AccountId32,
    target: AccountId32,
    amount: u128,
}

impl Action for TransferStakeAction {
    type Params = (String, String, String, u128);
    type ResponseData = TransferStakeActionResponse;

    async fn create(
        ctx: &impl ActionContext,
        (key, source, target, amount): Self::Params,
    ) -> anyhow::Result<Self> {
        let key = get_key(&key)?;
        let (_, keypair) = ctx.decrypt(&key)?;

        let source = get_account(&source)?;
        let target = get_account(&target)?;

        Ok(Self {
            keypair,
            source,
            target,
            amount,
        })
    }

    async fn estimate_fee(&self, ctx: &impl ActionContext) -> anyhow::Result<u128> {
        let fee = if !ctx.is_testnet() {
            let client = TorusClient::for_mainnet().await?;
            client
                .torus0()
                .calls()
                .transfer_stake_fee(
                    self.source.clone(),
                    self.target.clone(),
                    self.amount,
                    self.keypair.clone(),
                )
                .await?
        } else {
            let client = TorusClient::for_testnet().await?;
            client
                .torus0()
                .calls()
                .transfer_stake_fee(
                    self.source.clone(),
                    self.target.clone(),
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
            "Are you sure you want to transfer {} stake from `{}` to `{}`? {}\n[y/N]",
            format_torus(self.amount),
            self.source,
            self.target,
            if fee != 0 {
                format!("(there will be a {} torus fee)", format_torus(fee))
            } else {
                "".to_string()
            }
        )))
    }

    async fn execute(&self, ctx: &impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        if !ctx.is_testnet() {
            let client = TorusClient::for_mainnet().await?;
            client
                .torus0()
                .calls()
                .transfer_stake_fee(
                    self.source.clone(),
                    self.target.clone(),
                    self.amount,
                    self.keypair.clone(),
                )
                .await?
        } else {
            let client = TorusClient::for_testnet().await?;
            client
                .torus0()
                .calls()
                .transfer_stake_fee(
                    self.source.clone(),
                    self.target.clone(),
                    self.amount,
                    self.keypair.clone(),
                )
                .await?
        };

        Ok(TransferStakeActionResponse)
    }
}

#[derive(serde::Serialize)]
pub struct TransferStakeActionResponse;

impl Display for TransferStakeActionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Stake transfered successfully!")
    }
}
