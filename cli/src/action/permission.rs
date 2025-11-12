use std::{fmt::Display, str::FromStr};

use sp_core::H256;
use torus_client::{client::TorusClient, subxt::utils::AccountId32};

use crate::{
    action::{self, Action, ActionContext, Changes},
    keypair::Keypair,
    store::{get_account, get_key},
};

pub enum Allocation {
    Streams(Vec<(H256, u8)>),
    FixedAmount(u128),
}

pub enum DistributionControl {
    Manual,
    Automatic(u128),
    AtBlock(u64),
    Interval(u64),
}

pub enum Duration {
    UntilBlock(u64),
    Indefinite,
}

pub enum RevocationTerms {
    Irrevocable,
    RevocableByDelegator,
    RevocableByArbiters {
        accounts: Vec<AccountId32>,
        required_votes: u32,
    },
    RevocableAfter(u64),
}

pub enum EnforcementAuthority {
    None,
    ControlledBy {
        controllers: Vec<AccountId32>,
        required_votes: u32,
    },
}

pub struct DelegateStreamPermissionAction {
    key: Keypair,
    recipients: Vec<(AccountId32, u16)>,
    allocation: Allocation,
    distribution: DistributionControl,
    duration: Duration,
    revocation: RevocationTerms,
    enforcement: EnforcementAuthority,
    recipient_manager: Option<AccountId32>,
    weight_setter: Option<AccountId32>,
}

impl Action for DelegateStreamPermissionAction {
    type Params = (
        String,
        Vec<(AccountId32, u16)>,
        Allocation,
        DistributionControl,
        Duration,
        RevocationTerms,
        EnforcementAuthority,
        Option<AccountId32>,
        Option<AccountId32>,
    );
    type ResponseData = DelegateStreamPermissionActionResponse;

    async fn create(
        ctx: &mut impl ActionContext,
        (
            key,
            recipients,
            allocation,
            distribution,
            duration,
            revocation,
            enforcement,
            recipient_manager,
            weight_setter,
        ): Self::Params,
    ) -> anyhow::Result<Self> {
        let key = get_key(&key)?;
        let (_, keypair) = ctx.decrypt(&key)?;

        Ok(Self {
            key: keypair,
            recipients,
            allocation,
            distribution,
            duration,
            revocation,
            enforcement,
            recipient_manager,
            weight_setter,
        })
    }

    async fn execute(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;

            let allocation = match &self.allocation {
                    Allocation::Streams(items) => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::stream::StreamAllocation::Streams(
                        torus_client::interfaces::mainnet::api::runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap(items.iter().map(|(i, j)|
                        (*i, torus_client::interfaces::mainnet::api::runtime_types::sp_arithmetic::per_things::Percent(*j))).collect())
                    ),
                    Allocation::FixedAmount(value) => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::stream::StreamAllocation::FixedAmount(*value)
                };

            let distribution = match &self.distribution {
                DistributionControl::Manual => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::stream::DistributionControl::Manual,
                DistributionControl::Automatic(value) => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::stream::DistributionControl::Automatic(*value),
                DistributionControl::AtBlock(value) => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::stream::DistributionControl::AtBlock(*value),
                DistributionControl::Interval(value) => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::stream::DistributionControl::Interval(*value),
            };

            let duration = match &self.duration {
                Duration::UntilBlock(value) => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::PermissionDuration::UntilBlock(*value),
                Duration::Indefinite => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::PermissionDuration::Indefinite,
            };

            let revocation = match &self.revocation {
                RevocationTerms::Irrevocable => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::Irrevocable,
                RevocationTerms::RevocableByDelegator => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableByDelegator,
                RevocationTerms::RevocableByArbiters { accounts, required_votes } => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableByArbiters {
                    accounts: torus_client::interfaces::mainnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(accounts.clone()),
                    required_votes: *required_votes
                },
                RevocationTerms::RevocableAfter(value) => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableAfter(*value),
            };

            let enforcement = match &self.enforcement {
                EnforcementAuthority::None => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::EnforcementAuthority::None,
                EnforcementAuthority::ControlledBy { controllers, required_votes } => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::EnforcementAuthority::ControlledBy {
                    controllers: torus_client::interfaces::mainnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(controllers.clone()),
                    required_votes: *required_votes,
                },
            };

            client
                .permission0()
                .calls()
                .delegate_stream_permission_wait(
                    torus_client::interfaces::mainnet::api::runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap(self.recipients.clone()), allocation, distribution, duration, revocation, enforcement, self.recipient_manager.clone(), self.weight_setter.clone(), self.key.clone())
                .await?
        } else {
            let client = TorusClient::for_testnet().await?;
            let allocation = match &self.allocation {
                    Allocation::Streams(items) => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::stream::StreamAllocation::Streams(
                        torus_client::interfaces::testnet::api::runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap(items.iter().map(|(i, j)|
                        (*i, torus_client::interfaces::testnet::api::runtime_types::sp_arithmetic::per_things::Percent(*j))).collect())
                    ),
                    Allocation::FixedAmount(value) => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::stream::StreamAllocation::FixedAmount(*value)
                };

            let distribution = match &self.distribution {
                DistributionControl::Manual => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::stream::DistributionControl::Manual,
                DistributionControl::Automatic(value) => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::stream::DistributionControl::Automatic(*value),
                DistributionControl::AtBlock(value) => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::stream::DistributionControl::AtBlock(*value),
                DistributionControl::Interval(value) => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::stream::DistributionControl::Interval(*value),
            };

            let duration = match &self.duration {
                Duration::UntilBlock(value) => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::PermissionDuration::UntilBlock(*value),
                Duration::Indefinite => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::PermissionDuration::Indefinite,
            };

            let revocation = match &self.revocation {
                RevocationTerms::Irrevocable => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::Irrevocable,
                RevocationTerms::RevocableByDelegator => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableByDelegator,
                RevocationTerms::RevocableByArbiters { accounts, required_votes } => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableByArbiters {
                    accounts: torus_client::interfaces::testnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(accounts.clone()),
                    required_votes: *required_votes
                },
                RevocationTerms::RevocableAfter(value) => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableAfter(*value),
            };

            let enforcement = match &self.enforcement {
                EnforcementAuthority::None => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::EnforcementAuthority::None,
                EnforcementAuthority::ControlledBy { controllers, required_votes } => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::EnforcementAuthority::ControlledBy {
                    controllers: torus_client::interfaces::testnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(controllers.clone()),
                    required_votes: *required_votes,
                },
            };

            client
                .permission0()
                .calls()
                .delegate_stream_permission_wait(
                    torus_client::interfaces::testnet::api::runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap(self.recipients.clone()), allocation, distribution, duration, revocation, enforcement, self.recipient_manager.clone(), self.weight_setter.clone(), self.key.clone())
                .await?
        }

        Ok(DelegateStreamPermissionActionResponse)
    }

    async fn estimate_fee(&self, ctx: &mut impl ActionContext) -> anyhow::Result<u128> {
        let fee = if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;

            let allocation = match &self.allocation {
                    Allocation::Streams(items) => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::stream::StreamAllocation::Streams(
                        torus_client::interfaces::mainnet::api::runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap(items.iter().map(|(i, j)|
                        (*i, torus_client::interfaces::mainnet::api::runtime_types::sp_arithmetic::per_things::Percent(*j))).collect())
                    ),
                    Allocation::FixedAmount(value) => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::stream::StreamAllocation::FixedAmount(*value)
                };

            let distribution = match &self.distribution {
                DistributionControl::Manual => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::stream::DistributionControl::Manual,
                DistributionControl::Automatic(value) => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::stream::DistributionControl::Automatic(*value),
                DistributionControl::AtBlock(value) => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::stream::DistributionControl::AtBlock(*value),
                DistributionControl::Interval(value) => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::stream::DistributionControl::Interval(*value),
            };

            let duration = match &self.duration {
                Duration::UntilBlock(value) => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::PermissionDuration::UntilBlock(*value),
                Duration::Indefinite => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::PermissionDuration::Indefinite,
            };

            let revocation = match &self.revocation {
                RevocationTerms::Irrevocable => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::Irrevocable,
                RevocationTerms::RevocableByDelegator => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableByDelegator,
                RevocationTerms::RevocableByArbiters { accounts, required_votes } => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableByArbiters {
                    accounts: torus_client::interfaces::mainnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(accounts.clone()),
                    required_votes: *required_votes
                },
                RevocationTerms::RevocableAfter(value) => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableAfter(*value),
            };

            let enforcement = match &self.enforcement {
                EnforcementAuthority::None => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::EnforcementAuthority::None,
                EnforcementAuthority::ControlledBy { controllers, required_votes } => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::EnforcementAuthority::ControlledBy {
                    controllers: torus_client::interfaces::mainnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(controllers.clone()),
                    required_votes: *required_votes,
                },
            };

            client
                .permission0()
                .calls()
                .delegate_stream_permission_fee(
                    torus_client::interfaces::mainnet::api::runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap(self.recipients.clone()), allocation, distribution, duration, revocation, enforcement, self.recipient_manager.clone(), self.weight_setter.clone(), self.key.clone())
                .await?
        } else {
            let client = TorusClient::for_testnet().await?;
            let allocation = match &self.allocation {
                    Allocation::Streams(items) => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::stream::StreamAllocation::Streams(
                        torus_client::interfaces::testnet::api::runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap(items.iter().map(|(i, j)|
                        (*i, torus_client::interfaces::testnet::api::runtime_types::sp_arithmetic::per_things::Percent(*j))).collect())
                    ),
                    Allocation::FixedAmount(value) => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::stream::StreamAllocation::FixedAmount(*value)
                };

            let distribution = match &self.distribution {
                DistributionControl::Manual => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::stream::DistributionControl::Manual,
                DistributionControl::Automatic(value) => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::stream::DistributionControl::Automatic(*value),
                DistributionControl::AtBlock(value) => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::stream::DistributionControl::AtBlock(*value),
                DistributionControl::Interval(value) => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::stream::DistributionControl::Interval(*value),
            };

            let duration = match &self.duration {
                Duration::UntilBlock(value) => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::PermissionDuration::UntilBlock(*value),
                Duration::Indefinite => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::PermissionDuration::Indefinite,
            };

            let revocation = match &self.revocation {
                RevocationTerms::Irrevocable => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::Irrevocable,
                RevocationTerms::RevocableByDelegator => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableByDelegator,
                RevocationTerms::RevocableByArbiters { accounts, required_votes } => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableByArbiters {
                    accounts: torus_client::interfaces::testnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(accounts.clone()),
                    required_votes: *required_votes
                },
                RevocationTerms::RevocableAfter(value) => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableAfter(*value),
            };

            let enforcement = match &self.enforcement {
                EnforcementAuthority::None => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::EnforcementAuthority::None,
                EnforcementAuthority::ControlledBy { controllers, required_votes } => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::EnforcementAuthority::ControlledBy {
                    controllers: torus_client::interfaces::testnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(controllers.clone()),
                    required_votes: *required_votes,
                },
            };

            client
                .permission0()
                .calls()
                .delegate_stream_permission_fee(
                    torus_client::interfaces::testnet::api::runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap(self.recipients.clone()), allocation, distribution, duration, revocation, enforcement, self.recipient_manager.clone(), self.weight_setter.clone(), self.key.clone())
                .await?
        };

        Ok(fee)
    }

    async fn get_changes(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Option<Changes>> {
        let fee = self.estimate_fee(ctx).await?;
        Ok(Some(Changes {
            changes: vec![format!(
                "Delegate stream permission to {} recipients",
                self.recipients.len()
            )],
            fee: Some(fee),
        }))
    }
}

#[derive(serde::Serialize)]
pub struct DelegateStreamPermissionActionResponse;

impl Display for DelegateStreamPermissionActionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Stream permission delegated successfully")
    }
}

pub struct DelegateNamespacePermissionAction {
    pub key: Keypair,
    pub recipient: AccountId32,
    pub paths: Vec<(Option<H256>, Vec<String>)>,
    pub duration: Duration,
    pub revocation: RevocationTerms,
    pub instances: u32,
}

impl Action for DelegateNamespacePermissionAction {
    type Params = (
        String,
        String,
        Vec<(Option<H256>, Vec<String>)>,
        Duration,
        RevocationTerms,
        u32,
    );
    type ResponseData = DelegateNamespacePermissionActionResponse;

    async fn create(
        ctx: &mut impl ActionContext,
        (key, recipient, paths, duration, revocation, instances): Self::Params,
    ) -> anyhow::Result<Self> {
        let key = get_key(&key)?;
        let (_, keypair) = ctx.decrypt(&key)?;

        let recipient = get_account(&recipient)?;

        Ok(Self {
            key: keypair,
            recipient,
            paths,
            duration,
            revocation,
            instances,
        })
    }

    async fn execute(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;

            let paths = torus_client::interfaces::mainnet::api::runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap(
                self.paths.iter().cloned()
                .map(|(id, paths)| (id, torus_client::interfaces::mainnet::api::runtime_types::bounded_collections::bounded_btree_set::BoundedBTreeSet(
                    paths.into_iter().map(|path| torus_client::interfaces::mainnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(path.as_bytes().to_vec())).collect()
                ))).collect()
            );

            let duration = match &self.duration {
                Duration::UntilBlock(value) => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::PermissionDuration::UntilBlock(*value),
                Duration::Indefinite => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::PermissionDuration::Indefinite,
            };

            let revocation = match &self.revocation {
                RevocationTerms::Irrevocable => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::Irrevocable,
                RevocationTerms::RevocableByDelegator => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableByDelegator,
                RevocationTerms::RevocableByArbiters { accounts, required_votes } => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableByArbiters {
                    accounts: torus_client::interfaces::mainnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(accounts.clone()),
                    required_votes: *required_votes
                },
                RevocationTerms::RevocableAfter(value) => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableAfter(*value),
            };

            client
                .permission0()
                .calls()
                .delegate_namespace_permission_wait(
                    self.recipient.clone(),
                    paths,
                    duration,
                    revocation,
                    self.instances,
                    self.key.clone(),
                )
                .await?;
        } else {
            let client = TorusClient::for_testnet().await?;

            let paths = torus_client::interfaces::testnet::api::runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap(
                self.paths.iter().cloned()
                .map(|(id, paths)| (id, torus_client::interfaces::testnet::api::runtime_types::bounded_collections::bounded_btree_set::BoundedBTreeSet(
                    paths.into_iter().map(|path| torus_client::interfaces::testnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(path.as_bytes().to_vec())).collect()
                ))).collect()
            );

            let duration = match &self.duration {
                Duration::UntilBlock(value) => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::PermissionDuration::UntilBlock(*value),
                Duration::Indefinite => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::PermissionDuration::Indefinite,
            };

            let revocation = match &self.revocation {
                RevocationTerms::Irrevocable => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::Irrevocable,
                RevocationTerms::RevocableByDelegator => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableByDelegator,
                RevocationTerms::RevocableByArbiters { accounts, required_votes } => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableByArbiters {
                    accounts: torus_client::interfaces::testnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(accounts.clone()),
                    required_votes: *required_votes
                },
                RevocationTerms::RevocableAfter(value) => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableAfter(*value),
            };

            client
                .permission0()
                .calls()
                .delegate_namespace_permission_wait(
                    self.recipient.clone(),
                    paths,
                    duration,
                    revocation,
                    self.instances,
                    self.key.clone(),
                )
                .await?;
        }

        Ok(DelegateNamespacePermissionActionResponse)
    }

    async fn estimate_fee(&self, ctx: &mut impl ActionContext) -> anyhow::Result<u128> {
        let fee = if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;

            let paths = torus_client::interfaces::mainnet::api::runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap(
                self.paths.iter().cloned()
                .map(|(id, paths)| (id, torus_client::interfaces::mainnet::api::runtime_types::bounded_collections::bounded_btree_set::BoundedBTreeSet(
                    paths.into_iter().map(|path| torus_client::interfaces::mainnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(path.as_bytes().to_vec())).collect()
                ))).collect()
            );

            let duration = match &self.duration {
                Duration::UntilBlock(value) => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::PermissionDuration::UntilBlock(*value),
                Duration::Indefinite => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::PermissionDuration::Indefinite,
            };

            let revocation = match &self.revocation {
                RevocationTerms::Irrevocable => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::Irrevocable,
                RevocationTerms::RevocableByDelegator => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableByDelegator,
                RevocationTerms::RevocableByArbiters { accounts, required_votes } => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableByArbiters {
                    accounts: torus_client::interfaces::mainnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(accounts.clone()),
                    required_votes: *required_votes
                },
                RevocationTerms::RevocableAfter(value) => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableAfter(*value),
            };

            client
                .permission0()
                .calls()
                .delegate_namespace_permission_fee(
                    self.recipient.clone(),
                    paths,
                    duration,
                    revocation,
                    self.instances,
                    self.key.clone(),
                )
                .await?
        } else {
            let client = TorusClient::for_testnet().await?;

            let paths = torus_client::interfaces::testnet::api::runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap(
                self.paths.iter().cloned()
                .map(|(id, paths)| (id, torus_client::interfaces::testnet::api::runtime_types::bounded_collections::bounded_btree_set::BoundedBTreeSet(
                    paths.into_iter().map(|path| torus_client::interfaces::testnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(path.as_bytes().to_vec())).collect()
                ))).collect()
            );

            let duration = match &self.duration {
                Duration::UntilBlock(value) => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::PermissionDuration::UntilBlock(*value),
                Duration::Indefinite => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::PermissionDuration::Indefinite,
            };

            let revocation = match &self.revocation {
                RevocationTerms::Irrevocable => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::Irrevocable,
                RevocationTerms::RevocableByDelegator => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableByDelegator,
                RevocationTerms::RevocableByArbiters { accounts, required_votes } => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableByArbiters {
                    accounts: torus_client::interfaces::testnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(accounts.clone()),
                    required_votes: *required_votes
                },
                RevocationTerms::RevocableAfter(value) => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableAfter(*value),
            };

            client
                .permission0()
                .calls()
                .delegate_namespace_permission_fee(
                    self.recipient.clone(),
                    paths,
                    duration,
                    revocation,
                    self.instances,
                    self.key.clone(),
                )
                .await?
        };

        Ok(fee)
    }

    async fn get_changes(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Option<Changes>> {
        let fee = self.estimate_fee(ctx).await?;
        Ok(Some(Changes {
            changes: vec![format!(
                "Delegate namespace permission to {}",
                self.recipient
            )],
            fee: Some(fee),
        }))
    }
}

#[derive(serde::Serialize)]
pub struct DelegateNamespacePermissionActionResponse;

impl Display for DelegateNamespacePermissionActionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Namespace permission delegated successfully")
    }
}

pub struct DelegateWalletPermissionAction {
    pub key: Keypair,
    pub recipient: AccountId32,
    pub can_transfer_stake: bool,
    pub exclusive_stake_access: bool,
    pub duration: Duration,
    pub revocation: RevocationTerms,
}

impl Action for DelegateWalletPermissionAction {
    type Params = (String, String, bool, bool, Duration, RevocationTerms);
    type ResponseData = DelegateWalletPermissionActionResponse;

    async fn create(
        ctx: &mut impl ActionContext,
        (key, recipient, can_transfer_stake, exclusive_stake_access, duration, revocation): Self::Params,
    ) -> anyhow::Result<Self> {
        let key = get_key(&key)?;
        let (_, keypair) = ctx.decrypt(&key)?;

        let recipient = get_account(&recipient)?;

        Ok(Self {
            key: keypair,
            recipient,
            can_transfer_stake,
            exclusive_stake_access,
            duration,
            revocation,
        })
    }

    async fn execute(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;

            let duration = match &self.duration {
                Duration::UntilBlock(value) => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::PermissionDuration::UntilBlock(*value),
                Duration::Indefinite => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::PermissionDuration::Indefinite,
            };

            let revocation = match &self.revocation {
                RevocationTerms::Irrevocable => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::Irrevocable,
                RevocationTerms::RevocableByDelegator => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableByDelegator,
                RevocationTerms::RevocableByArbiters { accounts, required_votes } => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableByArbiters {
                    accounts: torus_client::interfaces::mainnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(accounts.clone()),
                    required_votes: *required_votes
                },
                RevocationTerms::RevocableAfter(value) => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableAfter(*value),
            };

            client
                .permission0()
                .calls()
                .delegate_wallet_stake_permission_wait(
                    self.recipient.clone(),
                    torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::wallet::WalletStake {
                        can_transfer_stake: self.can_transfer_stake,
                        exclusive_stake_access: self.exclusive_stake_access
                    },
                    duration,
                    revocation,
                    self.key.clone()
                )
                .await?;
        } else {
            let client = TorusClient::for_testnet().await?;

            let duration = match &self.duration {
                Duration::UntilBlock(value) => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::PermissionDuration::UntilBlock(*value),
                Duration::Indefinite => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::PermissionDuration::Indefinite,
            };

            let revocation = match &self.revocation {
                RevocationTerms::Irrevocable => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::Irrevocable,
                RevocationTerms::RevocableByDelegator => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableByDelegator,
                RevocationTerms::RevocableByArbiters { accounts, required_votes } => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableByArbiters {
                    accounts: torus_client::interfaces::testnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(accounts.clone()),
                    required_votes: *required_votes
                },
                RevocationTerms::RevocableAfter(value) => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableAfter(*value),
            };

            client
                .permission0()
                .calls()
                .delegate_wallet_stake_permission_wait(
                    self.recipient.clone(),
                    torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::wallet::WalletStake {
                        can_transfer_stake: self.can_transfer_stake,
                        exclusive_stake_access: self.exclusive_stake_access
                    },
                    duration,
                    revocation,
                    self.key.clone()
                )
                .await?;
        }

        Ok(DelegateWalletPermissionActionResponse)
    }

    async fn estimate_fee(&self, ctx: &mut impl ActionContext) -> anyhow::Result<u128> {
        let fee = if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;

            let duration = match &self.duration {
                Duration::UntilBlock(value) => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::PermissionDuration::UntilBlock(*value),
                Duration::Indefinite => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::PermissionDuration::Indefinite,
            };

            let revocation = match &self.revocation {
                RevocationTerms::Irrevocable => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::Irrevocable,
                RevocationTerms::RevocableByDelegator => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableByDelegator,
                RevocationTerms::RevocableByArbiters { accounts, required_votes } => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableByArbiters {
                    accounts: torus_client::interfaces::mainnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(accounts.clone()),
                    required_votes: *required_votes
                },
                RevocationTerms::RevocableAfter(value) => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableAfter(*value),
            };

            client
                .permission0()
                .calls()
                .delegate_wallet_stake_permission_fee(
                    self.recipient.clone(),
                    torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::wallet::WalletStake {
                        can_transfer_stake: self.can_transfer_stake,
                        exclusive_stake_access: self.exclusive_stake_access
                    },
                    duration,
                    revocation,
                    self.key.clone()
                )
                .await?
        } else {
            let client = TorusClient::for_testnet().await?;

            let duration = match &self.duration {
                Duration::UntilBlock(value) => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::PermissionDuration::UntilBlock(*value),
                Duration::Indefinite => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::PermissionDuration::Indefinite,
            };

            let revocation = match &self.revocation {
                RevocationTerms::Irrevocable => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::Irrevocable,
                RevocationTerms::RevocableByDelegator => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableByDelegator,
                RevocationTerms::RevocableByArbiters { accounts, required_votes } => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableByArbiters {
                    accounts: torus_client::interfaces::testnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(accounts.clone()),
                    required_votes: *required_votes
                },
                RevocationTerms::RevocableAfter(value) => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableAfter(*value),
            };

            client
                .permission0()
                .calls()
                .delegate_wallet_stake_permission_fee(
                    self.recipient.clone(),
                    torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::wallet::WalletStake {
                        can_transfer_stake: self.can_transfer_stake,
                        exclusive_stake_access: self.exclusive_stake_access
                    },
                    duration,
                    revocation,
                    self.key.clone()
                )
                .await?
        };

        Ok(fee)
    }

    async fn get_changes(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Option<Changes>> {
        let fee = self.estimate_fee(ctx).await?;
        Ok(Some(Changes {
            changes: vec![format!(
                "Delegate wallet stake permission to {}",
                self.recipient
            )],
            fee: Some(fee),
        }))
    }
}

#[derive(serde::Serialize)]
pub struct DelegateWalletPermissionActionResponse;

impl Display for DelegateWalletPermissionActionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Wallet stake permission delegated successfully")
    }
}

pub struct DelegateCuratorPermissionAction {
    pub key: Keypair,
    pub recipient: AccountId32,
    pub flags: Vec<(Option<H256>, u32)>,
    pub cooldown: Option<u64>,
    pub duration: Duration,
    pub instances: u32,
    pub revocation: RevocationTerms,
}

impl Action for DelegateCuratorPermissionAction {
    type Params = (
        String,
        String,
        Vec<(Option<H256>, u32)>,
        Option<u64>,
        Duration,
        u32,
        RevocationTerms,
    );
    type ResponseData = DelegateCuratorPermissionActionResponse;

    async fn create(
        ctx: &mut impl ActionContext,
        (key, recipient, flags, cooldown, duration, instances, revocation): Self::Params,
    ) -> anyhow::Result<Self> {
        let key = get_key(&key)?;
        let (_, keypair) = ctx.decrypt(&key)?;

        let recipient = get_account(&recipient)?;

        Ok(Self {
            key: keypair,
            recipient,
            flags,
            cooldown,
            duration,
            instances,
            revocation,
        })
    }

    async fn execute(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;

            let duration = match &self.duration {
                Duration::UntilBlock(value) => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::PermissionDuration::UntilBlock(*value),
                Duration::Indefinite => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::PermissionDuration::Indefinite,
            };

            let revocation = match &self.revocation {
                RevocationTerms::Irrevocable => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::Irrevocable,
                RevocationTerms::RevocableByDelegator => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableByDelegator,
                RevocationTerms::RevocableByArbiters { accounts, required_votes } => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableByArbiters {
                    accounts: torus_client::interfaces::mainnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(accounts.clone()),
                    required_votes: *required_votes
                },
                RevocationTerms::RevocableAfter(value) => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableAfter(*value),
            };

            client
                .permission0()
                .calls()
                .delegate_curator_permission_wait(
                    self.recipient.clone(),
                    torus_client::interfaces::mainnet::api::runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap(self.flags.clone()),
                    self.cooldown,
                    duration,
                    revocation,
                    self.instances,
                    self.key.clone(),
                )
                .await?;
        } else {
            let client = TorusClient::for_testnet().await?;

            let duration = match &self.duration {
                Duration::UntilBlock(value) => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::PermissionDuration::UntilBlock(*value),
                Duration::Indefinite => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::PermissionDuration::Indefinite,
            };

            let revocation = match &self.revocation {
                RevocationTerms::Irrevocable => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::Irrevocable,
                RevocationTerms::RevocableByDelegator => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableByDelegator,
                RevocationTerms::RevocableByArbiters { accounts, required_votes } => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableByArbiters {
                    accounts: torus_client::interfaces::testnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(accounts.clone()),
                    required_votes: *required_votes
                },
                RevocationTerms::RevocableAfter(value) => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableAfter(*value),
            };

            client
                .permission0()
                .calls()
                .delegate_curator_permission_wait(
                    self.recipient.clone(),
                    torus_client::interfaces::testnet::api::runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap(self.flags.clone()),
                    self.cooldown,
                    duration,
                    revocation,
                    self.instances,
                    self.key.clone(),
                )
                .await?;
        }

        Ok(DelegateCuratorPermissionActionResponse)
    }

    async fn estimate_fee(&self, ctx: &mut impl ActionContext) -> anyhow::Result<u128> {
        let fee = if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;

            let duration = match &self.duration {
                Duration::UntilBlock(value) => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::PermissionDuration::UntilBlock(*value),
                Duration::Indefinite => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::PermissionDuration::Indefinite,
            };

            let revocation = match &self.revocation {
                RevocationTerms::Irrevocable => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::Irrevocable,
                RevocationTerms::RevocableByDelegator => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableByDelegator,
                RevocationTerms::RevocableByArbiters { accounts, required_votes } => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableByArbiters {
                    accounts: torus_client::interfaces::mainnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(accounts.clone()),
                    required_votes: *required_votes
                },
                RevocationTerms::RevocableAfter(value) => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableAfter(*value),
            };

            client
                .permission0()
                .calls()
                .delegate_curator_permission_fee(
                    self.recipient.clone(),
                    torus_client::interfaces::mainnet::api::runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap(self.flags.clone()),
                    self.cooldown,
                    duration,
                    revocation,
                    self.instances,
                    self.key.clone(),
                )
                .await?
        } else {
            let client = TorusClient::for_testnet().await?;

            let duration = match &self.duration {
                Duration::UntilBlock(value) => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::PermissionDuration::UntilBlock(*value),
                Duration::Indefinite => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::PermissionDuration::Indefinite,
            };

            let revocation = match &self.revocation {
                RevocationTerms::Irrevocable => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::Irrevocable,
                RevocationTerms::RevocableByDelegator => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableByDelegator,
                RevocationTerms::RevocableByArbiters { accounts, required_votes } => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableByArbiters {
                    accounts: torus_client::interfaces::testnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(accounts.clone()),
                    required_votes: *required_votes
                },
                RevocationTerms::RevocableAfter(value) => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::RevocationTerms::RevocableAfter(*value),
            };

            client
                .permission0()
                .calls()
                .delegate_curator_permission_fee(
                    self.recipient.clone(),
                    torus_client::interfaces::testnet::api::runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap(self.flags.clone()),
                    self.cooldown,
                    duration,
                    revocation,
                    self.instances,
                    self.key.clone(),
                )
                .await?
        };

        Ok(fee)
    }

    async fn get_changes(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Option<Changes>> {
        let fee = self.estimate_fee(ctx).await?;
        Ok(Some(Changes {
            changes: vec![format!(
                "Delegate {} instances of curator permission to {}",
                self.instances, self.recipient
            )],
            fee: Some(fee),
        }))
    }
}

#[derive(serde::Serialize)]
pub struct DelegateCuratorPermissionActionResponse;

impl Display for DelegateCuratorPermissionActionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Curator permission delegated successfully")
    }
}

pub struct RevokePermissionAction {
    key: Keypair,
    permission_id: H256,
}

impl Action for RevokePermissionAction {
    type Params = (String, String);
    type ResponseData = RevokePermissionActionResponse;

    async fn create(
        ctx: &mut impl ActionContext,
        (key, permission_id): Self::Params,
    ) -> anyhow::Result<Self> {
        let key = get_key(&key)?;
        let (_, keypair) = ctx.decrypt(&key)?;

        let permission_id = H256::from_str(&permission_id)?;

        Ok(Self {
            key: keypair,
            permission_id,
        })
    }

    async fn execute(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;
            client
                .permission0()
                .calls()
                .revoke_permission_wait(self.permission_id, self.key.clone())
                .await?;
        } else {
            let client = TorusClient::for_testnet().await?;
            client
                .permission0()
                .calls()
                .revoke_permission_wait(self.permission_id, self.key.clone())
                .await?;
        }

        Ok(RevokePermissionActionResponse)
    }

    async fn estimate_fee(&self, ctx: &mut impl ActionContext) -> anyhow::Result<u128> {
        let fee = if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;
            client
                .permission0()
                .calls()
                .revoke_permission_fee(self.permission_id, self.key.clone())
                .await?
        } else {
            let client = TorusClient::for_testnet().await?;
            client
                .permission0()
                .calls()
                .revoke_permission_fee(self.permission_id, self.key.clone())
                .await?
        };

        Ok(fee)
    }

    async fn get_changes(
        &self,
        ctx: &mut impl ActionContext,
    ) -> anyhow::Result<Option<super::Changes>> {
        let fee = self.estimate_fee(ctx).await?;

        Ok(Some(super::Changes {
            changes: vec![format!(
                "Revoke permission {}",
                self.permission_id.to_string()
            )],
            fee: Some(fee),
        }))
    }
}

#[derive(serde::Serialize)]
pub struct RevokePermissionActionResponse;

impl Display for RevokePermissionActionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Permission revoked successfully!")
    }
}

pub struct ExecutePermissionAction {
    pub key: Keypair,
    pub permission_id: H256,
    pub enforce: bool,
}

impl Action for ExecutePermissionAction {
    type Params = (String, String, bool);
    type ResponseData = ExecutePermissionActionResponse;

    async fn create(
        ctx: &mut impl ActionContext,
        (key, permission_id, enforce): Self::Params,
    ) -> anyhow::Result<Self> {
        let key = get_key(&key)?;
        let (_, keypair) = ctx.decrypt(&key)?;

        let permission_id = H256::from_str(&permission_id)?;

        Ok(Self {
            key: keypair,
            permission_id,
            enforce,
        })
    }

    async fn execute(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;
            if !self.enforce {
                client
                    .permission0()
                    .calls()
                    .execute_permission_wait(self.permission_id, self.key.clone())
                    .await?;
            } else {
                client
                    .permission0()
                    .calls()
                    .enforcement_execute_permission_wait(self.permission_id, self.key.clone())
                    .await?;
            }
        } else {
            let client = TorusClient::for_testnet().await?;
            if !self.enforce {
                client
                    .permission0()
                    .calls()
                    .execute_permission_wait(self.permission_id, self.key.clone())
                    .await?;
            } else {
                client
                    .permission0()
                    .calls()
                    .enforcement_execute_permission_wait(self.permission_id, self.key.clone())
                    .await?;
            }
        }

        Ok(ExecutePermissionActionResponse)
    }

    async fn estimate_fee(&self, ctx: &mut impl ActionContext) -> anyhow::Result<u128> {
        let fee = if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;
            if !self.enforce {
                client
                    .permission0()
                    .calls()
                    .execute_permission_fee(self.permission_id, self.key.clone())
                    .await?
            } else {
                client
                    .permission0()
                    .calls()
                    .enforcement_execute_permission_fee(self.permission_id, self.key.clone())
                    .await?
            }
        } else {
            let client = TorusClient::for_testnet().await?;
            if !self.enforce {
                client
                    .permission0()
                    .calls()
                    .execute_permission_fee(self.permission_id, self.key.clone())
                    .await?
            } else {
                client
                    .permission0()
                    .calls()
                    .enforcement_execute_permission_fee(self.permission_id, self.key.clone())
                    .await?
            }
        };

        Ok(fee)
    }

    async fn get_changes(
        &self,
        ctx: &mut impl ActionContext,
    ) -> anyhow::Result<Option<action::Changes>> {
        let fee = self.estimate_fee(ctx).await?;

        Ok(Some(Changes {
            changes: vec![format!(
                "Execute the permission {}",
                self.permission_id.to_string()
            )],
            fee: Some(fee),
        }))
    }
}

#[derive(serde::Serialize)]
pub struct ExecutePermissionActionResponse;

impl Display for ExecutePermissionActionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Permission executed successfully!")
    }
}

pub struct SetPermissionAccumulationAction {
    pub key: Keypair,
    pub permission_id: H256,
    pub accumulating: bool,
}

impl Action for SetPermissionAccumulationAction {
    type Params = (String, String, bool);
    type ResponseData = SetPermissionAccumulationActionResponse;

    async fn create(
        ctx: &mut impl ActionContext,
        (key, permission_id, accumulating): Self::Params,
    ) -> anyhow::Result<Self> {
        let key = get_key(&key)?;
        let (_, keypair) = ctx.decrypt(&key)?;

        let permission_id = H256::from_str(&permission_id)?;

        Ok(Self {
            key: keypair,
            permission_id,
            accumulating,
        })
    }

    async fn execute(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;
            client
                .permission0()
                .calls()
                .toggle_permission_accumulation_wait(
                    self.permission_id,
                    self.accumulating,
                    self.key.clone(),
                )
                .await?;
        } else {
            let client = TorusClient::for_testnet().await?;
            client
                .permission0()
                .calls()
                .toggle_permission_accumulation_wait(
                    self.permission_id,
                    self.accumulating,
                    self.key.clone(),
                )
                .await?;
        }

        Ok(SetPermissionAccumulationActionResponse)
    }

    async fn estimate_fee(&self, ctx: &mut impl ActionContext) -> anyhow::Result<u128> {
        let fee = if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;
            client
                .permission0()
                .calls()
                .toggle_permission_accumulation_fee(
                    self.permission_id,
                    self.accumulating,
                    self.key.clone(),
                )
                .await?
        } else {
            let client = TorusClient::for_testnet().await?;
            client
                .permission0()
                .calls()
                .toggle_permission_accumulation_fee(
                    self.permission_id,
                    self.accumulating,
                    self.key.clone(),
                )
                .await?
        };

        Ok(fee)
    }

    async fn get_changes(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Option<Changes>> {
        let fee = self.estimate_fee(ctx).await?;

        Ok(Some(Changes {
            changes: vec![format!(
                "{} accumulating on the permission {}",
                if self.accumulating { "Start" } else { "Stop" },
                self.permission_id.to_string()
            )],
            fee: Some(fee),
        }))
    }
}

#[derive(serde::Serialize)]
pub struct SetPermissionAccumulationActionResponse;

impl Display for SetPermissionAccumulationActionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Permission accumulation set successfully!")
    }
}

pub struct SetPermissionEnforcementAuthorityAction {
    pub key: Keypair,
    pub permission_id: H256,
    pub enforcement_authority: Option<(Vec<AccountId32>, u32)>,
}

impl Action for SetPermissionEnforcementAuthorityAction {
    type Params = (String, String, Option<(Vec<String>, u32)>);
    type ResponseData = SetPermissionEnforcementAuthorityActionResponse;

    async fn create(
        ctx: &mut impl ActionContext,
        (key, permission_id, enforcement_authority): Self::Params,
    ) -> anyhow::Result<Self> {
        let key = get_key(&key)?;
        let (_, keypair) = ctx.decrypt(&key)?;

        let permission_id = H256::from_str(&permission_id)?;

        let enforcement_authority = match enforcement_authority {
            Some((vec, votes)) => Some((
                vec.iter()
                    .map(|acc| get_account(acc))
                    .collect::<anyhow::Result<Vec<_>>>()?,
                votes,
            )),
            None => None,
        };

        Ok(Self {
            key: keypair,
            permission_id,
            enforcement_authority,
        })
    }

    async fn execute(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Self::ResponseData> {
        if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;
            client
                .permission0()
                .calls()
                .set_enforcement_authority_wait(
                    self.permission_id,
                    match &self.enforcement_authority {
                        Some((accounts, votes)) => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::EnforcementAuthority::ControlledBy { controllers: torus_client::interfaces::mainnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(accounts.clone()), required_votes: *votes },
                        None => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::EnforcementAuthority::None,
                    },
                    self.key.clone(),
                )
                .await?;
        } else {
            let client = TorusClient::for_testnet().await?;
            client
                .permission0()
                .calls()
                .set_enforcement_authority_wait(
                    self.permission_id,
                    match &self.enforcement_authority {
                        Some((accounts, votes)) => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::EnforcementAuthority::ControlledBy { controllers: torus_client::interfaces::testnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(accounts.clone()), required_votes: *votes },
                        None => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::EnforcementAuthority::None,
                    },
                    self.key.clone(),
                )
                .await?;
        }

        Ok(SetPermissionEnforcementAuthorityActionResponse)
    }

    async fn estimate_fee(&self, ctx: &mut impl ActionContext) -> anyhow::Result<u128> {
        let fee = if ctx.is_mainnet() {
            let client = TorusClient::for_mainnet().await?;
            client
                .permission0()
                .calls()
                .set_enforcement_authority_fee(
                    self.permission_id,
                    match &self.enforcement_authority {
                        Some((accounts, votes)) => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::EnforcementAuthority::ControlledBy { controllers: torus_client::interfaces::mainnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(accounts.clone()), required_votes: *votes },
                        None => torus_client::interfaces::mainnet::api::runtime_types::pallet_permission0::permission::EnforcementAuthority::None,
                    },
                    self.key.clone(),
                )
                .await?
        } else {
            let client = TorusClient::for_testnet().await?;
            client
                .permission0()
                .calls()
                .set_enforcement_authority_fee(
                    self.permission_id,
                    match &self.enforcement_authority {
                        Some((accounts, votes)) => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::EnforcementAuthority::ControlledBy { controllers: torus_client::interfaces::testnet::api::runtime_types::bounded_collections::bounded_vec::BoundedVec(accounts.clone()), required_votes: *votes },
                        None => torus_client::interfaces::testnet::api::runtime_types::pallet_permission0::permission::EnforcementAuthority::None,
                    },
                    self.key.clone(),
                )
                .await?
        };

        Ok(fee)
    }

    async fn get_changes(&self, ctx: &mut impl ActionContext) -> anyhow::Result<Option<Changes>> {
        let fee = self.estimate_fee(ctx).await?;

        let changes = match &self.enforcement_authority {
            Some((accounts, votes)) => vec![
                format!(
                    "Add {} accounts as controllers on permission {}",
                    accounts.len(),
                    self.permission_id.to_string()
                ),
                format!("Set the required votes to {}", votes),
            ],
            None => vec![format!(
                "Remove enforcement authority from permission {}",
                self.permission_id.to_string()
            )],
        };

        Ok(Some(Changes {
            changes,
            fee: Some(fee),
        }))
    }
}

#[derive(serde::Serialize)]
pub struct SetPermissionEnforcementAuthorityActionResponse;

impl Display for SetPermissionEnforcementAuthorityActionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Enforce authority set successfully!")
    }
}
