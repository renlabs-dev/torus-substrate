use std::fmt::Display;

use anyhow::anyhow;
use tabled::{Table, Tabled};
use torus_client::{client::TorusClient, subxt::ext::futures::StreamExt};

use crate::{
    action::{Action, ActionContext, Changes},
    keypair::Keypair,
    store::get_key,
};

#[derive(Tabled, serde::Serialize)]
pub struct ApplicationEntry {
    pub id: u32,
    pub payer_key: String,
    pub agent_key: String,
    pub data: String,
    pub cost: u128,
    pub expires_at: u64,
    pub action: String,
    pub status: String,
}

pub struct ApplicationListAction {
    pub page: u32,
    pub count: u32,
}

impl Action for ApplicationListAction {
    type Params = (u32, u32);
    type ResponseData = ApplicationListActionResponse;

    async fn create(
        _ctx: &mut impl ActionContext,
        (page, count): Self::Params,
    ) -> anyhow::Result<Self> {
        Ok(Self { page, count })
    }

    async fn execute(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        ctx.info("Fetching application list...");
        let applications = if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;
            client
                .governance()
                .storage()
                .agent_applications_iter()
                .await?
                .skip((self.page * self.count) as usize)
                .take(self.count as usize)
                .collect::<Vec<_>>()
                .await
                .iter()
                .map(|res| match res {
                    Ok((id, application)) => Ok(ApplicationEntry {
                        id: *id,
                        payer_key: application.payer_key.to_string(),
                        agent_key: application.agent_key.to_string(),
                        data: String::from_utf8_lossy(&application.data.0).to_string(),
                        cost: application.cost,
                        expires_at: application.expires_at,
                        action: match application.action {
                            torus_client::interfaces::mainnet::api::runtime_types::pallet_governance::application::ApplicationAction::Add => "add".to_string(),
                            torus_client::interfaces::mainnet::api::runtime_types::pallet_governance::application::ApplicationAction::Remove => "remove".to_string(),
                        },
                        status: match application.status {
                            torus_client::interfaces::mainnet::api::runtime_types::pallet_governance::application::ApplicationStatus::Open => "open".to_string(),
                            torus_client::interfaces::mainnet::api::runtime_types::pallet_governance::application::ApplicationStatus::Resolved { accepted } => if accepted { "accepted".to_string() } else { "denied".to_string() },
                            torus_client::interfaces::mainnet::api::runtime_types::pallet_governance::application::ApplicationStatus::Expired => "expired".to_string(),
                        },
                    }),
                    Err(err) => Err(anyhow!("{err}")),
                })
                .collect::<Result<Vec<ApplicationEntry>, anyhow::Error>>()?
        } else {
            let client = TorusClient::for_testnet().await?;
            client
                .governance()
                .storage()
                .agent_applications_iter()
                .await?
                .skip((self.page * self.count) as usize)
                .take(self.count as usize)
                .collect::<Vec<_>>()
                .await
                .iter()
                .map(|res| match res {
                    Ok((id, application)) => Ok(ApplicationEntry {
                        id: *id,
                        payer_key: application.payer_key.to_string(),
                        agent_key: application.agent_key.to_string(),
                        data: String::from_utf8_lossy(&application.data.0).to_string(),
                        cost: application.cost,
                        expires_at: application.expires_at,
                        action: match application.action {
                            torus_client::interfaces::testnet::api::runtime_types::pallet_governance::application::ApplicationAction::Add => "add".to_string(),
                            torus_client::interfaces::testnet::api::runtime_types::pallet_governance::application::ApplicationAction::Remove => "remove".to_string(),
                        },
                        status: match application.status {
                            torus_client::interfaces::testnet::api::runtime_types::pallet_governance::application::ApplicationStatus::Open => "open".to_string(),
                            torus_client::interfaces::testnet::api::runtime_types::pallet_governance::application::ApplicationStatus::Resolved { accepted } => if accepted { "accepted".to_string() } else { "denied".to_string() },
                            torus_client::interfaces::testnet::api::runtime_types::pallet_governance::application::ApplicationStatus::Expired => "expired".to_string(),
                        },
                    }),
                    Err(err) => Err(anyhow!("{err}")),
                })
                .collect::<Result<Vec<ApplicationEntry>, anyhow::Error>>()?
        };

        Ok(ApplicationListActionResponse { applications })
    }
}

#[derive(serde::Serialize)]
pub struct ApplicationListActionResponse {
    applications: Vec<ApplicationEntry>,
}

impl Display for ApplicationListActionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let table = Table::new(&self.applications);
        writeln!(f, "{table}")
    }
}

pub struct ApplicationCreateAction {
    key: Keypair,
    metadata: String,
    remove: bool,
}

impl Action for ApplicationCreateAction {
    type Params = (String, String, bool);
    type ResponseData = ApplicationCreateActionResponse;

    async fn create(
        ctx: &mut impl ActionContext,
        (key, metadata, remove): Self::Params,
    ) -> anyhow::Result<Self> {
        let key = get_key(&key)?;
        let (_, keypair) = ctx.decrypt(&key)?;

        Ok(Self {
            key: keypair,
            metadata,
            remove,
        })
    }

    async fn estimate_fee(&self, ctx: &mut impl ActionContext) -> anyhow::Result<u128> {
        let fee = if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;
            client
                .governance()
                .calls()
                .submit_application_fee(
                    self.key.account(),
                    self.metadata.as_bytes().to_vec(),
                    self.remove,
                    self.key.clone(),
                )
                .await?
        } else {
            let client = TorusClient::for_testnet().await?;
            client
                .governance()
                .calls()
                .submit_application_fee(
                    self.key.account(),
                    self.metadata.as_bytes().to_vec(),
                    self.remove,
                    self.key.clone(),
                )
                .await?
        };

        Ok(fee)
    }

    async fn get_changes(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Option<Changes>> {
        let fee = self.estimate_fee(ctx).await?;
        Ok(Some(Changes {
            changes: if !self.remove {
                vec![format!(
                    "Apply to add `{}` as an agent",
                    self.key.ss58_address()
                )]
            } else {
                vec![format!(
                    "Apply to remove `{}` from agents",
                    self.key.ss58_address()
                )]
            },
            fee: Some(fee),
        }))
    }

    async fn execute(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;
            client
                .governance()
                .calls()
                .submit_application_wait(
                    self.key.account(),
                    self.metadata.as_bytes().to_vec(),
                    self.remove,
                    self.key.clone(),
                )
                .await?;
        } else {
            let client = TorusClient::for_testnet().await?;
            client
                .governance()
                .calls()
                .submit_application_wait(
                    self.key.account(),
                    self.metadata.as_bytes().to_vec(),
                    self.remove,
                    self.key.clone(),
                )
                .await?;
        }

        Ok(ApplicationCreateActionResponse)
    }
}

#[derive(serde::Serialize)]
pub struct ApplicationCreateActionResponse;

impl Display for ApplicationCreateActionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Application created successfully")
    }
}
