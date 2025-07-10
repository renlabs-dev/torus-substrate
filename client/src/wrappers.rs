//! Generated storage wrapper functions
//!
//! This module provides ergonomic access to Substrate storage items.
//! Functions are automatically generated from the subxt API metadata.
#![allow(dead_code)]
use codec::Decode;
use futures::{Stream, StreamExt, TryStreamExt};
use std::marker::PhantomData;
use subxt::{utils::H256, OnlineClient, PolkadotConfig};
pub struct AuraClient<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) _pd: PhantomData<C>,
}
impl<C: crate::chain::Chain> crate::client::TorusClient<C> {
    pub fn aura(&self) -> AuraClient<C> {
        AuraClient {
            client: self.client.clone(),
            _pd: PhantomData,
        }
    }
}
pub struct AuraStorage<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) block: Option<H256>,
    pub(crate) _pd: PhantomData<C>,
}
pub struct AuraCalls<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) _pd: PhantomData<C>,
}
pub struct BalancesClient<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) _pd: PhantomData<C>,
}
impl<C: crate::chain::Chain> crate::client::TorusClient<C> {
    pub fn balances(&self) -> BalancesClient<C> {
        BalancesClient {
            client: self.client.clone(),
            _pd: PhantomData,
        }
    }
}
pub struct BalancesStorage<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) block: Option<H256>,
    pub(crate) _pd: PhantomData<C>,
}
pub struct BalancesCalls<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) _pd: PhantomData<C>,
}
pub struct Emission0Client<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) _pd: PhantomData<C>,
}
impl<C: crate::chain::Chain> crate::client::TorusClient<C> {
    pub fn emission0(&self) -> Emission0Client<C> {
        Emission0Client {
            client: self.client.clone(),
            _pd: PhantomData,
        }
    }
}
pub struct Emission0Storage<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) block: Option<H256>,
    pub(crate) _pd: PhantomData<C>,
}
pub struct Emission0Calls<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) _pd: PhantomData<C>,
}
pub struct EthereumClient<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) _pd: PhantomData<C>,
}
impl<C: crate::chain::Chain> crate::client::TorusClient<C> {
    pub fn ethereum(&self) -> EthereumClient<C> {
        EthereumClient {
            client: self.client.clone(),
            _pd: PhantomData,
        }
    }
}
pub struct EthereumStorage<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) block: Option<H256>,
    pub(crate) _pd: PhantomData<C>,
}
pub struct EthereumCalls<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) _pd: PhantomData<C>,
}
pub struct EvmClient<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) _pd: PhantomData<C>,
}
impl<C: crate::chain::Chain> crate::client::TorusClient<C> {
    pub fn evm(&self) -> EvmClient<C> {
        EvmClient {
            client: self.client.clone(),
            _pd: PhantomData,
        }
    }
}
pub struct EvmStorage<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) block: Option<H256>,
    pub(crate) _pd: PhantomData<C>,
}
pub struct EvmCalls<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) _pd: PhantomData<C>,
}
pub struct FaucetClient<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) _pd: PhantomData<C>,
}
impl<C: crate::chain::Chain> crate::client::TorusClient<C> {
    pub fn faucet(&self) -> FaucetClient<C> {
        FaucetClient {
            client: self.client.clone(),
            _pd: PhantomData,
        }
    }
}
pub struct FaucetStorage<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) block: Option<H256>,
    pub(crate) _pd: PhantomData<C>,
}
pub struct FaucetCalls<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) _pd: PhantomData<C>,
}
pub struct GovernanceClient<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) _pd: PhantomData<C>,
}
impl<C: crate::chain::Chain> crate::client::TorusClient<C> {
    pub fn governance(&self) -> GovernanceClient<C> {
        GovernanceClient {
            client: self.client.clone(),
            _pd: PhantomData,
        }
    }
}
pub struct GovernanceStorage<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) block: Option<H256>,
    pub(crate) _pd: PhantomData<C>,
}
pub struct GovernanceCalls<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) _pd: PhantomData<C>,
}
pub struct GrandpaClient<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) _pd: PhantomData<C>,
}
impl<C: crate::chain::Chain> crate::client::TorusClient<C> {
    pub fn grandpa(&self) -> GrandpaClient<C> {
        GrandpaClient {
            client: self.client.clone(),
            _pd: PhantomData,
        }
    }
}
pub struct GrandpaStorage<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) block: Option<H256>,
    pub(crate) _pd: PhantomData<C>,
}
pub struct GrandpaCalls<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) _pd: PhantomData<C>,
}
pub struct MultisigClient<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) _pd: PhantomData<C>,
}
impl<C: crate::chain::Chain> crate::client::TorusClient<C> {
    pub fn multisig(&self) -> MultisigClient<C> {
        MultisigClient {
            client: self.client.clone(),
            _pd: PhantomData,
        }
    }
}
pub struct MultisigStorage<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) block: Option<H256>,
    pub(crate) _pd: PhantomData<C>,
}
pub struct MultisigCalls<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) _pd: PhantomData<C>,
}
pub struct Permission0Client<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) _pd: PhantomData<C>,
}
impl<C: crate::chain::Chain> crate::client::TorusClient<C> {
    pub fn permission0(&self) -> Permission0Client<C> {
        Permission0Client {
            client: self.client.clone(),
            _pd: PhantomData,
        }
    }
}
pub struct Permission0Storage<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) block: Option<H256>,
    pub(crate) _pd: PhantomData<C>,
}
pub struct Permission0Calls<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) _pd: PhantomData<C>,
}
pub struct SudoClient<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) _pd: PhantomData<C>,
}
impl<C: crate::chain::Chain> crate::client::TorusClient<C> {
    pub fn sudo(&self) -> SudoClient<C> {
        SudoClient {
            client: self.client.clone(),
            _pd: PhantomData,
        }
    }
}
pub struct SudoStorage<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) block: Option<H256>,
    pub(crate) _pd: PhantomData<C>,
}
pub struct SudoCalls<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) _pd: PhantomData<C>,
}
pub struct SystemClient<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) _pd: PhantomData<C>,
}
impl<C: crate::chain::Chain> crate::client::TorusClient<C> {
    pub fn system(&self) -> SystemClient<C> {
        SystemClient {
            client: self.client.clone(),
            _pd: PhantomData,
        }
    }
}
pub struct SystemStorage<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) block: Option<H256>,
    pub(crate) _pd: PhantomData<C>,
}
pub struct SystemCalls<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) _pd: PhantomData<C>,
}
pub struct TimestampClient<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) _pd: PhantomData<C>,
}
impl<C: crate::chain::Chain> crate::client::TorusClient<C> {
    pub fn timestamp(&self) -> TimestampClient<C> {
        TimestampClient {
            client: self.client.clone(),
            _pd: PhantomData,
        }
    }
}
pub struct TimestampStorage<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) block: Option<H256>,
    pub(crate) _pd: PhantomData<C>,
}
pub struct TimestampCalls<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) _pd: PhantomData<C>,
}
pub struct Torus0Client<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) _pd: PhantomData<C>,
}
impl<C: crate::chain::Chain> crate::client::TorusClient<C> {
    pub fn torus0(&self) -> Torus0Client<C> {
        Torus0Client {
            client: self.client.clone(),
            _pd: PhantomData,
        }
    }
}
pub struct Torus0Storage<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) block: Option<H256>,
    pub(crate) _pd: PhantomData<C>,
}
pub struct Torus0Calls<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) _pd: PhantomData<C>,
}
pub struct TransactionPaymentClient<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) _pd: PhantomData<C>,
}
impl<C: crate::chain::Chain> crate::client::TorusClient<C> {
    pub fn transaction_payment(&self) -> TransactionPaymentClient<C> {
        TransactionPaymentClient {
            client: self.client.clone(),
            _pd: PhantomData,
        }
    }
}
pub struct TransactionPaymentStorage<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) block: Option<H256>,
    pub(crate) _pd: PhantomData<C>,
}
pub struct TransactionPaymentCalls<C: crate::chain::Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) _pd: PhantomData<C>,
}
#[cfg(feature = "mainnet")]
pub mod mainnet {
    use super::*;
    use crate::interfaces::mainnet::api::runtime_types;
    pub mod system {
        use super::*;
        impl SystemClient<crate::chain::MainNet> {
            pub fn storage(&self) -> SystemStorage<crate::chain::MainNet> {
                SystemStorage {
                    client: self.client.clone(),
                    block: None,
                    _pd: PhantomData,
                }
            }
            pub fn storage_at(&self, block_hash: H256) -> SystemStorage<crate::chain::MainNet> {
                SystemStorage {
                    client: self.client.clone(),
                    block: Some(block_hash),
                    _pd: PhantomData,
                }
            }
        }
        impl SystemStorage<crate::chain::MainNet> {
            pub async fn digest(
                &self,
            ) -> crate::Result<Option<runtime_types::sp_runtime::generic::digest::Digest>>
            {
                let call = crate::interfaces::mainnet::api::storage().system().digest();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn parent_hash(
                &self,
            ) -> crate::Result<Option<::subxt::ext::subxt_core::utils::H256>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .system()
                    .parent_hash();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn event_topics_get(
                &self,
                key: &::subxt::ext::subxt_core::utils::H256,
            ) -> crate::Result<
                Option<
                    ::subxt::ext::subxt_core::alloc::vec::Vec<(
                        ::core::primitive::u64,
                        ::core::primitive::u32,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .system()
                    .event_topics(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn event_topics_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::subxt::ext::subxt_core::utils::H256,
                        ::subxt::ext::subxt_core::alloc::vec::Vec<(
                            ::core::primitive::u64,
                            ::core::primitive::u32,
                        )>,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .system()
                    .event_topics_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::subxt::ext::subxt_core::utils::H256 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn event_count(&self) -> crate::Result<Option<::core::primitive::u32>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .system()
                    .event_count();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn events(
                &self,
            ) -> crate::Result<
                Option<
                    ::subxt::ext::subxt_core::alloc::vec::Vec<
                        runtime_types::frame_system::EventRecord<
                            runtime_types::torus_runtime::RuntimeEvent,
                            ::subxt::ext::subxt_core::utils::H256,
                        >,
                    >,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage().system().events();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn last_runtime_upgrade(
                &self,
            ) -> crate::Result<Option<runtime_types::frame_system::LastRuntimeUpgradeInfo>>
            {
                let call = crate::interfaces::mainnet::api::storage()
                    .system()
                    .last_runtime_upgrade();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn number(&self) -> crate::Result<Option<::core::primitive::u64>> {
                let call = crate::interfaces::mainnet::api::storage().system().number();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn block_hash_get(
                &self,
                key: &::core::primitive::u64,
            ) -> crate::Result<Option<::subxt::ext::subxt_core::utils::H256>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .system()
                    .block_hash(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn block_hash_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::core::primitive::u64,
                        ::subxt::ext::subxt_core::utils::H256,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .system()
                    .block_hash_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::core::primitive::u64 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn execution_phase(
                &self,
            ) -> crate::Result<Option<runtime_types::frame_system::Phase>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .system()
                    .execution_phase();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn account_get(
                &self,
                key: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<
                Option<
                    runtime_types::frame_system::AccountInfo<
                        ::core::primitive::u32,
                        runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
                    >,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .system()
                    .account(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn account_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        runtime_types::frame_system::AccountInfo<
                            ::core::primitive::u32,
                            runtime_types::pallet_balances::types::AccountData<
                                ::core::primitive::u128,
                            >,
                        >,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .system()
                    .account_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::subxt::ext::subxt_core::utils::AccountId32 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn extrinsic_data_get(
                &self,
                key: &::core::primitive::u32,
            ) -> crate::Result<
                Option<::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>>,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .system()
                    .extrinsic_data(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn extrinsic_data_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::core::primitive::u32,
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .system()
                    .extrinsic_data_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::core::primitive::u32 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn all_extrinsics_len(
                &self,
            ) -> crate::Result<Option<::core::primitive::u32>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .system()
                    .all_extrinsics_len();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn upgraded_to_triple_ref_count(
                &self,
            ) -> crate::Result<Option<::core::primitive::bool>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .system()
                    .upgraded_to_triple_ref_count();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn authorized_upgrade(
                &self,
            ) -> crate::Result<Option<runtime_types::frame_system::CodeUpgradeAuthorization>>
            {
                let call = crate::interfaces::mainnet::api::storage()
                    .system()
                    .authorized_upgrade();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn extrinsic_count(&self) -> crate::Result<Option<::core::primitive::u32>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .system()
                    .extrinsic_count();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn inherents_applied(
                &self,
            ) -> crate::Result<Option<::core::primitive::bool>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .system()
                    .inherents_applied();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn block_weight(
                &self,
            ) -> crate::Result<
                Option<
                    runtime_types::frame_support::dispatch::PerDispatchClass<
                        runtime_types::sp_weights::weight_v2::Weight,
                    >,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .system()
                    .block_weight();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn upgraded_to_u32_ref_count(
                &self,
            ) -> crate::Result<Option<::core::primitive::bool>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .system()
                    .upgraded_to_u32_ref_count();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
        }
        impl SystemClient<crate::chain::MainNet> {
            pub fn calls(&self) -> SystemCalls<crate::chain::MainNet> {
                SystemCalls {
                    client: self.client.clone(),
                    _pd: PhantomData,
                }
            }
        }
        impl SystemCalls<crate::chain::MainNet> {
            pub async fn remark(
                &self,
                remark: crate::interfaces::mainnet::api::system::calls::types::remark::Remark,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .system()
                    .remark(remark);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn remark_wait(
                &self,
                remark: crate::interfaces::mainnet::api::system::calls::types::remark::Remark,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .system()
                    .remark(remark);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn set_heap_pages(
                &self,
                pages: crate::interfaces::mainnet::api::system::calls::types::set_heap_pages::Pages,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .system()
                    .set_heap_pages(pages);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn set_heap_pages_wait(
                &self,
                pages: crate::interfaces::mainnet::api::system::calls::types::set_heap_pages::Pages,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .system()
                    .set_heap_pages(pages);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn set_code(
                &self,
                code: crate::interfaces::mainnet::api::system::calls::types::set_code::Code,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .system()
                    .set_code(code);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn set_code_wait(
                &self,
                code: crate::interfaces::mainnet::api::system::calls::types::set_code::Code,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .system()
                    .set_code(code);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn set_code_without_checks(
                &self,
                code: crate::interfaces::mainnet::api::system::calls::types::set_code_without_checks::Code,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .system()
                    .set_code_without_checks(code);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn set_code_without_checks_wait(
                &self,
                code: crate::interfaces::mainnet::api::system::calls::types::set_code_without_checks::Code,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .system()
                    .set_code_without_checks(code);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn set_storage(
                &self,
                items: crate::interfaces::mainnet::api::system::calls::types::set_storage::Items,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .system()
                    .set_storage(items);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn set_storage_wait(
                &self,
                items: crate::interfaces::mainnet::api::system::calls::types::set_storage::Items,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .system()
                    .set_storage(items);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn kill_storage(
                &self,
                keys: crate::interfaces::mainnet::api::system::calls::types::kill_storage::Keys,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .system()
                    .kill_storage(keys);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn kill_storage_wait(
                &self,
                keys: crate::interfaces::mainnet::api::system::calls::types::kill_storage::Keys,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .system()
                    .kill_storage(keys);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn kill_prefix(
                &self,
                prefix: crate::interfaces::mainnet::api::system::calls::types::kill_prefix::Prefix,
                subkeys: crate::interfaces::mainnet::api::system::calls::types::kill_prefix::Subkeys,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .system()
                    .kill_prefix(prefix, subkeys);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn kill_prefix_wait(
                &self,
                prefix: crate::interfaces::mainnet::api::system::calls::types::kill_prefix::Prefix,
                subkeys: crate::interfaces::mainnet::api::system::calls::types::kill_prefix::Subkeys,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .system()
                    .kill_prefix(prefix, subkeys);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn remark_with_event(
                &self,
                remark: crate::interfaces::mainnet::api::system::calls::types::remark_with_event::Remark,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .system()
                    .remark_with_event(remark);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn remark_with_event_wait(
                &self,
                remark: crate::interfaces::mainnet::api::system::calls::types::remark_with_event::Remark,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .system()
                    .remark_with_event(remark);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn do_task(
                &self,
                task: crate::interfaces::mainnet::api::system::calls::types::do_task::Task,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx().system().do_task(task);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn do_task_wait(
                &self,
                task: crate::interfaces::mainnet::api::system::calls::types::do_task::Task,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx().system().do_task(task);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn authorize_upgrade(
                &self,
                code_hash: crate::interfaces::mainnet::api::system::calls::types::authorize_upgrade::CodeHash,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .system()
                    .authorize_upgrade(code_hash);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn authorize_upgrade_wait(
                &self,
                code_hash: crate::interfaces::mainnet::api::system::calls::types::authorize_upgrade::CodeHash,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .system()
                    .authorize_upgrade(code_hash);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn authorize_upgrade_without_checks(
                &self,
                code_hash: crate::interfaces::mainnet::api::system::calls::types::authorize_upgrade_without_checks::CodeHash,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .system()
                    .authorize_upgrade_without_checks(code_hash);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn authorize_upgrade_without_checks_wait(
                &self,
                code_hash: crate::interfaces::mainnet::api::system::calls::types::authorize_upgrade_without_checks::CodeHash,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .system()
                    .authorize_upgrade_without_checks(code_hash);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn apply_authorized_upgrade(
                &self,
                code: crate::interfaces::mainnet::api::system::calls::types::apply_authorized_upgrade::Code,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .system()
                    .apply_authorized_upgrade(code);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn apply_authorized_upgrade_wait(
                &self,
                code: crate::interfaces::mainnet::api::system::calls::types::apply_authorized_upgrade::Code,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .system()
                    .apply_authorized_upgrade(code);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
        }
    }
    pub mod timestamp {
        use super::*;
        impl TimestampClient<crate::chain::MainNet> {
            pub fn storage(&self) -> TimestampStorage<crate::chain::MainNet> {
                TimestampStorage {
                    client: self.client.clone(),
                    block: None,
                    _pd: PhantomData,
                }
            }
            pub fn storage_at(&self, block_hash: H256) -> TimestampStorage<crate::chain::MainNet> {
                TimestampStorage {
                    client: self.client.clone(),
                    block: Some(block_hash),
                    _pd: PhantomData,
                }
            }
        }
        impl TimestampStorage<crate::chain::MainNet> {
            pub async fn did_update(&self) -> crate::Result<Option<::core::primitive::bool>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .timestamp()
                    .did_update();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn now(&self) -> crate::Result<Option<::core::primitive::u64>> {
                let call = crate::interfaces::mainnet::api::storage().timestamp().now();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
        }
        impl TimestampClient<crate::chain::MainNet> {
            pub fn calls(&self) -> TimestampCalls<crate::chain::MainNet> {
                TimestampCalls {
                    client: self.client.clone(),
                    _pd: PhantomData,
                }
            }
        }
        impl TimestampCalls<crate::chain::MainNet> {
            pub async fn set(
                &self,
                now: crate::interfaces::mainnet::api::timestamp::calls::types::set::Now,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx().timestamp().set(now);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn set_wait(
                &self,
                now: crate::interfaces::mainnet::api::timestamp::calls::types::set::Now,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx().timestamp().set(now);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
        }
    }
    pub mod aura {
        use super::*;
        impl AuraClient<crate::chain::MainNet> {
            pub fn storage(&self) -> AuraStorage<crate::chain::MainNet> {
                AuraStorage {
                    client: self.client.clone(),
                    block: None,
                    _pd: PhantomData,
                }
            }
            pub fn storage_at(&self, block_hash: H256) -> AuraStorage<crate::chain::MainNet> {
                AuraStorage {
                    client: self.client.clone(),
                    block: Some(block_hash),
                    _pd: PhantomData,
                }
            }
        }
        impl AuraStorage<crate::chain::MainNet> {
            pub async fn authorities(
                &self,
            ) -> crate::Result<
                Option<
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
                    >,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .aura()
                    .authorities();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn current_slot(
                &self,
            ) -> crate::Result<Option<runtime_types::sp_consensus_slots::Slot>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .aura()
                    .current_slot();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
        }
    }
    pub mod grandpa {
        use super::*;
        impl GrandpaClient<crate::chain::MainNet> {
            pub fn storage(&self) -> GrandpaStorage<crate::chain::MainNet> {
                GrandpaStorage {
                    client: self.client.clone(),
                    block: None,
                    _pd: PhantomData,
                }
            }
            pub fn storage_at(&self, block_hash: H256) -> GrandpaStorage<crate::chain::MainNet> {
                GrandpaStorage {
                    client: self.client.clone(),
                    block: Some(block_hash),
                    _pd: PhantomData,
                }
            }
        }
        impl GrandpaStorage<crate::chain::MainNet> {
            pub async fn state(
                &self,
            ) -> crate::Result<
                Option<runtime_types::pallet_grandpa::StoredState<::core::primitive::u64>>,
            > {
                let call = crate::interfaces::mainnet::api::storage().grandpa().state();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn next_forced(&self) -> crate::Result<Option<::core::primitive::u64>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .grandpa()
                    .next_forced();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn stalled(
                &self,
            ) -> crate::Result<Option<(::core::primitive::u64, ::core::primitive::u64)>>
            {
                let call = crate::interfaces::mainnet::api::storage()
                    .grandpa()
                    .stalled();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn current_set_id(&self) -> crate::Result<Option<::core::primitive::u64>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .grandpa()
                    .current_set_id();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn set_id_session_get(
                &self,
                key: &::core::primitive::u64,
            ) -> crate::Result<Option<::core::primitive::u32>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .grandpa()
                    .set_id_session(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn set_id_session_iter(
                &self,
            ) -> crate::Result<
                impl Stream<Item = crate::Result<(::core::primitive::u64, ::core::primitive::u32)>>,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .grandpa()
                    .set_id_session_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::core::primitive::u64 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn authorities(
                &self,
            ) -> crate::Result<
                Option<
                    runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_consensus_grandpa::app::Public,
                        ::core::primitive::u64,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .grandpa()
                    .authorities();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn pending_change(
                &self,
            ) -> crate::Result<
                Option<runtime_types::pallet_grandpa::StoredPendingChange<::core::primitive::u64>>,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .grandpa()
                    .pending_change();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
        }
        impl GrandpaClient<crate::chain::MainNet> {
            pub fn calls(&self) -> GrandpaCalls<crate::chain::MainNet> {
                GrandpaCalls {
                    client: self.client.clone(),
                    _pd: PhantomData,
                }
            }
        }
        impl GrandpaCalls<crate::chain::MainNet> {
            pub async fn report_equivocation(
                &self,
                equivocation_proof: crate::interfaces::mainnet::api::grandpa::calls::types::report_equivocation::EquivocationProof,
                key_owner_proof: crate::interfaces::mainnet::api::grandpa::calls::types::report_equivocation::KeyOwnerProof,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .grandpa()
                    .report_equivocation(equivocation_proof, key_owner_proof);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn report_equivocation_wait(
                &self,
                equivocation_proof: crate::interfaces::mainnet::api::grandpa::calls::types::report_equivocation::EquivocationProof,
                key_owner_proof: crate::interfaces::mainnet::api::grandpa::calls::types::report_equivocation::KeyOwnerProof,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .grandpa()
                    .report_equivocation(equivocation_proof, key_owner_proof);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn report_equivocation_unsigned(
                &self,
                equivocation_proof: crate::interfaces::mainnet::api::grandpa::calls::types::report_equivocation_unsigned::EquivocationProof,
                key_owner_proof: crate::interfaces::mainnet::api::grandpa::calls::types::report_equivocation_unsigned::KeyOwnerProof,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .grandpa()
                    .report_equivocation_unsigned(equivocation_proof, key_owner_proof);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn report_equivocation_unsigned_wait(
                &self,
                equivocation_proof: crate::interfaces::mainnet::api::grandpa::calls::types::report_equivocation_unsigned::EquivocationProof,
                key_owner_proof: crate::interfaces::mainnet::api::grandpa::calls::types::report_equivocation_unsigned::KeyOwnerProof,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .grandpa()
                    .report_equivocation_unsigned(equivocation_proof, key_owner_proof);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn note_stalled(
                &self,
                delay: crate::interfaces::mainnet::api::grandpa::calls::types::note_stalled::Delay,
                best_finalized_block_number: crate::interfaces::mainnet::api::grandpa::calls::types::note_stalled::BestFinalizedBlockNumber,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .grandpa()
                    .note_stalled(delay, best_finalized_block_number);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn note_stalled_wait(
                &self,
                delay: crate::interfaces::mainnet::api::grandpa::calls::types::note_stalled::Delay,
                best_finalized_block_number: crate::interfaces::mainnet::api::grandpa::calls::types::note_stalled::BestFinalizedBlockNumber,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .grandpa()
                    .note_stalled(delay, best_finalized_block_number);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
        }
    }
    pub mod balances {
        use super::*;
        impl BalancesClient<crate::chain::MainNet> {
            pub fn storage(&self) -> BalancesStorage<crate::chain::MainNet> {
                BalancesStorage {
                    client: self.client.clone(),
                    block: None,
                    _pd: PhantomData,
                }
            }
            pub fn storage_at(&self, block_hash: H256) -> BalancesStorage<crate::chain::MainNet> {
                BalancesStorage {
                    client: self.client.clone(),
                    block: Some(block_hash),
                    _pd: PhantomData,
                }
            }
        }
        impl BalancesStorage<crate::chain::MainNet> {
            pub async fn total_issuance(&self) -> crate::Result<Option<::core::primitive::u128>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .balances()
                    .total_issuance();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn inactive_issuance(
                &self,
            ) -> crate::Result<Option<::core::primitive::u128>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .balances()
                    .inactive_issuance();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn account_get(
                &self,
                key: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<
                Option<runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>>,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .balances()
                    .account(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn account_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .balances()
                    .account_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::subxt::ext::subxt_core::utils::AccountId32 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn locks_get(
                &self,
                key: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<
                Option<
                    runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::pallet_balances::types::BalanceLock<::core::primitive::u128>,
                    >,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .balances()
                    .locks(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn locks_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
                            runtime_types::pallet_balances::types::BalanceLock<
                                ::core::primitive::u128,
                            >,
                        >,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .balances()
                    .locks_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::subxt::ext::subxt_core::utils::AccountId32 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn reserves_get(
                &self,
                key: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<
                Option<
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::ReserveData<
                            (),
                            ::core::primitive::u128,
                        >,
                    >,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .balances()
                    .reserves(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn reserves_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            runtime_types::pallet_balances::types::ReserveData<
                                (),
                                ::core::primitive::u128,
                            >,
                        >,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .balances()
                    .reserves_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::subxt::ext::subxt_core::utils::AccountId32 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn holds_get(
                &self,
                key: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<
                Option<
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::frame_support::traits::tokens::misc::IdAmount<
                            runtime_types::torus_runtime::RuntimeHoldReason,
                            ::core::primitive::u128,
                        >,
                    >,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .balances()
                    .holds(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn holds_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            runtime_types::frame_support::traits::tokens::misc::IdAmount<
                                runtime_types::torus_runtime::RuntimeHoldReason,
                                ::core::primitive::u128,
                            >,
                        >,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .balances()
                    .holds_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::subxt::ext::subxt_core::utils::AccountId32 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn freezes_get(
                &self,
                key: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<
                Option<
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::frame_support::traits::tokens::misc::IdAmount<
                            (),
                            ::core::primitive::u128,
                        >,
                    >,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .balances()
                    .freezes(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn freezes_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            runtime_types::frame_support::traits::tokens::misc::IdAmount<
                                (),
                                ::core::primitive::u128,
                            >,
                        >,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .balances()
                    .freezes_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::subxt::ext::subxt_core::utils::AccountId32 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
        }
        impl BalancesClient<crate::chain::MainNet> {
            pub fn calls(&self) -> BalancesCalls<crate::chain::MainNet> {
                BalancesCalls {
                    client: self.client.clone(),
                    _pd: PhantomData,
                }
            }
        }
        impl BalancesCalls<crate::chain::MainNet> {
            pub async fn transfer_allow_death(
                &self,
                dest: crate::interfaces::mainnet::api::balances::calls::types::transfer_allow_death::Dest,
                value: crate::interfaces::mainnet::api::balances::calls::types::transfer_allow_death::Value,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .balances()
                    .transfer_allow_death(dest, value);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn transfer_allow_death_wait(
                &self,
                dest: crate::interfaces::mainnet::api::balances::calls::types::transfer_allow_death::Dest,
                value: crate::interfaces::mainnet::api::balances::calls::types::transfer_allow_death::Value,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .balances()
                    .transfer_allow_death(dest, value);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn force_transfer(
                &self,
                source: crate::interfaces::mainnet::api::balances::calls::types::force_transfer::Source,
                dest: crate::interfaces::mainnet::api::balances::calls::types::force_transfer::Dest,
                value: crate::interfaces::mainnet::api::balances::calls::types::force_transfer::Value,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .balances()
                    .force_transfer(source, dest, value);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn force_transfer_wait(
                &self,
                source: crate::interfaces::mainnet::api::balances::calls::types::force_transfer::Source,
                dest: crate::interfaces::mainnet::api::balances::calls::types::force_transfer::Dest,
                value: crate::interfaces::mainnet::api::balances::calls::types::force_transfer::Value,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .balances()
                    .force_transfer(source, dest, value);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn transfer_keep_alive(
                &self,
                dest: crate::interfaces::mainnet::api::balances::calls::types::transfer_keep_alive::Dest,
                value: crate::interfaces::mainnet::api::balances::calls::types::transfer_keep_alive::Value,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .balances()
                    .transfer_keep_alive(dest, value);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn transfer_keep_alive_wait(
                &self,
                dest: crate::interfaces::mainnet::api::balances::calls::types::transfer_keep_alive::Dest,
                value: crate::interfaces::mainnet::api::balances::calls::types::transfer_keep_alive::Value,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .balances()
                    .transfer_keep_alive(dest, value);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn transfer_all(
                &self,
                dest: crate::interfaces::mainnet::api::balances::calls::types::transfer_all::Dest,
                keep_alive: crate::interfaces::mainnet::api::balances::calls::types::transfer_all::KeepAlive,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .balances()
                    .transfer_all(dest, keep_alive);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn transfer_all_wait(
                &self,
                dest: crate::interfaces::mainnet::api::balances::calls::types::transfer_all::Dest,
                keep_alive: crate::interfaces::mainnet::api::balances::calls::types::transfer_all::KeepAlive,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .balances()
                    .transfer_all(dest, keep_alive);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn force_unreserve(
                &self,
                who: crate::interfaces::mainnet::api::balances::calls::types::force_unreserve::Who,
                amount: crate::interfaces::mainnet::api::balances::calls::types::force_unreserve::Amount,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .balances()
                    .force_unreserve(who, amount);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn force_unreserve_wait(
                &self,
                who: crate::interfaces::mainnet::api::balances::calls::types::force_unreserve::Who,
                amount: crate::interfaces::mainnet::api::balances::calls::types::force_unreserve::Amount,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .balances()
                    .force_unreserve(who, amount);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn upgrade_accounts(
                &self,
                who: crate::interfaces::mainnet::api::balances::calls::types::upgrade_accounts::Who,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .balances()
                    .upgrade_accounts(who);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn upgrade_accounts_wait(
                &self,
                who: crate::interfaces::mainnet::api::balances::calls::types::upgrade_accounts::Who,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .balances()
                    .upgrade_accounts(who);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn force_set_balance(
                &self,
                who: crate::interfaces::mainnet::api::balances::calls::types::force_set_balance::Who,
                new_free: crate::interfaces::mainnet::api::balances::calls::types::force_set_balance::NewFree,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .balances()
                    .force_set_balance(who, new_free);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn force_set_balance_wait(
                &self,
                who: crate::interfaces::mainnet::api::balances::calls::types::force_set_balance::Who,
                new_free: crate::interfaces::mainnet::api::balances::calls::types::force_set_balance::NewFree,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .balances()
                    .force_set_balance(who, new_free);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn force_adjust_total_issuance(
                &self,
                direction: crate::interfaces::mainnet::api::balances::calls::types::force_adjust_total_issuance::Direction,
                delta: crate::interfaces::mainnet::api::balances::calls::types::force_adjust_total_issuance::Delta,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .balances()
                    .force_adjust_total_issuance(direction, delta);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn force_adjust_total_issuance_wait(
                &self,
                direction: crate::interfaces::mainnet::api::balances::calls::types::force_adjust_total_issuance::Direction,
                delta: crate::interfaces::mainnet::api::balances::calls::types::force_adjust_total_issuance::Delta,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .balances()
                    .force_adjust_total_issuance(direction, delta);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn burn(
                &self,
                value: crate::interfaces::mainnet::api::balances::calls::types::burn::Value,
                keep_alive: crate::interfaces::mainnet::api::balances::calls::types::burn::KeepAlive,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .balances()
                    .burn(value, keep_alive);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn burn_wait(
                &self,
                value: crate::interfaces::mainnet::api::balances::calls::types::burn::Value,
                keep_alive: crate::interfaces::mainnet::api::balances::calls::types::burn::KeepAlive,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .balances()
                    .burn(value, keep_alive);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
        }
    }
    pub mod transaction_payment {
        use super::*;
        impl TransactionPaymentClient<crate::chain::MainNet> {
            pub fn storage(&self) -> TransactionPaymentStorage<crate::chain::MainNet> {
                TransactionPaymentStorage {
                    client: self.client.clone(),
                    block: None,
                    _pd: PhantomData,
                }
            }
            pub fn storage_at(
                &self,
                block_hash: H256,
            ) -> TransactionPaymentStorage<crate::chain::MainNet> {
                TransactionPaymentStorage {
                    client: self.client.clone(),
                    block: Some(block_hash),
                    _pd: PhantomData,
                }
            }
        }
        impl TransactionPaymentStorage<crate::chain::MainNet> {
            pub async fn storage_version(
                &self,
            ) -> crate::Result<Option<runtime_types::pallet_transaction_payment::Releases>>
            {
                let call = crate::interfaces::mainnet::api::storage()
                    .transaction_payment()
                    .storage_version();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn next_fee_multiplier(
                &self,
            ) -> crate::Result<Option<runtime_types::sp_arithmetic::fixed_point::FixedU128>>
            {
                let call = crate::interfaces::mainnet::api::storage()
                    .transaction_payment()
                    .next_fee_multiplier();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
        }
    }
    pub mod sudo {
        use super::*;
        impl SudoClient<crate::chain::MainNet> {
            pub fn storage(&self) -> SudoStorage<crate::chain::MainNet> {
                SudoStorage {
                    client: self.client.clone(),
                    block: None,
                    _pd: PhantomData,
                }
            }
            pub fn storage_at(&self, block_hash: H256) -> SudoStorage<crate::chain::MainNet> {
                SudoStorage {
                    client: self.client.clone(),
                    block: Some(block_hash),
                    _pd: PhantomData,
                }
            }
        }
        impl SudoStorage<crate::chain::MainNet> {
            pub async fn key(
                &self,
            ) -> crate::Result<Option<::subxt::ext::subxt_core::utils::AccountId32>> {
                let call = crate::interfaces::mainnet::api::storage().sudo().key();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
        }
        impl SudoClient<crate::chain::MainNet> {
            pub fn calls(&self) -> SudoCalls<crate::chain::MainNet> {
                SudoCalls {
                    client: self.client.clone(),
                    _pd: PhantomData,
                }
            }
        }
        impl SudoCalls<crate::chain::MainNet> {
            pub async fn sudo(
                &self,
                call: crate::interfaces::mainnet::api::sudo::calls::types::sudo::Call,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx().sudo().sudo(call);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn sudo_wait(
                &self,
                call: crate::interfaces::mainnet::api::sudo::calls::types::sudo::Call,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx().sudo().sudo(call);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn sudo_unchecked_weight(
                &self,
                call: crate::interfaces::mainnet::api::sudo::calls::types::sudo_unchecked_weight::Call,
                weight: crate::interfaces::mainnet::api::sudo::calls::types::sudo_unchecked_weight::Weight,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .sudo()
                    .sudo_unchecked_weight(call, weight);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn sudo_unchecked_weight_wait(
                &self,
                call: crate::interfaces::mainnet::api::sudo::calls::types::sudo_unchecked_weight::Call,
                weight: crate::interfaces::mainnet::api::sudo::calls::types::sudo_unchecked_weight::Weight,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .sudo()
                    .sudo_unchecked_weight(call, weight);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn set_key(
                &self,
                new: crate::interfaces::mainnet::api::sudo::calls::types::set_key::New,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx().sudo().set_key(new);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn set_key_wait(
                &self,
                new: crate::interfaces::mainnet::api::sudo::calls::types::set_key::New,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx().sudo().set_key(new);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn sudo_as(
                &self,
                who: crate::interfaces::mainnet::api::sudo::calls::types::sudo_as::Who,
                call: crate::interfaces::mainnet::api::sudo::calls::types::sudo_as::Call,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .sudo()
                    .sudo_as(who, call);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn sudo_as_wait(
                &self,
                who: crate::interfaces::mainnet::api::sudo::calls::types::sudo_as::Who,
                call: crate::interfaces::mainnet::api::sudo::calls::types::sudo_as::Call,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .sudo()
                    .sudo_as(who, call);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn remove_key(
                &self,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx().sudo().remove_key();
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn remove_key_wait(
                &self,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx().sudo().remove_key();
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
        }
    }
    pub mod multisig {
        use super::*;
        impl MultisigClient<crate::chain::MainNet> {
            pub fn storage(&self) -> MultisigStorage<crate::chain::MainNet> {
                MultisigStorage {
                    client: self.client.clone(),
                    block: None,
                    _pd: PhantomData,
                }
            }
            pub fn storage_at(&self, block_hash: H256) -> MultisigStorage<crate::chain::MainNet> {
                MultisigStorage {
                    client: self.client.clone(),
                    block: Some(block_hash),
                    _pd: PhantomData,
                }
            }
        }
        impl MultisigStorage<crate::chain::MainNet> {
            pub async fn multisigs_get(
                &self,
                key1: &::subxt::ext::subxt_core::utils::AccountId32,
                key2: &[::core::primitive::u8; 32usize],
            ) -> crate::Result<
                Option<
                    runtime_types::pallet_multisig::Multisig<
                        ::core::primitive::u64,
                        ::core::primitive::u128,
                        ::subxt::ext::subxt_core::utils::AccountId32,
                    >,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .multisig()
                    .multisigs(key1, key2);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn multisigs_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        (
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            [::core::primitive::u8; 32usize],
                        ),
                        runtime_types::pallet_multisig::Multisig<
                            ::core::primitive::u64,
                            ::core::primitive::u128,
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        >,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .multisig()
                    .multisigs_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let pair = <(
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            [::core::primitive::u8; 32usize],
                        ) as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((pair, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn multisigs_iter1(
                &self,
                key1: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        (
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            [::core::primitive::u8; 32usize],
                        ),
                        runtime_types::pallet_multisig::Multisig<
                            ::core::primitive::u64,
                            ::core::primitive::u128,
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        >,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .multisig()
                    .multisigs_iter1(key1);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let pair = <(
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            [::core::primitive::u8; 32usize],
                        ) as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((pair, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
        }
        impl MultisigClient<crate::chain::MainNet> {
            pub fn calls(&self) -> MultisigCalls<crate::chain::MainNet> {
                MultisigCalls {
                    client: self.client.clone(),
                    _pd: PhantomData,
                }
            }
        }
        impl MultisigCalls<crate::chain::MainNet> {
            pub async fn as_multi_threshold_1(
                &self,
                other_signatories: crate::interfaces::mainnet::api::multisig::calls::types::as_multi_threshold1::OtherSignatories,
                call: crate::interfaces::mainnet::api::multisig::calls::types::as_multi_threshold1::Call,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .multisig()
                    .as_multi_threshold_1(other_signatories, call);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn as_multi_threshold_1_wait(
                &self,
                other_signatories: crate::interfaces::mainnet::api::multisig::calls::types::as_multi_threshold1::OtherSignatories,
                call: crate::interfaces::mainnet::api::multisig::calls::types::as_multi_threshold1::Call,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .multisig()
                    .as_multi_threshold_1(other_signatories, call);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn as_multi(
                &self,
                threshold: crate::interfaces::mainnet::api::multisig::calls::types::as_multi::Threshold,
                other_signatories: crate::interfaces::mainnet::api::multisig::calls::types::as_multi::OtherSignatories,
                maybe_timepoint: crate::interfaces::mainnet::api::multisig::calls::types::as_multi::MaybeTimepoint,
                call: crate::interfaces::mainnet::api::multisig::calls::types::as_multi::Call,
                max_weight: crate::interfaces::mainnet::api::multisig::calls::types::as_multi::MaxWeight,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx().multisig().as_multi(
                    threshold,
                    other_signatories,
                    maybe_timepoint,
                    call,
                    max_weight,
                );
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn as_multi_wait(
                &self,
                threshold: crate::interfaces::mainnet::api::multisig::calls::types::as_multi::Threshold,
                other_signatories: crate::interfaces::mainnet::api::multisig::calls::types::as_multi::OtherSignatories,
                maybe_timepoint: crate::interfaces::mainnet::api::multisig::calls::types::as_multi::MaybeTimepoint,
                call: crate::interfaces::mainnet::api::multisig::calls::types::as_multi::Call,
                max_weight: crate::interfaces::mainnet::api::multisig::calls::types::as_multi::MaxWeight,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx().multisig().as_multi(
                    threshold,
                    other_signatories,
                    maybe_timepoint,
                    call,
                    max_weight,
                );
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn approve_as_multi(
                &self,
                threshold: crate::interfaces::mainnet::api::multisig::calls::types::approve_as_multi::Threshold,
                other_signatories: crate::interfaces::mainnet::api::multisig::calls::types::approve_as_multi::OtherSignatories,
                maybe_timepoint: crate::interfaces::mainnet::api::multisig::calls::types::approve_as_multi::MaybeTimepoint,
                call_hash: crate::interfaces::mainnet::api::multisig::calls::types::approve_as_multi::CallHash,
                max_weight: crate::interfaces::mainnet::api::multisig::calls::types::approve_as_multi::MaxWeight,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .multisig()
                    .approve_as_multi(
                        threshold,
                        other_signatories,
                        maybe_timepoint,
                        call_hash,
                        max_weight,
                    );
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn approve_as_multi_wait(
                &self,
                threshold: crate::interfaces::mainnet::api::multisig::calls::types::approve_as_multi::Threshold,
                other_signatories: crate::interfaces::mainnet::api::multisig::calls::types::approve_as_multi::OtherSignatories,
                maybe_timepoint: crate::interfaces::mainnet::api::multisig::calls::types::approve_as_multi::MaybeTimepoint,
                call_hash: crate::interfaces::mainnet::api::multisig::calls::types::approve_as_multi::CallHash,
                max_weight: crate::interfaces::mainnet::api::multisig::calls::types::approve_as_multi::MaxWeight,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .multisig()
                    .approve_as_multi(
                        threshold,
                        other_signatories,
                        maybe_timepoint,
                        call_hash,
                        max_weight,
                    );
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn cancel_as_multi(
                &self,
                threshold: crate::interfaces::mainnet::api::multisig::calls::types::cancel_as_multi::Threshold,
                other_signatories: crate::interfaces::mainnet::api::multisig::calls::types::cancel_as_multi::OtherSignatories,
                timepoint: crate::interfaces::mainnet::api::multisig::calls::types::cancel_as_multi::Timepoint,
                call_hash: crate::interfaces::mainnet::api::multisig::calls::types::cancel_as_multi::CallHash,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .multisig()
                    .cancel_as_multi(threshold, other_signatories, timepoint, call_hash);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn cancel_as_multi_wait(
                &self,
                threshold: crate::interfaces::mainnet::api::multisig::calls::types::cancel_as_multi::Threshold,
                other_signatories: crate::interfaces::mainnet::api::multisig::calls::types::cancel_as_multi::OtherSignatories,
                timepoint: crate::interfaces::mainnet::api::multisig::calls::types::cancel_as_multi::Timepoint,
                call_hash: crate::interfaces::mainnet::api::multisig::calls::types::cancel_as_multi::CallHash,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .multisig()
                    .cancel_as_multi(threshold, other_signatories, timepoint, call_hash);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
        }
    }
    pub mod ethereum {
        use super::*;
        impl EthereumClient<crate::chain::MainNet> {
            pub fn storage(&self) -> EthereumStorage<crate::chain::MainNet> {
                EthereumStorage {
                    client: self.client.clone(),
                    block: None,
                    _pd: PhantomData,
                }
            }
            pub fn storage_at(&self, block_hash: H256) -> EthereumStorage<crate::chain::MainNet> {
                EthereumStorage {
                    client: self.client.clone(),
                    block: Some(block_hash),
                    _pd: PhantomData,
                }
            }
        }
        impl EthereumStorage<crate::chain::MainNet> {
            pub async fn current_transaction_statuses(
                &self,
            ) -> crate::Result<
                Option<
                    ::subxt::ext::subxt_core::alloc::vec::Vec<
                        runtime_types::fp_rpc::TransactionStatus,
                    >,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .ethereum()
                    .current_transaction_statuses();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn block_hash_get(
                &self,
                key: &runtime_types::primitive_types::U256,
            ) -> crate::Result<Option<::subxt::ext::subxt_core::utils::H256>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .ethereum()
                    .block_hash(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn block_hash_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        runtime_types::primitive_types::U256,
                        ::subxt::ext::subxt_core::utils::H256,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .ethereum()
                    .block_hash_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <runtime_types::primitive_types::U256 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn current_receipts(
                &self,
            ) -> crate::Result<
                Option<
                    ::subxt::ext::subxt_core::alloc::vec::Vec<
                        runtime_types::ethereum::receipt::ReceiptV3,
                    >,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .ethereum()
                    .current_receipts();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn pending(
                &self,
            ) -> crate::Result<
                Option<
                    ::subxt::ext::subxt_core::alloc::vec::Vec<(
                        runtime_types::ethereum::transaction::TransactionV2,
                        runtime_types::fp_rpc::TransactionStatus,
                        runtime_types::ethereum::receipt::ReceiptV3,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .ethereum()
                    .pending();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn current_block(
                &self,
            ) -> crate::Result<
                Option<
                    runtime_types::ethereum::block::Block<
                        runtime_types::ethereum::transaction::TransactionV2,
                    >,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .ethereum()
                    .current_block();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
        }
        impl EthereumClient<crate::chain::MainNet> {
            pub fn calls(&self) -> EthereumCalls<crate::chain::MainNet> {
                EthereumCalls {
                    client: self.client.clone(),
                    _pd: PhantomData,
                }
            }
        }
        impl EthereumCalls<crate::chain::MainNet> {
            pub async fn transact(
                &self,
                transaction: crate::interfaces::mainnet::api::ethereum::calls::types::transact::Transaction,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .ethereum()
                    .transact(transaction);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn transact_wait(
                &self,
                transaction: crate::interfaces::mainnet::api::ethereum::calls::types::transact::Transaction,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .ethereum()
                    .transact(transaction);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
        }
    }
    pub mod evm {
        use super::*;
        impl EvmClient<crate::chain::MainNet> {
            pub fn storage(&self) -> EvmStorage<crate::chain::MainNet> {
                EvmStorage {
                    client: self.client.clone(),
                    block: None,
                    _pd: PhantomData,
                }
            }
            pub fn storage_at(&self, block_hash: H256) -> EvmStorage<crate::chain::MainNet> {
                EvmStorage {
                    client: self.client.clone(),
                    block: Some(block_hash),
                    _pd: PhantomData,
                }
            }
        }
        impl EvmStorage<crate::chain::MainNet> {
            pub async fn account_codes_get(
                &self,
                key: &::subxt::ext::subxt_core::utils::H160,
            ) -> crate::Result<
                Option<::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>>,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .evm()
                    .account_codes(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn account_codes_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::subxt::ext::subxt_core::utils::H160,
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .evm()
                    .account_codes_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::subxt::ext::subxt_core::utils::H160 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn account_storages_get(
                &self,
                key1: &::subxt::ext::subxt_core::utils::H160,
                key2: &::subxt::ext::subxt_core::utils::H256,
            ) -> crate::Result<Option<::subxt::ext::subxt_core::utils::H256>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .evm()
                    .account_storages(key1, key2);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn account_storages_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        (
                            ::subxt::ext::subxt_core::utils::H160,
                            ::subxt::ext::subxt_core::utils::H256,
                        ),
                        ::subxt::ext::subxt_core::utils::H256,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .evm()
                    .account_storages_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let pair = <(
                            ::subxt::ext::subxt_core::utils::H160,
                            ::subxt::ext::subxt_core::utils::H256,
                        ) as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((pair, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn account_storages_iter1(
                &self,
                key1: &::subxt::ext::subxt_core::utils::H160,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        (
                            ::subxt::ext::subxt_core::utils::H160,
                            ::subxt::ext::subxt_core::utils::H256,
                        ),
                        ::subxt::ext::subxt_core::utils::H256,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .evm()
                    .account_storages_iter1(key1);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let pair = <(
                            ::subxt::ext::subxt_core::utils::H160,
                            ::subxt::ext::subxt_core::utils::H256,
                        ) as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((pair, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn account_codes_metadata_get(
                &self,
                key: &::subxt::ext::subxt_core::utils::H160,
            ) -> crate::Result<Option<runtime_types::pallet_evm::CodeMetadata>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .evm()
                    .account_codes_metadata(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn account_codes_metadata_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::subxt::ext::subxt_core::utils::H160,
                        runtime_types::pallet_evm::CodeMetadata,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .evm()
                    .account_codes_metadata_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::subxt::ext::subxt_core::utils::H160 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn suicided_get(
                &self,
                key: &::subxt::ext::subxt_core::utils::H160,
            ) -> crate::Result<Option<()>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .evm()
                    .suicided(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn suicided_iter(
                &self,
            ) -> crate::Result<
                impl Stream<Item = crate::Result<(::subxt::ext::subxt_core::utils::H160, ())>>,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .evm()
                    .suicided_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::subxt::ext::subxt_core::utils::H160 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
        }
        impl EvmClient<crate::chain::MainNet> {
            pub fn calls(&self) -> EvmCalls<crate::chain::MainNet> {
                EvmCalls {
                    client: self.client.clone(),
                    _pd: PhantomData,
                }
            }
        }
        impl EvmCalls<crate::chain::MainNet> {
            pub async fn withdraw(
                &self,
                address: crate::interfaces::mainnet::api::evm::calls::types::withdraw::Address,
                value: crate::interfaces::mainnet::api::evm::calls::types::withdraw::Value,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .evm()
                    .withdraw(address, value);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn withdraw_wait(
                &self,
                address: crate::interfaces::mainnet::api::evm::calls::types::withdraw::Address,
                value: crate::interfaces::mainnet::api::evm::calls::types::withdraw::Value,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .evm()
                    .withdraw(address, value);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn call(
                &self,
                source: crate::interfaces::mainnet::api::evm::calls::types::call::Source,
                target: crate::interfaces::mainnet::api::evm::calls::types::call::Target,
                input: crate::interfaces::mainnet::api::evm::calls::types::call::Input,
                value: crate::interfaces::mainnet::api::evm::calls::types::call::Value,
                gas_limit: crate::interfaces::mainnet::api::evm::calls::types::call::GasLimit,
                max_fee_per_gas: crate::interfaces::mainnet::api::evm::calls::types::call::MaxFeePerGas,
                max_priority_fee_per_gas: crate::interfaces::mainnet::api::evm::calls::types::call::MaxPriorityFeePerGas,
                nonce: crate::interfaces::mainnet::api::evm::calls::types::call::Nonce,
                access_list: crate::interfaces::mainnet::api::evm::calls::types::call::AccessList,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx().evm().call(
                    source,
                    target,
                    input,
                    value,
                    gas_limit,
                    max_fee_per_gas,
                    max_priority_fee_per_gas,
                    nonce,
                    access_list,
                );
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn call_wait(
                &self,
                source: crate::interfaces::mainnet::api::evm::calls::types::call::Source,
                target: crate::interfaces::mainnet::api::evm::calls::types::call::Target,
                input: crate::interfaces::mainnet::api::evm::calls::types::call::Input,
                value: crate::interfaces::mainnet::api::evm::calls::types::call::Value,
                gas_limit: crate::interfaces::mainnet::api::evm::calls::types::call::GasLimit,
                max_fee_per_gas: crate::interfaces::mainnet::api::evm::calls::types::call::MaxFeePerGas,
                max_priority_fee_per_gas: crate::interfaces::mainnet::api::evm::calls::types::call::MaxPriorityFeePerGas,
                nonce: crate::interfaces::mainnet::api::evm::calls::types::call::Nonce,
                access_list: crate::interfaces::mainnet::api::evm::calls::types::call::AccessList,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx().evm().call(
                    source,
                    target,
                    input,
                    value,
                    gas_limit,
                    max_fee_per_gas,
                    max_priority_fee_per_gas,
                    nonce,
                    access_list,
                );
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn create(
                &self,
                source: crate::interfaces::mainnet::api::evm::calls::types::create::Source,
                init: crate::interfaces::mainnet::api::evm::calls::types::create::Init,
                value: crate::interfaces::mainnet::api::evm::calls::types::create::Value,
                gas_limit: crate::interfaces::mainnet::api::evm::calls::types::create::GasLimit,
                max_fee_per_gas: crate::interfaces::mainnet::api::evm::calls::types::create::MaxFeePerGas,
                max_priority_fee_per_gas: crate::interfaces::mainnet::api::evm::calls::types::create::MaxPriorityFeePerGas,
                nonce: crate::interfaces::mainnet::api::evm::calls::types::create::Nonce,
                access_list: crate::interfaces::mainnet::api::evm::calls::types::create::AccessList,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx().evm().create(
                    source,
                    init,
                    value,
                    gas_limit,
                    max_fee_per_gas,
                    max_priority_fee_per_gas,
                    nonce,
                    access_list,
                );
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn create_wait(
                &self,
                source: crate::interfaces::mainnet::api::evm::calls::types::create::Source,
                init: crate::interfaces::mainnet::api::evm::calls::types::create::Init,
                value: crate::interfaces::mainnet::api::evm::calls::types::create::Value,
                gas_limit: crate::interfaces::mainnet::api::evm::calls::types::create::GasLimit,
                max_fee_per_gas: crate::interfaces::mainnet::api::evm::calls::types::create::MaxFeePerGas,
                max_priority_fee_per_gas: crate::interfaces::mainnet::api::evm::calls::types::create::MaxPriorityFeePerGas,
                nonce: crate::interfaces::mainnet::api::evm::calls::types::create::Nonce,
                access_list: crate::interfaces::mainnet::api::evm::calls::types::create::AccessList,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx().evm().create(
                    source,
                    init,
                    value,
                    gas_limit,
                    max_fee_per_gas,
                    max_priority_fee_per_gas,
                    nonce,
                    access_list,
                );
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn create2(
                &self,
                source: crate::interfaces::mainnet::api::evm::calls::types::create2::Source,
                init: crate::interfaces::mainnet::api::evm::calls::types::create2::Init,
                salt: crate::interfaces::mainnet::api::evm::calls::types::create2::Salt,
                value: crate::interfaces::mainnet::api::evm::calls::types::create2::Value,
                gas_limit: crate::interfaces::mainnet::api::evm::calls::types::create2::GasLimit,
                max_fee_per_gas: crate::interfaces::mainnet::api::evm::calls::types::create2::MaxFeePerGas,
                max_priority_fee_per_gas: crate::interfaces::mainnet::api::evm::calls::types::create2::MaxPriorityFeePerGas,
                nonce: crate::interfaces::mainnet::api::evm::calls::types::create2::Nonce,
                access_list: crate::interfaces::mainnet::api::evm::calls::types::create2::AccessList,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx().evm().create2(
                    source,
                    init,
                    salt,
                    value,
                    gas_limit,
                    max_fee_per_gas,
                    max_priority_fee_per_gas,
                    nonce,
                    access_list,
                );
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn create2_wait(
                &self,
                source: crate::interfaces::mainnet::api::evm::calls::types::create2::Source,
                init: crate::interfaces::mainnet::api::evm::calls::types::create2::Init,
                salt: crate::interfaces::mainnet::api::evm::calls::types::create2::Salt,
                value: crate::interfaces::mainnet::api::evm::calls::types::create2::Value,
                gas_limit: crate::interfaces::mainnet::api::evm::calls::types::create2::GasLimit,
                max_fee_per_gas: crate::interfaces::mainnet::api::evm::calls::types::create2::MaxFeePerGas,
                max_priority_fee_per_gas: crate::interfaces::mainnet::api::evm::calls::types::create2::MaxPriorityFeePerGas,
                nonce: crate::interfaces::mainnet::api::evm::calls::types::create2::Nonce,
                access_list: crate::interfaces::mainnet::api::evm::calls::types::create2::AccessList,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx().evm().create2(
                    source,
                    init,
                    salt,
                    value,
                    gas_limit,
                    max_fee_per_gas,
                    max_priority_fee_per_gas,
                    nonce,
                    access_list,
                );
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
        }
    }
    pub mod governance {
        use super::*;
        impl GovernanceClient<crate::chain::MainNet> {
            pub fn storage(&self) -> GovernanceStorage<crate::chain::MainNet> {
                GovernanceStorage {
                    client: self.client.clone(),
                    block: None,
                    _pd: PhantomData,
                }
            }
            pub fn storage_at(&self, block_hash: H256) -> GovernanceStorage<crate::chain::MainNet> {
                GovernanceStorage {
                    client: self.client.clone(),
                    block: Some(block_hash),
                    _pd: PhantomData,
                }
            }
        }
        impl GovernanceStorage<crate::chain::MainNet> {
            pub async fn proposals_get(
                &self,
                key: &::core::primitive::u64,
            ) -> crate::Result<Option<runtime_types::pallet_governance::proposal::Proposal>>
            {
                let call = crate::interfaces::mainnet::api::storage()
                    .governance()
                    .proposals(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn proposals_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::core::primitive::u64,
                        runtime_types::pallet_governance::proposal::Proposal,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .governance()
                    .proposals_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::core::primitive::u64 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn agents_frozen(&self) -> crate::Result<Option<::core::primitive::bool>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .governance()
                    .agents_frozen();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn agent_applications_get(
                &self,
                key: &::core::primitive::u32,
            ) -> crate::Result<
                Option<runtime_types::pallet_governance::application::AgentApplication>,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .governance()
                    .agent_applications(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn agent_applications_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::core::primitive::u32,
                        runtime_types::pallet_governance::application::AgentApplication,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .governance()
                    .agent_applications_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::core::primitive::u32 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn not_delegating_voting_power(
                &self,
            ) -> crate::Result<
                Option<
                    runtime_types::bounded_collections::bounded_btree_set::BoundedBTreeSet<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                    >,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .governance()
                    .not_delegating_voting_power();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn unrewarded_proposals_get(
                &self,
                key: &::core::primitive::u64,
            ) -> crate::Result<Option<runtime_types::pallet_governance::proposal::UnrewardedProposal>>
            {
                let call = crate::interfaces::mainnet::api::storage()
                    .governance()
                    .unrewarded_proposals(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn unrewarded_proposals_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::core::primitive::u64,
                        runtime_types::pallet_governance::proposal::UnrewardedProposal,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .governance()
                    .unrewarded_proposals_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::core::primitive::u64 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn dao_treasury_address(
                &self,
            ) -> crate::Result<Option<::subxt::ext::subxt_core::utils::AccountId32>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .governance()
                    .dao_treasury_address();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn namespaces_frozen(
                &self,
            ) -> crate::Result<Option<::core::primitive::bool>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .governance()
                    .namespaces_frozen();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn global_governance_config(
                &self,
            ) -> crate::Result<
                Option<runtime_types::pallet_governance::config::GovernanceConfiguration>,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .governance()
                    .global_governance_config();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn whitelist_get(
                &self,
                key: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<Option<()>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .governance()
                    .whitelist(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn whitelist_iter(
                &self,
            ) -> crate::Result<
                impl Stream<Item = crate::Result<(::subxt::ext::subxt_core::utils::AccountId32, ())>>,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .governance()
                    .whitelist_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::subxt::ext::subxt_core::utils::AccountId32 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn treasury_emission_fee(
                &self,
            ) -> crate::Result<Option<runtime_types::sp_arithmetic::per_things::Percent>>
            {
                let call = crate::interfaces::mainnet::api::storage()
                    .governance()
                    .treasury_emission_fee();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn allocators_get(
                &self,
                key: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<Option<()>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .governance()
                    .allocators(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn allocators_iter(
                &self,
            ) -> crate::Result<
                impl Stream<Item = crate::Result<(::subxt::ext::subxt_core::utils::AccountId32, ())>>,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .governance()
                    .allocators_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::subxt::ext::subxt_core::utils::AccountId32 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
        }
        impl GovernanceClient<crate::chain::MainNet> {
            pub fn calls(&self) -> GovernanceCalls<crate::chain::MainNet> {
                GovernanceCalls {
                    client: self.client.clone(),
                    _pd: PhantomData,
                }
            }
        }
        impl GovernanceCalls<crate::chain::MainNet> {
            pub async fn add_allocator(
                &self,
                key: crate::interfaces::mainnet::api::governance::calls::types::add_allocator::Key,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .governance()
                    .add_allocator(key);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn add_allocator_wait(
                &self,
                key: crate::interfaces::mainnet::api::governance::calls::types::add_allocator::Key,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .governance()
                    .add_allocator(key);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn remove_allocator(
                &self,
                key: crate::interfaces::mainnet::api::governance::calls::types::remove_allocator::Key,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .governance()
                    .remove_allocator(key);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn remove_allocator_wait(
                &self,
                key: crate::interfaces::mainnet::api::governance::calls::types::remove_allocator::Key,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .governance()
                    .remove_allocator(key);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn add_to_whitelist(
                &self,
                key: crate::interfaces::mainnet::api::governance::calls::types::add_to_whitelist::Key,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .governance()
                    .add_to_whitelist(key);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn add_to_whitelist_wait(
                &self,
                key: crate::interfaces::mainnet::api::governance::calls::types::add_to_whitelist::Key,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .governance()
                    .add_to_whitelist(key);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn remove_from_whitelist(
                &self,
                key: crate::interfaces::mainnet::api::governance::calls::types::remove_from_whitelist::Key,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .governance()
                    .remove_from_whitelist(key);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn remove_from_whitelist_wait(
                &self,
                key: crate::interfaces::mainnet::api::governance::calls::types::remove_from_whitelist::Key,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .governance()
                    .remove_from_whitelist(key);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn accept_application(
                &self,
                application_id: crate::interfaces::mainnet::api::governance::calls::types::accept_application::ApplicationId,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .governance()
                    .accept_application(application_id);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn accept_application_wait(
                &self,
                application_id: crate::interfaces::mainnet::api::governance::calls::types::accept_application::ApplicationId,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .governance()
                    .accept_application(application_id);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn deny_application(
                &self,
                application_id: crate::interfaces::mainnet::api::governance::calls::types::deny_application::ApplicationId,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .governance()
                    .deny_application(application_id);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn deny_application_wait(
                &self,
                application_id: crate::interfaces::mainnet::api::governance::calls::types::deny_application::ApplicationId,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .governance()
                    .deny_application(application_id);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn penalize_agent(
                &self,
                agent_key: crate::interfaces::mainnet::api::governance::calls::types::penalize_agent::AgentKey,
                percentage: crate::interfaces::mainnet::api::governance::calls::types::penalize_agent::Percentage,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .governance()
                    .penalize_agent(agent_key, percentage);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn penalize_agent_wait(
                &self,
                agent_key: crate::interfaces::mainnet::api::governance::calls::types::penalize_agent::AgentKey,
                percentage: crate::interfaces::mainnet::api::governance::calls::types::penalize_agent::Percentage,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .governance()
                    .penalize_agent(agent_key, percentage);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn submit_application(
                &self,
                agent_key: crate::interfaces::mainnet::api::governance::calls::types::submit_application::AgentKey,
                metadata: crate::interfaces::mainnet::api::governance::calls::types::submit_application::Metadata,
                removing: crate::interfaces::mainnet::api::governance::calls::types::submit_application::Removing,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .governance()
                    .submit_application(agent_key, metadata, removing);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn submit_application_wait(
                &self,
                agent_key: crate::interfaces::mainnet::api::governance::calls::types::submit_application::AgentKey,
                metadata: crate::interfaces::mainnet::api::governance::calls::types::submit_application::Metadata,
                removing: crate::interfaces::mainnet::api::governance::calls::types::submit_application::Removing,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .governance()
                    .submit_application(agent_key, metadata, removing);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn add_global_params_proposal(
                &self,
                data: crate::interfaces::mainnet::api::governance::calls::types::add_global_params_proposal::Data,
                metadata: crate::interfaces::mainnet::api::governance::calls::types::add_global_params_proposal::Metadata,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .governance()
                    .add_global_params_proposal(data, metadata);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn add_global_params_proposal_wait(
                &self,
                data: crate::interfaces::mainnet::api::governance::calls::types::add_global_params_proposal::Data,
                metadata: crate::interfaces::mainnet::api::governance::calls::types::add_global_params_proposal::Metadata,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .governance()
                    .add_global_params_proposal(data, metadata);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn add_global_custom_proposal(
                &self,
                metadata: crate::interfaces::mainnet::api::governance::calls::types::add_global_custom_proposal::Metadata,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .governance()
                    .add_global_custom_proposal(metadata);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn add_global_custom_proposal_wait(
                &self,
                metadata: crate::interfaces::mainnet::api::governance::calls::types::add_global_custom_proposal::Metadata,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .governance()
                    .add_global_custom_proposal(metadata);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn add_dao_treasury_transfer_proposal(
                &self,
                value: crate::interfaces::mainnet::api::governance::calls::types::add_dao_treasury_transfer_proposal::Value,
                destination_key: crate::interfaces::mainnet::api::governance::calls::types::add_dao_treasury_transfer_proposal::DestinationKey,
                data: crate::interfaces::mainnet::api::governance::calls::types::add_dao_treasury_transfer_proposal::Data,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .governance()
                    .add_dao_treasury_transfer_proposal(value, destination_key, data);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn add_dao_treasury_transfer_proposal_wait(
                &self,
                value: crate::interfaces::mainnet::api::governance::calls::types::add_dao_treasury_transfer_proposal::Value,
                destination_key: crate::interfaces::mainnet::api::governance::calls::types::add_dao_treasury_transfer_proposal::DestinationKey,
                data: crate::interfaces::mainnet::api::governance::calls::types::add_dao_treasury_transfer_proposal::Data,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .governance()
                    .add_dao_treasury_transfer_proposal(value, destination_key, data);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn vote_proposal(
                &self,
                proposal_id: crate::interfaces::mainnet::api::governance::calls::types::vote_proposal::ProposalId,
                agree: crate::interfaces::mainnet::api::governance::calls::types::vote_proposal::Agree,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .governance()
                    .vote_proposal(proposal_id, agree);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn vote_proposal_wait(
                &self,
                proposal_id: crate::interfaces::mainnet::api::governance::calls::types::vote_proposal::ProposalId,
                agree: crate::interfaces::mainnet::api::governance::calls::types::vote_proposal::Agree,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .governance()
                    .vote_proposal(proposal_id, agree);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn remove_vote_proposal(
                &self,
                proposal_id: crate::interfaces::mainnet::api::governance::calls::types::remove_vote_proposal::ProposalId,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .governance()
                    .remove_vote_proposal(proposal_id);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn remove_vote_proposal_wait(
                &self,
                proposal_id: crate::interfaces::mainnet::api::governance::calls::types::remove_vote_proposal::ProposalId,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .governance()
                    .remove_vote_proposal(proposal_id);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn enable_vote_delegation(
                &self,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .governance()
                    .enable_vote_delegation();
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn enable_vote_delegation_wait(
                &self,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .governance()
                    .enable_vote_delegation();
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn disable_vote_delegation(
                &self,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .governance()
                    .disable_vote_delegation();
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn disable_vote_delegation_wait(
                &self,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .governance()
                    .disable_vote_delegation();
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn add_emission_proposal(
                &self,
                recycling_percentage: crate::interfaces::mainnet::api::governance::calls::types::add_emission_proposal::RecyclingPercentage,
                treasury_percentage: crate::interfaces::mainnet::api::governance::calls::types::add_emission_proposal::TreasuryPercentage,
                incentives_ratio: crate::interfaces::mainnet::api::governance::calls::types::add_emission_proposal::IncentivesRatio,
                data: crate::interfaces::mainnet::api::governance::calls::types::add_emission_proposal::Data,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .governance()
                    .add_emission_proposal(
                        recycling_percentage,
                        treasury_percentage,
                        incentives_ratio,
                        data,
                    );
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn add_emission_proposal_wait(
                &self,
                recycling_percentage: crate::interfaces::mainnet::api::governance::calls::types::add_emission_proposal::RecyclingPercentage,
                treasury_percentage: crate::interfaces::mainnet::api::governance::calls::types::add_emission_proposal::TreasuryPercentage,
                incentives_ratio: crate::interfaces::mainnet::api::governance::calls::types::add_emission_proposal::IncentivesRatio,
                data: crate::interfaces::mainnet::api::governance::calls::types::add_emission_proposal::Data,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .governance()
                    .add_emission_proposal(
                        recycling_percentage,
                        treasury_percentage,
                        incentives_ratio,
                        data,
                    );
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn set_emission_params(
                &self,
                recycling_percentage: crate::interfaces::mainnet::api::governance::calls::types::set_emission_params::RecyclingPercentage,
                treasury_percentage: crate::interfaces::mainnet::api::governance::calls::types::set_emission_params::TreasuryPercentage,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .governance()
                    .set_emission_params(recycling_percentage, treasury_percentage);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn set_emission_params_wait(
                &self,
                recycling_percentage: crate::interfaces::mainnet::api::governance::calls::types::set_emission_params::RecyclingPercentage,
                treasury_percentage: crate::interfaces::mainnet::api::governance::calls::types::set_emission_params::TreasuryPercentage,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .governance()
                    .set_emission_params(recycling_percentage, treasury_percentage);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn toggle_agent_freezing(
                &self,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .governance()
                    .toggle_agent_freezing();
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn toggle_agent_freezing_wait(
                &self,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .governance()
                    .toggle_agent_freezing();
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn toggle_namespace_freezing(
                &self,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .governance()
                    .toggle_namespace_freezing();
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn toggle_namespace_freezing_wait(
                &self,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .governance()
                    .toggle_namespace_freezing();
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
        }
    }
    pub mod torus0 {
        use super::*;
        impl Torus0Client<crate::chain::MainNet> {
            pub fn storage(&self) -> Torus0Storage<crate::chain::MainNet> {
                Torus0Storage {
                    client: self.client.clone(),
                    block: None,
                    _pd: PhantomData,
                }
            }
            pub fn storage_at(&self, block_hash: H256) -> Torus0Storage<crate::chain::MainNet> {
                Torus0Storage {
                    client: self.client.clone(),
                    block: Some(block_hash),
                    _pd: PhantomData,
                }
            }
        }
        impl Torus0Storage<crate::chain::MainNet> {
            pub async fn namespace_pricing_config(
                &self,
            ) -> crate::Result<
                Option<runtime_types::pallet_torus0::namespace::NamespacePricingConfig>,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .torus0()
                    .namespace_pricing_config();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn namespace_count_get(
                &self,
                key: &runtime_types::pallet_torus0::namespace::NamespaceOwnership,
            ) -> crate::Result<Option<::core::primitive::u32>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .torus0()
                    .namespace_count(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn namespace_count_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        runtime_types::pallet_torus0::namespace::NamespaceOwnership,
                        ::core::primitive::u32,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .torus0()
                    .namespace_count_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <runtime_types::pallet_torus0::namespace::NamespaceOwnership as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn max_agent_url_length(
                &self,
            ) -> crate::Result<Option<::core::primitive::u16>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .torus0()
                    .max_agent_url_length();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn dividends_participation_weight(
                &self,
            ) -> crate::Result<Option<runtime_types::sp_arithmetic::per_things::Percent>>
            {
                let call = crate::interfaces::mainnet::api::storage()
                    .torus0()
                    .dividends_participation_weight();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn max_allowed_validators(
                &self,
            ) -> crate::Result<Option<::core::primitive::u16>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .torus0()
                    .max_allowed_validators();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn min_validator_stake(
                &self,
            ) -> crate::Result<Option<::core::primitive::u128>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .torus0()
                    .min_validator_stake();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn registrations_this_interval(
                &self,
            ) -> crate::Result<Option<::core::primitive::u16>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .torus0()
                    .registrations_this_interval();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn agents_get(
                &self,
                key: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<Option<runtime_types::pallet_torus0::agent::Agent>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .torus0()
                    .agents(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn agents_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        runtime_types::pallet_torus0::agent::Agent,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .torus0()
                    .agents_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::subxt::ext::subxt_core::utils::AccountId32 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn reward_interval(&self) -> crate::Result<Option<::core::primitive::u16>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .torus0()
                    .reward_interval();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn max_name_length(&self) -> crate::Result<Option<::core::primitive::u16>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .torus0()
                    .max_name_length();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn max_registrations_per_block(
                &self,
            ) -> crate::Result<Option<::core::primitive::u16>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .torus0()
                    .max_registrations_per_block();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn namespaces_get(
                &self,
                key1: &runtime_types::pallet_torus0::namespace::NamespaceOwnership,
                key2: &runtime_types::pallet_torus0_api::NamespacePath,
            ) -> crate::Result<Option<runtime_types::pallet_torus0::namespace::NamespaceMetadata>>
            {
                let call = crate::interfaces::mainnet::api::storage()
                    .torus0()
                    .namespaces(key1, key2);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn namespaces_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        (
                            runtime_types::pallet_torus0::namespace::NamespaceOwnership,
                            runtime_types::pallet_torus0_api::NamespacePath,
                        ),
                        runtime_types::pallet_torus0::namespace::NamespaceMetadata,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .torus0()
                    .namespaces_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let pair = <(
                            runtime_types::pallet_torus0::namespace::NamespaceOwnership,
                            runtime_types::pallet_torus0_api::NamespacePath,
                        ) as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((pair, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn namespaces_iter1(
                &self,
                key1: &runtime_types::pallet_torus0::namespace::NamespaceOwnership,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        (
                            runtime_types::pallet_torus0::namespace::NamespaceOwnership,
                            runtime_types::pallet_torus0_api::NamespacePath,
                        ),
                        runtime_types::pallet_torus0::namespace::NamespaceMetadata,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .torus0()
                    .namespaces_iter1(key1);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let pair = <(
                            runtime_types::pallet_torus0::namespace::NamespaceOwnership,
                            runtime_types::pallet_torus0_api::NamespacePath,
                        ) as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((pair, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn registrations_this_block(
                &self,
            ) -> crate::Result<Option<::core::primitive::u16>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .torus0()
                    .registrations_this_block();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn min_allowed_stake(
                &self,
            ) -> crate::Result<Option<::core::primitive::u128>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .torus0()
                    .min_allowed_stake();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn burn_config(
                &self,
            ) -> crate::Result<Option<runtime_types::pallet_torus0::burn::BurnConfiguration>>
            {
                let call = crate::interfaces::mainnet::api::storage()
                    .torus0()
                    .burn_config();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn staking_to_get(
                &self,
                key1: &::subxt::ext::subxt_core::utils::AccountId32,
                key2: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<Option<::core::primitive::u128>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .torus0()
                    .staking_to(key1, key2);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn staking_to_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        (
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        ),
                        ::core::primitive::u128,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .torus0()
                    .staking_to_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let pair = <(
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        ) as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((pair, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn staking_to_iter1(
                &self,
                key1: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        (
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        ),
                        ::core::primitive::u128,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .torus0()
                    .staking_to_iter1(key1);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let pair = <(
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        ) as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((pair, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn min_name_length(&self) -> crate::Result<Option<::core::primitive::u16>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .torus0()
                    .min_name_length();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn staked_by_get(
                &self,
                key1: &::subxt::ext::subxt_core::utils::AccountId32,
                key2: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<Option<::core::primitive::u128>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .torus0()
                    .staked_by(key1, key2);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn staked_by_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        (
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        ),
                        ::core::primitive::u128,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .torus0()
                    .staked_by_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let pair = <(
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        ) as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((pair, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn staked_by_iter1(
                &self,
                key1: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        (
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        ),
                        ::core::primitive::u128,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .torus0()
                    .staked_by_iter1(key1);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let pair = <(
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        ) as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((pair, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn burn(&self) -> crate::Result<Option<::core::primitive::u128>> {
                let call = crate::interfaces::mainnet::api::storage().torus0().burn();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn fee_constraints(
                &self,
            ) -> crate::Result<Option<runtime_types::pallet_torus0::fee::ValidatorFeeConstraints>>
            {
                let call = crate::interfaces::mainnet::api::storage()
                    .torus0()
                    .fee_constraints();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn agent_update_cooldown(
                &self,
            ) -> crate::Result<Option<::core::primitive::u64>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .torus0()
                    .agent_update_cooldown();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn total_stake(&self) -> crate::Result<Option<::core::primitive::u128>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .torus0()
                    .total_stake();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
        }
        impl Torus0Client<crate::chain::MainNet> {
            pub fn calls(&self) -> Torus0Calls<crate::chain::MainNet> {
                Torus0Calls {
                    client: self.client.clone(),
                    _pd: PhantomData,
                }
            }
        }
        impl Torus0Calls<crate::chain::MainNet> {
            pub async fn add_stake(
                &self,
                agent_key: crate::interfaces::mainnet::api::torus0::calls::types::add_stake::AgentKey,
                amount: crate::interfaces::mainnet::api::torus0::calls::types::add_stake::Amount,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .torus0()
                    .add_stake(agent_key, amount);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn add_stake_wait(
                &self,
                agent_key: crate::interfaces::mainnet::api::torus0::calls::types::add_stake::AgentKey,
                amount: crate::interfaces::mainnet::api::torus0::calls::types::add_stake::Amount,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .torus0()
                    .add_stake(agent_key, amount);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn remove_stake(
                &self,
                agent_key: crate::interfaces::mainnet::api::torus0::calls::types::remove_stake::AgentKey,
                amount: crate::interfaces::mainnet::api::torus0::calls::types::remove_stake::Amount,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .torus0()
                    .remove_stake(agent_key, amount);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn remove_stake_wait(
                &self,
                agent_key: crate::interfaces::mainnet::api::torus0::calls::types::remove_stake::AgentKey,
                amount: crate::interfaces::mainnet::api::torus0::calls::types::remove_stake::Amount,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .torus0()
                    .remove_stake(agent_key, amount);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn transfer_stake(
                &self,
                agent_key: crate::interfaces::mainnet::api::torus0::calls::types::transfer_stake::AgentKey,
                new_agent_key: crate::interfaces::mainnet::api::torus0::calls::types::transfer_stake::NewAgentKey,
                amount: crate::interfaces::mainnet::api::torus0::calls::types::transfer_stake::Amount,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .torus0()
                    .transfer_stake(agent_key, new_agent_key, amount);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn transfer_stake_wait(
                &self,
                agent_key: crate::interfaces::mainnet::api::torus0::calls::types::transfer_stake::AgentKey,
                new_agent_key: crate::interfaces::mainnet::api::torus0::calls::types::transfer_stake::NewAgentKey,
                amount: crate::interfaces::mainnet::api::torus0::calls::types::transfer_stake::Amount,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .torus0()
                    .transfer_stake(agent_key, new_agent_key, amount);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn register_agent(
                &self,
                agent_key: crate::interfaces::mainnet::api::torus0::calls::types::register_agent::AgentKey,
                name: crate::interfaces::mainnet::api::torus0::calls::types::register_agent::Name,
                url: crate::interfaces::mainnet::api::torus0::calls::types::register_agent::Url,
                metadata: crate::interfaces::mainnet::api::torus0::calls::types::register_agent::Metadata,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .torus0()
                    .register_agent(agent_key, name, url, metadata);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn register_agent_wait(
                &self,
                agent_key: crate::interfaces::mainnet::api::torus0::calls::types::register_agent::AgentKey,
                name: crate::interfaces::mainnet::api::torus0::calls::types::register_agent::Name,
                url: crate::interfaces::mainnet::api::torus0::calls::types::register_agent::Url,
                metadata: crate::interfaces::mainnet::api::torus0::calls::types::register_agent::Metadata,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .torus0()
                    .register_agent(agent_key, name, url, metadata);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn unregister_agent(
                &self,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .torus0()
                    .unregister_agent();
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn unregister_agent_wait(
                &self,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .torus0()
                    .unregister_agent();
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn update_agent(
                &self,
                url: crate::interfaces::mainnet::api::torus0::calls::types::update_agent::Url,
                metadata: crate::interfaces::mainnet::api::torus0::calls::types::update_agent::Metadata,
                staking_fee: crate::interfaces::mainnet::api::torus0::calls::types::update_agent::StakingFee,
                weight_control_fee: crate::interfaces::mainnet::api::torus0::calls::types::update_agent::WeightControlFee,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx().torus0().update_agent(
                    url,
                    metadata,
                    staking_fee,
                    weight_control_fee,
                );
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn update_agent_wait(
                &self,
                url: crate::interfaces::mainnet::api::torus0::calls::types::update_agent::Url,
                metadata: crate::interfaces::mainnet::api::torus0::calls::types::update_agent::Metadata,
                staking_fee: crate::interfaces::mainnet::api::torus0::calls::types::update_agent::StakingFee,
                weight_control_fee: crate::interfaces::mainnet::api::torus0::calls::types::update_agent::WeightControlFee,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx().torus0().update_agent(
                    url,
                    metadata,
                    staking_fee,
                    weight_control_fee,
                );
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn set_agent_update_cooldown(
                &self,
                new_cooldown: crate::interfaces::mainnet::api::torus0::calls::types::set_agent_update_cooldown::NewCooldown,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .torus0()
                    .set_agent_update_cooldown(new_cooldown);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn set_agent_update_cooldown_wait(
                &self,
                new_cooldown: crate::interfaces::mainnet::api::torus0::calls::types::set_agent_update_cooldown::NewCooldown,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .torus0()
                    .set_agent_update_cooldown(new_cooldown);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn create_namespace(
                &self,
                path: crate::interfaces::mainnet::api::torus0::calls::types::create_namespace::Path,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .torus0()
                    .create_namespace(path);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn create_namespace_wait(
                &self,
                path: crate::interfaces::mainnet::api::torus0::calls::types::create_namespace::Path,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .torus0()
                    .create_namespace(path);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn delete_namespace(
                &self,
                path: crate::interfaces::mainnet::api::torus0::calls::types::delete_namespace::Path,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .torus0()
                    .delete_namespace(path);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn delete_namespace_wait(
                &self,
                path: crate::interfaces::mainnet::api::torus0::calls::types::delete_namespace::Path,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .torus0()
                    .delete_namespace(path);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
        }
    }
    pub mod emission0 {
        use super::*;
        impl Emission0Client<crate::chain::MainNet> {
            pub fn storage(&self) -> Emission0Storage<crate::chain::MainNet> {
                Emission0Storage {
                    client: self.client.clone(),
                    block: None,
                    _pd: PhantomData,
                }
            }
            pub fn storage_at(&self, block_hash: H256) -> Emission0Storage<crate::chain::MainNet> {
                Emission0Storage {
                    client: self.client.clone(),
                    block: Some(block_hash),
                    _pd: PhantomData,
                }
            }
        }
        impl Emission0Storage<crate::chain::MainNet> {
            pub async fn consensus_members_get(
                &self,
                key: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<Option<runtime_types::pallet_emission0::ConsensusMember>>
            {
                let call = crate::interfaces::mainnet::api::storage()
                    .emission0()
                    .consensus_members(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn consensus_members_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        runtime_types::pallet_emission0::ConsensusMember,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .emission0()
                    .consensus_members_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::subxt::ext::subxt_core::utils::AccountId32 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn pending_emission(&self) -> crate::Result<Option<::core::primitive::u128>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .emission0()
                    .pending_emission();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn weight_control_delegation_get(
                &self,
                key: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<Option<::subxt::ext::subxt_core::utils::AccountId32>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .emission0()
                    .weight_control_delegation(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn weight_control_delegation_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        ::subxt::ext::subxt_core::utils::AccountId32,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .emission0()
                    .weight_control_delegation_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::subxt::ext::subxt_core::utils::AccountId32 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn incentives_ratio(
                &self,
            ) -> crate::Result<Option<runtime_types::sp_arithmetic::per_things::Percent>>
            {
                let call = crate::interfaces::mainnet::api::storage()
                    .emission0()
                    .incentives_ratio();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn emission_recycling_percentage(
                &self,
            ) -> crate::Result<Option<runtime_types::sp_arithmetic::per_things::Percent>>
            {
                let call = crate::interfaces::mainnet::api::storage()
                    .emission0()
                    .emission_recycling_percentage();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
        }
        impl Emission0Client<crate::chain::MainNet> {
            pub fn calls(&self) -> Emission0Calls<crate::chain::MainNet> {
                Emission0Calls {
                    client: self.client.clone(),
                    _pd: PhantomData,
                }
            }
        }
        impl Emission0Calls<crate::chain::MainNet> {
            pub async fn set_weights(
                &self,
                weights: crate::interfaces::mainnet::api::emission0::calls::types::set_weights::Weights,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .emission0()
                    .set_weights(weights);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn set_weights_wait(
                &self,
                weights: crate::interfaces::mainnet::api::emission0::calls::types::set_weights::Weights,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .emission0()
                    .set_weights(weights);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn delegate_weight_control(
                &self,
                target: crate::interfaces::mainnet::api::emission0::calls::types::delegate_weight_control::Target,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .emission0()
                    .delegate_weight_control(target);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn delegate_weight_control_wait(
                &self,
                target: crate::interfaces::mainnet::api::emission0::calls::types::delegate_weight_control::Target,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .emission0()
                    .delegate_weight_control(target);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn regain_weight_control(
                &self,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .emission0()
                    .regain_weight_control();
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn regain_weight_control_wait(
                &self,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .emission0()
                    .regain_weight_control();
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
        }
    }
    pub mod permission0 {
        use super::*;
        impl Permission0Client<crate::chain::MainNet> {
            pub fn storage(&self) -> Permission0Storage<crate::chain::MainNet> {
                Permission0Storage {
                    client: self.client.clone(),
                    block: None,
                    _pd: PhantomData,
                }
            }
            pub fn storage_at(
                &self,
                block_hash: H256,
            ) -> Permission0Storage<crate::chain::MainNet> {
                Permission0Storage {
                    client: self.client.clone(),
                    block: Some(block_hash),
                    _pd: PhantomData,
                }
            }
        }
        impl Permission0Storage<crate::chain::MainNet> {
            /// Get storage n-map value by keys
            pub async fn accumulated_stream_amounts_get(
                &self,
                account_id32_1: ::subxt::ext::subxt_core::utils::AccountId32,
                h256_2: ::subxt::ext::subxt_core::utils::H256,
                h256_3: ::subxt::ext::subxt_core::utils::H256,
            ) -> crate::Result<Option<::core::primitive::u128>> {
                let call = crate::interfaces::mainnet::api::storage()
                    .permission0()
                    .accumulated_stream_amounts(account_id32_1, h256_2, h256_3);
                let storage = self.client.storage().at_latest().await?;
                Ok(storage.fetch(&call).await?)
            }
            /// Query all entries in storage n-map
            pub async fn accumulated_stream_amounts_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        (
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            ::subxt::ext::subxt_core::utils::H256,
                            ::subxt::ext::subxt_core::utils::H256,
                        ),
                        ::core::primitive::u128,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .permission0()
                    .accumulated_stream_amounts_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let tuple = <(
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            ::subxt::ext::subxt_core::utils::H256,
                            ::subxt::ext::subxt_core::utils::H256,
                        ) as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((tuple, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn accumulated_stream_amounts_iter1(
                &self,
                key0: ::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        ::core::primitive::u128,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .permission0()
                    .accumulated_stream_amounts_iter1(key0);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let tuple =
                            <::subxt::ext::subxt_core::utils::AccountId32 as Decode>::decode(
                                &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                            )?;
                        Ok((tuple, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn accumulated_stream_amounts_iter2(
                &self,
                key0: ::subxt::ext::subxt_core::utils::AccountId32,
                key1: ::subxt::ext::subxt_core::utils::H256,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        (
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            ::subxt::ext::subxt_core::utils::H256,
                        ),
                        ::core::primitive::u128,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .permission0()
                    .accumulated_stream_amounts_iter2(key0, key1);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let tuple = <(
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            ::subxt::ext::subxt_core::utils::H256,
                        ) as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((tuple, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn permissions_by_grantor_get(
                &self,
                key: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<
                Option<
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::subxt::ext::subxt_core::utils::H256,
                    >,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .permission0()
                    .permissions_by_grantor(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn permissions_by_grantor_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::subxt::ext::subxt_core::utils::H256,
                        >,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .permission0()
                    .permissions_by_grantor_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::subxt::ext::subxt_core::utils::AccountId32 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn permissions_by_participants_get(
                &self,
                key1: &::subxt::ext::subxt_core::utils::AccountId32,
                key2: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<
                Option<
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::subxt::ext::subxt_core::utils::H256,
                    >,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .permission0()
                    .permissions_by_participants(key1, key2);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn permissions_by_participants_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        (
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        ),
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::subxt::ext::subxt_core::utils::H256,
                        >,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .permission0()
                    .permissions_by_participants_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let pair = <(
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        ) as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((pair, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn permissions_by_participants_iter1(
                &self,
                key1: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        (
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        ),
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::subxt::ext::subxt_core::utils::H256,
                        >,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .permission0()
                    .permissions_by_participants_iter1(key1);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let pair = <(
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        ) as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((pair, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn enforcement_tracking_get(
                &self,
                key1: &::subxt::ext::subxt_core::utils::H256,
                key2: &runtime_types::pallet_permission0::permission::EnforcementReferendum,
            ) -> crate::Result<
                Option<
                    runtime_types::bounded_collections::bounded_btree_set::BoundedBTreeSet<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                    >,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .permission0()
                    .enforcement_tracking(key1, key2);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn enforcement_tracking_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        (
                            ::subxt::ext::subxt_core::utils::H256,
                            runtime_types::pallet_permission0::permission::EnforcementReferendum,
                        ),
                        runtime_types::bounded_collections::bounded_btree_set::BoundedBTreeSet<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        >,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .permission0()
                    .enforcement_tracking_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let pair = <(
                            ::subxt::ext::subxt_core::utils::H256,
                            runtime_types::pallet_permission0::permission::EnforcementReferendum,
                        ) as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((pair, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn enforcement_tracking_iter1(
                &self,
                key1: &::subxt::ext::subxt_core::utils::H256,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        (
                            ::subxt::ext::subxt_core::utils::H256,
                            runtime_types::pallet_permission0::permission::EnforcementReferendum,
                        ),
                        runtime_types::bounded_collections::bounded_btree_set::BoundedBTreeSet<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        >,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .permission0()
                    .enforcement_tracking_iter1(key1);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let pair = <(
                            ::subxt::ext::subxt_core::utils::H256,
                            runtime_types::pallet_permission0::permission::EnforcementReferendum,
                        ) as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((pair, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn permissions_get(
                &self,
                key: &::subxt::ext::subxt_core::utils::H256,
            ) -> crate::Result<
                Option<runtime_types::pallet_permission0::permission::PermissionContract>,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .permission0()
                    .permissions(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn permissions_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::subxt::ext::subxt_core::utils::H256,
                        runtime_types::pallet_permission0::permission::PermissionContract,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .permission0()
                    .permissions_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::subxt::ext::subxt_core::utils::H256 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn permissions_by_grantee_get(
                &self,
                key: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<
                Option<
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::subxt::ext::subxt_core::utils::H256,
                    >,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .permission0()
                    .permissions_by_grantee(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn permissions_by_grantee_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::subxt::ext::subxt_core::utils::H256,
                        >,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .permission0()
                    .permissions_by_grantee_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::subxt::ext::subxt_core::utils::AccountId32 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn revocation_tracking_get(
                &self,
                key: &::subxt::ext::subxt_core::utils::H256,
            ) -> crate::Result<
                Option<
                    runtime_types::bounded_collections::bounded_btree_set::BoundedBTreeSet<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                    >,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .permission0()
                    .revocation_tracking(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn revocation_tracking_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::subxt::ext::subxt_core::utils::H256,
                        runtime_types::bounded_collections::bounded_btree_set::BoundedBTreeSet<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        >,
                    )>,
                >,
            > {
                let call = crate::interfaces::mainnet::api::storage()
                    .permission0()
                    .revocation_tracking_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::subxt::ext::subxt_core::utils::H256 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
        }
        impl Permission0Client<crate::chain::MainNet> {
            pub fn calls(&self) -> Permission0Calls<crate::chain::MainNet> {
                Permission0Calls {
                    client: self.client.clone(),
                    _pd: PhantomData,
                }
            }
        }
        impl Permission0Calls<crate::chain::MainNet> {
            pub async fn grant_emission_permission(
                &self,
                grantee: crate::interfaces::mainnet::api::permission0::calls::types::grant_emission_permission::Grantee,
                allocation: crate::interfaces::mainnet::api::permission0::calls::types::grant_emission_permission::Allocation,
                targets: crate::interfaces::mainnet::api::permission0::calls::types::grant_emission_permission::Targets,
                distribution: crate::interfaces::mainnet::api::permission0::calls::types::grant_emission_permission::Distribution,
                duration: crate::interfaces::mainnet::api::permission0::calls::types::grant_emission_permission::Duration,
                revocation: crate::interfaces::mainnet::api::permission0::calls::types::grant_emission_permission::Revocation,
                enforcement: crate::interfaces::mainnet::api::permission0::calls::types::grant_emission_permission::Enforcement,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .permission0()
                    .grant_emission_permission(
                        grantee,
                        allocation,
                        targets,
                        distribution,
                        duration,
                        revocation,
                        enforcement,
                    );
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn grant_emission_permission_wait(
                &self,
                grantee: crate::interfaces::mainnet::api::permission0::calls::types::grant_emission_permission::Grantee,
                allocation: crate::interfaces::mainnet::api::permission0::calls::types::grant_emission_permission::Allocation,
                targets: crate::interfaces::mainnet::api::permission0::calls::types::grant_emission_permission::Targets,
                distribution: crate::interfaces::mainnet::api::permission0::calls::types::grant_emission_permission::Distribution,
                duration: crate::interfaces::mainnet::api::permission0::calls::types::grant_emission_permission::Duration,
                revocation: crate::interfaces::mainnet::api::permission0::calls::types::grant_emission_permission::Revocation,
                enforcement: crate::interfaces::mainnet::api::permission0::calls::types::grant_emission_permission::Enforcement,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .permission0()
                    .grant_emission_permission(
                        grantee,
                        allocation,
                        targets,
                        distribution,
                        duration,
                        revocation,
                        enforcement,
                    );
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn revoke_permission(
                &self,
                permission_id: crate::interfaces::mainnet::api::permission0::calls::types::revoke_permission::PermissionId,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .permission0()
                    .revoke_permission(permission_id);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn revoke_permission_wait(
                &self,
                permission_id: crate::interfaces::mainnet::api::permission0::calls::types::revoke_permission::PermissionId,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .permission0()
                    .revoke_permission(permission_id);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn execute_permission(
                &self,
                permission_id: crate::interfaces::mainnet::api::permission0::calls::types::execute_permission::PermissionId,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .permission0()
                    .execute_permission(permission_id);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn execute_permission_wait(
                &self,
                permission_id: crate::interfaces::mainnet::api::permission0::calls::types::execute_permission::PermissionId,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .permission0()
                    .execute_permission(permission_id);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn toggle_permission_accumulation(
                &self,
                permission_id: crate::interfaces::mainnet::api::permission0::calls::types::toggle_permission_accumulation::PermissionId,
                accumulating: crate::interfaces::mainnet::api::permission0::calls::types::toggle_permission_accumulation::Accumulating,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .permission0()
                    .toggle_permission_accumulation(permission_id, accumulating);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn toggle_permission_accumulation_wait(
                &self,
                permission_id: crate::interfaces::mainnet::api::permission0::calls::types::toggle_permission_accumulation::PermissionId,
                accumulating: crate::interfaces::mainnet::api::permission0::calls::types::toggle_permission_accumulation::Accumulating,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .permission0()
                    .toggle_permission_accumulation(permission_id, accumulating);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn enforcement_execute_permission(
                &self,
                permission_id: crate::interfaces::mainnet::api::permission0::calls::types::enforcement_execute_permission::PermissionId,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .permission0()
                    .enforcement_execute_permission(permission_id);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn enforcement_execute_permission_wait(
                &self,
                permission_id: crate::interfaces::mainnet::api::permission0::calls::types::enforcement_execute_permission::PermissionId,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .permission0()
                    .enforcement_execute_permission(permission_id);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn set_enforcement_authority(
                &self,
                permission_id: crate::interfaces::mainnet::api::permission0::calls::types::set_enforcement_authority::PermissionId,
                enforcement: crate::interfaces::mainnet::api::permission0::calls::types::set_enforcement_authority::Enforcement,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .permission0()
                    .set_enforcement_authority(permission_id, enforcement);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn set_enforcement_authority_wait(
                &self,
                permission_id: crate::interfaces::mainnet::api::permission0::calls::types::set_enforcement_authority::PermissionId,
                enforcement: crate::interfaces::mainnet::api::permission0::calls::types::set_enforcement_authority::Enforcement,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .permission0()
                    .set_enforcement_authority(permission_id, enforcement);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn grant_curator_permission(
                &self,
                grantee: crate::interfaces::mainnet::api::permission0::calls::types::grant_curator_permission::Grantee,
                flags: crate::interfaces::mainnet::api::permission0::calls::types::grant_curator_permission::Flags,
                cooldown: crate::interfaces::mainnet::api::permission0::calls::types::grant_curator_permission::Cooldown,
                duration: crate::interfaces::mainnet::api::permission0::calls::types::grant_curator_permission::Duration,
                revocation: crate::interfaces::mainnet::api::permission0::calls::types::grant_curator_permission::Revocation,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .permission0()
                    .grant_curator_permission(grantee, flags, cooldown, duration, revocation);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn grant_curator_permission_wait(
                &self,
                grantee: crate::interfaces::mainnet::api::permission0::calls::types::grant_curator_permission::Grantee,
                flags: crate::interfaces::mainnet::api::permission0::calls::types::grant_curator_permission::Flags,
                cooldown: crate::interfaces::mainnet::api::permission0::calls::types::grant_curator_permission::Cooldown,
                duration: crate::interfaces::mainnet::api::permission0::calls::types::grant_curator_permission::Duration,
                revocation: crate::interfaces::mainnet::api::permission0::calls::types::grant_curator_permission::Revocation,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .permission0()
                    .grant_curator_permission(grantee, flags, cooldown, duration, revocation);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn grant_namespace_permission(
                &self,
                grantee: crate::interfaces::mainnet::api::permission0::calls::types::grant_namespace_permission::Grantee,
                paths: crate::interfaces::mainnet::api::permission0::calls::types::grant_namespace_permission::Paths,
                duration: crate::interfaces::mainnet::api::permission0::calls::types::grant_namespace_permission::Duration,
                revocation: crate::interfaces::mainnet::api::permission0::calls::types::grant_namespace_permission::Revocation,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .permission0()
                    .grant_namespace_permission(grantee, paths, duration, revocation);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn grant_namespace_permission_wait(
                &self,
                grantee: crate::interfaces::mainnet::api::permission0::calls::types::grant_namespace_permission::Grantee,
                paths: crate::interfaces::mainnet::api::permission0::calls::types::grant_namespace_permission::Paths,
                duration: crate::interfaces::mainnet::api::permission0::calls::types::grant_namespace_permission::Duration,
                revocation: crate::interfaces::mainnet::api::permission0::calls::types::grant_namespace_permission::Revocation,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .permission0()
                    .grant_namespace_permission(grantee, paths, duration, revocation);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn update_emission_permission(
                &self,
                permission_id: crate::interfaces::mainnet::api::permission0::calls::types::update_emission_permission::PermissionId,
                new_targets: crate::interfaces::mainnet::api::permission0::calls::types::update_emission_permission::NewTargets,
                new_streams: crate::interfaces::mainnet::api::permission0::calls::types::update_emission_permission::NewStreams,
                new_distribution_control: crate::interfaces::mainnet::api::permission0::calls::types::update_emission_permission::NewDistributionControl,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .permission0()
                    .update_emission_permission(
                        permission_id,
                        new_targets,
                        new_streams,
                        new_distribution_control,
                    );
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn update_emission_permission_wait(
                &self,
                permission_id: crate::interfaces::mainnet::api::permission0::calls::types::update_emission_permission::PermissionId,
                new_targets: crate::interfaces::mainnet::api::permission0::calls::types::update_emission_permission::NewTargets,
                new_streams: crate::interfaces::mainnet::api::permission0::calls::types::update_emission_permission::NewStreams,
                new_distribution_control: crate::interfaces::mainnet::api::permission0::calls::types::update_emission_permission::NewDistributionControl,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::mainnet::api::tx()
                    .permission0()
                    .update_emission_permission(
                        permission_id,
                        new_targets,
                        new_streams,
                        new_distribution_control,
                    );
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
        }
    }
}
#[cfg(feature = "testnet")]
pub mod testnet {
    use super::*;
    use crate::interfaces::testnet::api::runtime_types;
    pub mod system {
        use super::*;
        impl SystemClient<crate::chain::TestNet> {
            pub fn storage(&self) -> SystemStorage<crate::chain::TestNet> {
                SystemStorage {
                    client: self.client.clone(),
                    block: None,
                    _pd: PhantomData,
                }
            }
            pub fn storage_at(&self, block_hash: H256) -> SystemStorage<crate::chain::TestNet> {
                SystemStorage {
                    client: self.client.clone(),
                    block: Some(block_hash),
                    _pd: PhantomData,
                }
            }
        }
        impl SystemStorage<crate::chain::TestNet> {
            pub async fn inherents_applied(
                &self,
            ) -> crate::Result<Option<::core::primitive::bool>> {
                let call = crate::interfaces::testnet::api::storage()
                    .system()
                    .inherents_applied();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn extrinsic_data_get(
                &self,
                key: &::core::primitive::u32,
            ) -> crate::Result<
                Option<::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>>,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .system()
                    .extrinsic_data(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn extrinsic_data_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::core::primitive::u32,
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .system()
                    .extrinsic_data_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::core::primitive::u32 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn block_weight(
                &self,
            ) -> crate::Result<
                Option<
                    runtime_types::frame_support::dispatch::PerDispatchClass<
                        runtime_types::sp_weights::weight_v2::Weight,
                    >,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .system()
                    .block_weight();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn authorized_upgrade(
                &self,
            ) -> crate::Result<Option<runtime_types::frame_system::CodeUpgradeAuthorization>>
            {
                let call = crate::interfaces::testnet::api::storage()
                    .system()
                    .authorized_upgrade();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn extrinsic_count(&self) -> crate::Result<Option<::core::primitive::u32>> {
                let call = crate::interfaces::testnet::api::storage()
                    .system()
                    .extrinsic_count();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn event_topics_get(
                &self,
                key: &::subxt::ext::subxt_core::utils::H256,
            ) -> crate::Result<
                Option<
                    ::subxt::ext::subxt_core::alloc::vec::Vec<(
                        ::core::primitive::u64,
                        ::core::primitive::u32,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .system()
                    .event_topics(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn event_topics_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::subxt::ext::subxt_core::utils::H256,
                        ::subxt::ext::subxt_core::alloc::vec::Vec<(
                            ::core::primitive::u64,
                            ::core::primitive::u32,
                        )>,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .system()
                    .event_topics_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::subxt::ext::subxt_core::utils::H256 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn execution_phase(
                &self,
            ) -> crate::Result<Option<runtime_types::frame_system::Phase>> {
                let call = crate::interfaces::testnet::api::storage()
                    .system()
                    .execution_phase();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn last_runtime_upgrade(
                &self,
            ) -> crate::Result<Option<runtime_types::frame_system::LastRuntimeUpgradeInfo>>
            {
                let call = crate::interfaces::testnet::api::storage()
                    .system()
                    .last_runtime_upgrade();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn upgraded_to_u32_ref_count(
                &self,
            ) -> crate::Result<Option<::core::primitive::bool>> {
                let call = crate::interfaces::testnet::api::storage()
                    .system()
                    .upgraded_to_u32_ref_count();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn upgraded_to_triple_ref_count(
                &self,
            ) -> crate::Result<Option<::core::primitive::bool>> {
                let call = crate::interfaces::testnet::api::storage()
                    .system()
                    .upgraded_to_triple_ref_count();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn events(
                &self,
            ) -> crate::Result<
                Option<
                    ::subxt::ext::subxt_core::alloc::vec::Vec<
                        runtime_types::frame_system::EventRecord<
                            runtime_types::torus_runtime::RuntimeEvent,
                            ::subxt::ext::subxt_core::utils::H256,
                        >,
                    >,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage().system().events();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn event_count(&self) -> crate::Result<Option<::core::primitive::u32>> {
                let call = crate::interfaces::testnet::api::storage()
                    .system()
                    .event_count();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn block_hash_get(
                &self,
                key: &::core::primitive::u64,
            ) -> crate::Result<Option<::subxt::ext::subxt_core::utils::H256>> {
                let call = crate::interfaces::testnet::api::storage()
                    .system()
                    .block_hash(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn block_hash_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::core::primitive::u64,
                        ::subxt::ext::subxt_core::utils::H256,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .system()
                    .block_hash_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::core::primitive::u64 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn digest(
                &self,
            ) -> crate::Result<Option<runtime_types::sp_runtime::generic::digest::Digest>>
            {
                let call = crate::interfaces::testnet::api::storage().system().digest();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn number(&self) -> crate::Result<Option<::core::primitive::u64>> {
                let call = crate::interfaces::testnet::api::storage().system().number();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn all_extrinsics_len(
                &self,
            ) -> crate::Result<Option<::core::primitive::u32>> {
                let call = crate::interfaces::testnet::api::storage()
                    .system()
                    .all_extrinsics_len();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn parent_hash(
                &self,
            ) -> crate::Result<Option<::subxt::ext::subxt_core::utils::H256>> {
                let call = crate::interfaces::testnet::api::storage()
                    .system()
                    .parent_hash();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn account_get(
                &self,
                key: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<
                Option<
                    runtime_types::frame_system::AccountInfo<
                        ::core::primitive::u32,
                        runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
                    >,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .system()
                    .account(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn account_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        runtime_types::frame_system::AccountInfo<
                            ::core::primitive::u32,
                            runtime_types::pallet_balances::types::AccountData<
                                ::core::primitive::u128,
                            >,
                        >,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .system()
                    .account_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::subxt::ext::subxt_core::utils::AccountId32 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
        }
        impl SystemClient<crate::chain::TestNet> {
            pub fn calls(&self) -> SystemCalls<crate::chain::TestNet> {
                SystemCalls {
                    client: self.client.clone(),
                    _pd: PhantomData,
                }
            }
        }
        impl SystemCalls<crate::chain::TestNet> {
            pub async fn remark(
                &self,
                remark: crate::interfaces::testnet::api::system::calls::types::remark::Remark,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .system()
                    .remark(remark);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn remark_wait(
                &self,
                remark: crate::interfaces::testnet::api::system::calls::types::remark::Remark,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .system()
                    .remark(remark);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn set_heap_pages(
                &self,
                pages: crate::interfaces::testnet::api::system::calls::types::set_heap_pages::Pages,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .system()
                    .set_heap_pages(pages);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn set_heap_pages_wait(
                &self,
                pages: crate::interfaces::testnet::api::system::calls::types::set_heap_pages::Pages,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .system()
                    .set_heap_pages(pages);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn set_code(
                &self,
                code: crate::interfaces::testnet::api::system::calls::types::set_code::Code,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .system()
                    .set_code(code);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn set_code_wait(
                &self,
                code: crate::interfaces::testnet::api::system::calls::types::set_code::Code,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .system()
                    .set_code(code);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn set_code_without_checks(
                &self,
                code: crate::interfaces::testnet::api::system::calls::types::set_code_without_checks::Code,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .system()
                    .set_code_without_checks(code);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn set_code_without_checks_wait(
                &self,
                code: crate::interfaces::testnet::api::system::calls::types::set_code_without_checks::Code,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .system()
                    .set_code_without_checks(code);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn set_storage(
                &self,
                items: crate::interfaces::testnet::api::system::calls::types::set_storage::Items,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .system()
                    .set_storage(items);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn set_storage_wait(
                &self,
                items: crate::interfaces::testnet::api::system::calls::types::set_storage::Items,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .system()
                    .set_storage(items);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn kill_storage(
                &self,
                keys: crate::interfaces::testnet::api::system::calls::types::kill_storage::Keys,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .system()
                    .kill_storage(keys);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn kill_storage_wait(
                &self,
                keys: crate::interfaces::testnet::api::system::calls::types::kill_storage::Keys,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .system()
                    .kill_storage(keys);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn kill_prefix(
                &self,
                prefix: crate::interfaces::testnet::api::system::calls::types::kill_prefix::Prefix,
                subkeys: crate::interfaces::testnet::api::system::calls::types::kill_prefix::Subkeys,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .system()
                    .kill_prefix(prefix, subkeys);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn kill_prefix_wait(
                &self,
                prefix: crate::interfaces::testnet::api::system::calls::types::kill_prefix::Prefix,
                subkeys: crate::interfaces::testnet::api::system::calls::types::kill_prefix::Subkeys,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .system()
                    .kill_prefix(prefix, subkeys);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn remark_with_event(
                &self,
                remark: crate::interfaces::testnet::api::system::calls::types::remark_with_event::Remark,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .system()
                    .remark_with_event(remark);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn remark_with_event_wait(
                &self,
                remark: crate::interfaces::testnet::api::system::calls::types::remark_with_event::Remark,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .system()
                    .remark_with_event(remark);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn do_task(
                &self,
                task: crate::interfaces::testnet::api::system::calls::types::do_task::Task,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx().system().do_task(task);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn do_task_wait(
                &self,
                task: crate::interfaces::testnet::api::system::calls::types::do_task::Task,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx().system().do_task(task);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn authorize_upgrade(
                &self,
                code_hash: crate::interfaces::testnet::api::system::calls::types::authorize_upgrade::CodeHash,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .system()
                    .authorize_upgrade(code_hash);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn authorize_upgrade_wait(
                &self,
                code_hash: crate::interfaces::testnet::api::system::calls::types::authorize_upgrade::CodeHash,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .system()
                    .authorize_upgrade(code_hash);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn authorize_upgrade_without_checks(
                &self,
                code_hash: crate::interfaces::testnet::api::system::calls::types::authorize_upgrade_without_checks::CodeHash,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .system()
                    .authorize_upgrade_without_checks(code_hash);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn authorize_upgrade_without_checks_wait(
                &self,
                code_hash: crate::interfaces::testnet::api::system::calls::types::authorize_upgrade_without_checks::CodeHash,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .system()
                    .authorize_upgrade_without_checks(code_hash);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn apply_authorized_upgrade(
                &self,
                code: crate::interfaces::testnet::api::system::calls::types::apply_authorized_upgrade::Code,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .system()
                    .apply_authorized_upgrade(code);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn apply_authorized_upgrade_wait(
                &self,
                code: crate::interfaces::testnet::api::system::calls::types::apply_authorized_upgrade::Code,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .system()
                    .apply_authorized_upgrade(code);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
        }
    }
    pub mod timestamp {
        use super::*;
        impl TimestampClient<crate::chain::TestNet> {
            pub fn storage(&self) -> TimestampStorage<crate::chain::TestNet> {
                TimestampStorage {
                    client: self.client.clone(),
                    block: None,
                    _pd: PhantomData,
                }
            }
            pub fn storage_at(&self, block_hash: H256) -> TimestampStorage<crate::chain::TestNet> {
                TimestampStorage {
                    client: self.client.clone(),
                    block: Some(block_hash),
                    _pd: PhantomData,
                }
            }
        }
        impl TimestampStorage<crate::chain::TestNet> {
            pub async fn now(&self) -> crate::Result<Option<::core::primitive::u64>> {
                let call = crate::interfaces::testnet::api::storage().timestamp().now();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn did_update(&self) -> crate::Result<Option<::core::primitive::bool>> {
                let call = crate::interfaces::testnet::api::storage()
                    .timestamp()
                    .did_update();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
        }
        impl TimestampClient<crate::chain::TestNet> {
            pub fn calls(&self) -> TimestampCalls<crate::chain::TestNet> {
                TimestampCalls {
                    client: self.client.clone(),
                    _pd: PhantomData,
                }
            }
        }
        impl TimestampCalls<crate::chain::TestNet> {
            pub async fn set(
                &self,
                now: crate::interfaces::testnet::api::timestamp::calls::types::set::Now,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx().timestamp().set(now);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn set_wait(
                &self,
                now: crate::interfaces::testnet::api::timestamp::calls::types::set::Now,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx().timestamp().set(now);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
        }
    }
    pub mod aura {
        use super::*;
        impl AuraClient<crate::chain::TestNet> {
            pub fn storage(&self) -> AuraStorage<crate::chain::TestNet> {
                AuraStorage {
                    client: self.client.clone(),
                    block: None,
                    _pd: PhantomData,
                }
            }
            pub fn storage_at(&self, block_hash: H256) -> AuraStorage<crate::chain::TestNet> {
                AuraStorage {
                    client: self.client.clone(),
                    block: Some(block_hash),
                    _pd: PhantomData,
                }
            }
        }
        impl AuraStorage<crate::chain::TestNet> {
            pub async fn authorities(
                &self,
            ) -> crate::Result<
                Option<
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
                    >,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .aura()
                    .authorities();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn current_slot(
                &self,
            ) -> crate::Result<Option<runtime_types::sp_consensus_slots::Slot>> {
                let call = crate::interfaces::testnet::api::storage()
                    .aura()
                    .current_slot();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
        }
    }
    pub mod grandpa {
        use super::*;
        impl GrandpaClient<crate::chain::TestNet> {
            pub fn storage(&self) -> GrandpaStorage<crate::chain::TestNet> {
                GrandpaStorage {
                    client: self.client.clone(),
                    block: None,
                    _pd: PhantomData,
                }
            }
            pub fn storage_at(&self, block_hash: H256) -> GrandpaStorage<crate::chain::TestNet> {
                GrandpaStorage {
                    client: self.client.clone(),
                    block: Some(block_hash),
                    _pd: PhantomData,
                }
            }
        }
        impl GrandpaStorage<crate::chain::TestNet> {
            pub async fn authorities(
                &self,
            ) -> crate::Result<
                Option<
                    runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_consensus_grandpa::app::Public,
                        ::core::primitive::u64,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .grandpa()
                    .authorities();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn stalled(
                &self,
            ) -> crate::Result<Option<(::core::primitive::u64, ::core::primitive::u64)>>
            {
                let call = crate::interfaces::testnet::api::storage()
                    .grandpa()
                    .stalled();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn state(
                &self,
            ) -> crate::Result<
                Option<runtime_types::pallet_grandpa::StoredState<::core::primitive::u64>>,
            > {
                let call = crate::interfaces::testnet::api::storage().grandpa().state();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn current_set_id(&self) -> crate::Result<Option<::core::primitive::u64>> {
                let call = crate::interfaces::testnet::api::storage()
                    .grandpa()
                    .current_set_id();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn pending_change(
                &self,
            ) -> crate::Result<
                Option<runtime_types::pallet_grandpa::StoredPendingChange<::core::primitive::u64>>,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .grandpa()
                    .pending_change();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn next_forced(&self) -> crate::Result<Option<::core::primitive::u64>> {
                let call = crate::interfaces::testnet::api::storage()
                    .grandpa()
                    .next_forced();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn set_id_session_get(
                &self,
                key: &::core::primitive::u64,
            ) -> crate::Result<Option<::core::primitive::u32>> {
                let call = crate::interfaces::testnet::api::storage()
                    .grandpa()
                    .set_id_session(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn set_id_session_iter(
                &self,
            ) -> crate::Result<
                impl Stream<Item = crate::Result<(::core::primitive::u64, ::core::primitive::u32)>>,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .grandpa()
                    .set_id_session_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::core::primitive::u64 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
        }
        impl GrandpaClient<crate::chain::TestNet> {
            pub fn calls(&self) -> GrandpaCalls<crate::chain::TestNet> {
                GrandpaCalls {
                    client: self.client.clone(),
                    _pd: PhantomData,
                }
            }
        }
        impl GrandpaCalls<crate::chain::TestNet> {
            pub async fn report_equivocation(
                &self,
                equivocation_proof: crate::interfaces::testnet::api::grandpa::calls::types::report_equivocation::EquivocationProof,
                key_owner_proof: crate::interfaces::testnet::api::grandpa::calls::types::report_equivocation::KeyOwnerProof,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .grandpa()
                    .report_equivocation(equivocation_proof, key_owner_proof);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn report_equivocation_wait(
                &self,
                equivocation_proof: crate::interfaces::testnet::api::grandpa::calls::types::report_equivocation::EquivocationProof,
                key_owner_proof: crate::interfaces::testnet::api::grandpa::calls::types::report_equivocation::KeyOwnerProof,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .grandpa()
                    .report_equivocation(equivocation_proof, key_owner_proof);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn report_equivocation_unsigned(
                &self,
                equivocation_proof: crate::interfaces::testnet::api::grandpa::calls::types::report_equivocation_unsigned::EquivocationProof,
                key_owner_proof: crate::interfaces::testnet::api::grandpa::calls::types::report_equivocation_unsigned::KeyOwnerProof,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .grandpa()
                    .report_equivocation_unsigned(equivocation_proof, key_owner_proof);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn report_equivocation_unsigned_wait(
                &self,
                equivocation_proof: crate::interfaces::testnet::api::grandpa::calls::types::report_equivocation_unsigned::EquivocationProof,
                key_owner_proof: crate::interfaces::testnet::api::grandpa::calls::types::report_equivocation_unsigned::KeyOwnerProof,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .grandpa()
                    .report_equivocation_unsigned(equivocation_proof, key_owner_proof);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn note_stalled(
                &self,
                delay: crate::interfaces::testnet::api::grandpa::calls::types::note_stalled::Delay,
                best_finalized_block_number: crate::interfaces::testnet::api::grandpa::calls::types::note_stalled::BestFinalizedBlockNumber,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .grandpa()
                    .note_stalled(delay, best_finalized_block_number);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn note_stalled_wait(
                &self,
                delay: crate::interfaces::testnet::api::grandpa::calls::types::note_stalled::Delay,
                best_finalized_block_number: crate::interfaces::testnet::api::grandpa::calls::types::note_stalled::BestFinalizedBlockNumber,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .grandpa()
                    .note_stalled(delay, best_finalized_block_number);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
        }
    }
    pub mod balances {
        use super::*;
        impl BalancesClient<crate::chain::TestNet> {
            pub fn storage(&self) -> BalancesStorage<crate::chain::TestNet> {
                BalancesStorage {
                    client: self.client.clone(),
                    block: None,
                    _pd: PhantomData,
                }
            }
            pub fn storage_at(&self, block_hash: H256) -> BalancesStorage<crate::chain::TestNet> {
                BalancesStorage {
                    client: self.client.clone(),
                    block: Some(block_hash),
                    _pd: PhantomData,
                }
            }
        }
        impl BalancesStorage<crate::chain::TestNet> {
            pub async fn reserves_get(
                &self,
                key: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<
                Option<
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::ReserveData<
                            (),
                            ::core::primitive::u128,
                        >,
                    >,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .balances()
                    .reserves(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn reserves_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            runtime_types::pallet_balances::types::ReserveData<
                                (),
                                ::core::primitive::u128,
                            >,
                        >,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .balances()
                    .reserves_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::subxt::ext::subxt_core::utils::AccountId32 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn inactive_issuance(
                &self,
            ) -> crate::Result<Option<::core::primitive::u128>> {
                let call = crate::interfaces::testnet::api::storage()
                    .balances()
                    .inactive_issuance();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn freezes_get(
                &self,
                key: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<
                Option<
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::frame_support::traits::tokens::misc::IdAmount<
                            (),
                            ::core::primitive::u128,
                        >,
                    >,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .balances()
                    .freezes(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn freezes_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            runtime_types::frame_support::traits::tokens::misc::IdAmount<
                                (),
                                ::core::primitive::u128,
                            >,
                        >,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .balances()
                    .freezes_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::subxt::ext::subxt_core::utils::AccountId32 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn locks_get(
                &self,
                key: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<
                Option<
                    runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::pallet_balances::types::BalanceLock<::core::primitive::u128>,
                    >,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .balances()
                    .locks(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn locks_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
                            runtime_types::pallet_balances::types::BalanceLock<
                                ::core::primitive::u128,
                            >,
                        >,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .balances()
                    .locks_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::subxt::ext::subxt_core::utils::AccountId32 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn holds_get(
                &self,
                key: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<
                Option<
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::frame_support::traits::tokens::misc::IdAmount<
                            runtime_types::torus_runtime::RuntimeHoldReason,
                            ::core::primitive::u128,
                        >,
                    >,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .balances()
                    .holds(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn holds_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            runtime_types::frame_support::traits::tokens::misc::IdAmount<
                                runtime_types::torus_runtime::RuntimeHoldReason,
                                ::core::primitive::u128,
                            >,
                        >,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .balances()
                    .holds_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::subxt::ext::subxt_core::utils::AccountId32 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn total_issuance(&self) -> crate::Result<Option<::core::primitive::u128>> {
                let call = crate::interfaces::testnet::api::storage()
                    .balances()
                    .total_issuance();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn account_get(
                &self,
                key: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<
                Option<runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>>,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .balances()
                    .account(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn account_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .balances()
                    .account_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::subxt::ext::subxt_core::utils::AccountId32 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
        }
        impl BalancesClient<crate::chain::TestNet> {
            pub fn calls(&self) -> BalancesCalls<crate::chain::TestNet> {
                BalancesCalls {
                    client: self.client.clone(),
                    _pd: PhantomData,
                }
            }
        }
        impl BalancesCalls<crate::chain::TestNet> {
            pub async fn transfer_allow_death(
                &self,
                dest: crate::interfaces::testnet::api::balances::calls::types::transfer_allow_death::Dest,
                value: crate::interfaces::testnet::api::balances::calls::types::transfer_allow_death::Value,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .balances()
                    .transfer_allow_death(dest, value);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn transfer_allow_death_wait(
                &self,
                dest: crate::interfaces::testnet::api::balances::calls::types::transfer_allow_death::Dest,
                value: crate::interfaces::testnet::api::balances::calls::types::transfer_allow_death::Value,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .balances()
                    .transfer_allow_death(dest, value);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn force_transfer(
                &self,
                source: crate::interfaces::testnet::api::balances::calls::types::force_transfer::Source,
                dest: crate::interfaces::testnet::api::balances::calls::types::force_transfer::Dest,
                value: crate::interfaces::testnet::api::balances::calls::types::force_transfer::Value,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .balances()
                    .force_transfer(source, dest, value);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn force_transfer_wait(
                &self,
                source: crate::interfaces::testnet::api::balances::calls::types::force_transfer::Source,
                dest: crate::interfaces::testnet::api::balances::calls::types::force_transfer::Dest,
                value: crate::interfaces::testnet::api::balances::calls::types::force_transfer::Value,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .balances()
                    .force_transfer(source, dest, value);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn transfer_keep_alive(
                &self,
                dest: crate::interfaces::testnet::api::balances::calls::types::transfer_keep_alive::Dest,
                value: crate::interfaces::testnet::api::balances::calls::types::transfer_keep_alive::Value,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .balances()
                    .transfer_keep_alive(dest, value);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn transfer_keep_alive_wait(
                &self,
                dest: crate::interfaces::testnet::api::balances::calls::types::transfer_keep_alive::Dest,
                value: crate::interfaces::testnet::api::balances::calls::types::transfer_keep_alive::Value,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .balances()
                    .transfer_keep_alive(dest, value);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn transfer_all(
                &self,
                dest: crate::interfaces::testnet::api::balances::calls::types::transfer_all::Dest,
                keep_alive: crate::interfaces::testnet::api::balances::calls::types::transfer_all::KeepAlive,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .balances()
                    .transfer_all(dest, keep_alive);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn transfer_all_wait(
                &self,
                dest: crate::interfaces::testnet::api::balances::calls::types::transfer_all::Dest,
                keep_alive: crate::interfaces::testnet::api::balances::calls::types::transfer_all::KeepAlive,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .balances()
                    .transfer_all(dest, keep_alive);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn force_unreserve(
                &self,
                who: crate::interfaces::testnet::api::balances::calls::types::force_unreserve::Who,
                amount: crate::interfaces::testnet::api::balances::calls::types::force_unreserve::Amount,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .balances()
                    .force_unreserve(who, amount);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn force_unreserve_wait(
                &self,
                who: crate::interfaces::testnet::api::balances::calls::types::force_unreserve::Who,
                amount: crate::interfaces::testnet::api::balances::calls::types::force_unreserve::Amount,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .balances()
                    .force_unreserve(who, amount);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn upgrade_accounts(
                &self,
                who: crate::interfaces::testnet::api::balances::calls::types::upgrade_accounts::Who,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .balances()
                    .upgrade_accounts(who);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn upgrade_accounts_wait(
                &self,
                who: crate::interfaces::testnet::api::balances::calls::types::upgrade_accounts::Who,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .balances()
                    .upgrade_accounts(who);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn force_set_balance(
                &self,
                who: crate::interfaces::testnet::api::balances::calls::types::force_set_balance::Who,
                new_free: crate::interfaces::testnet::api::balances::calls::types::force_set_balance::NewFree,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .balances()
                    .force_set_balance(who, new_free);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn force_set_balance_wait(
                &self,
                who: crate::interfaces::testnet::api::balances::calls::types::force_set_balance::Who,
                new_free: crate::interfaces::testnet::api::balances::calls::types::force_set_balance::NewFree,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .balances()
                    .force_set_balance(who, new_free);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn force_adjust_total_issuance(
                &self,
                direction: crate::interfaces::testnet::api::balances::calls::types::force_adjust_total_issuance::Direction,
                delta: crate::interfaces::testnet::api::balances::calls::types::force_adjust_total_issuance::Delta,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .balances()
                    .force_adjust_total_issuance(direction, delta);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn force_adjust_total_issuance_wait(
                &self,
                direction: crate::interfaces::testnet::api::balances::calls::types::force_adjust_total_issuance::Direction,
                delta: crate::interfaces::testnet::api::balances::calls::types::force_adjust_total_issuance::Delta,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .balances()
                    .force_adjust_total_issuance(direction, delta);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn burn(
                &self,
                value: crate::interfaces::testnet::api::balances::calls::types::burn::Value,
                keep_alive: crate::interfaces::testnet::api::balances::calls::types::burn::KeepAlive,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .balances()
                    .burn(value, keep_alive);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn burn_wait(
                &self,
                value: crate::interfaces::testnet::api::balances::calls::types::burn::Value,
                keep_alive: crate::interfaces::testnet::api::balances::calls::types::burn::KeepAlive,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .balances()
                    .burn(value, keep_alive);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
        }
    }
    pub mod transaction_payment {
        use super::*;
        impl TransactionPaymentClient<crate::chain::TestNet> {
            pub fn storage(&self) -> TransactionPaymentStorage<crate::chain::TestNet> {
                TransactionPaymentStorage {
                    client: self.client.clone(),
                    block: None,
                    _pd: PhantomData,
                }
            }
            pub fn storage_at(
                &self,
                block_hash: H256,
            ) -> TransactionPaymentStorage<crate::chain::TestNet> {
                TransactionPaymentStorage {
                    client: self.client.clone(),
                    block: Some(block_hash),
                    _pd: PhantomData,
                }
            }
        }
        impl TransactionPaymentStorage<crate::chain::TestNet> {
            pub async fn next_fee_multiplier(
                &self,
            ) -> crate::Result<Option<runtime_types::sp_arithmetic::fixed_point::FixedU128>>
            {
                let call = crate::interfaces::testnet::api::storage()
                    .transaction_payment()
                    .next_fee_multiplier();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn storage_version(
                &self,
            ) -> crate::Result<Option<runtime_types::pallet_transaction_payment::Releases>>
            {
                let call = crate::interfaces::testnet::api::storage()
                    .transaction_payment()
                    .storage_version();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
        }
    }
    pub mod sudo {
        use super::*;
        impl SudoClient<crate::chain::TestNet> {
            pub fn storage(&self) -> SudoStorage<crate::chain::TestNet> {
                SudoStorage {
                    client: self.client.clone(),
                    block: None,
                    _pd: PhantomData,
                }
            }
            pub fn storage_at(&self, block_hash: H256) -> SudoStorage<crate::chain::TestNet> {
                SudoStorage {
                    client: self.client.clone(),
                    block: Some(block_hash),
                    _pd: PhantomData,
                }
            }
        }
        impl SudoStorage<crate::chain::TestNet> {
            pub async fn key(
                &self,
            ) -> crate::Result<Option<::subxt::ext::subxt_core::utils::AccountId32>> {
                let call = crate::interfaces::testnet::api::storage().sudo().key();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
        }
        impl SudoClient<crate::chain::TestNet> {
            pub fn calls(&self) -> SudoCalls<crate::chain::TestNet> {
                SudoCalls {
                    client: self.client.clone(),
                    _pd: PhantomData,
                }
            }
        }
        impl SudoCalls<crate::chain::TestNet> {
            pub async fn sudo(
                &self,
                call: crate::interfaces::testnet::api::sudo::calls::types::sudo::Call,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx().sudo().sudo(call);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn sudo_wait(
                &self,
                call: crate::interfaces::testnet::api::sudo::calls::types::sudo::Call,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx().sudo().sudo(call);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn sudo_unchecked_weight(
                &self,
                call: crate::interfaces::testnet::api::sudo::calls::types::sudo_unchecked_weight::Call,
                weight: crate::interfaces::testnet::api::sudo::calls::types::sudo_unchecked_weight::Weight,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .sudo()
                    .sudo_unchecked_weight(call, weight);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn sudo_unchecked_weight_wait(
                &self,
                call: crate::interfaces::testnet::api::sudo::calls::types::sudo_unchecked_weight::Call,
                weight: crate::interfaces::testnet::api::sudo::calls::types::sudo_unchecked_weight::Weight,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .sudo()
                    .sudo_unchecked_weight(call, weight);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn set_key(
                &self,
                new: crate::interfaces::testnet::api::sudo::calls::types::set_key::New,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx().sudo().set_key(new);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn set_key_wait(
                &self,
                new: crate::interfaces::testnet::api::sudo::calls::types::set_key::New,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx().sudo().set_key(new);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn sudo_as(
                &self,
                who: crate::interfaces::testnet::api::sudo::calls::types::sudo_as::Who,
                call: crate::interfaces::testnet::api::sudo::calls::types::sudo_as::Call,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .sudo()
                    .sudo_as(who, call);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn sudo_as_wait(
                &self,
                who: crate::interfaces::testnet::api::sudo::calls::types::sudo_as::Who,
                call: crate::interfaces::testnet::api::sudo::calls::types::sudo_as::Call,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .sudo()
                    .sudo_as(who, call);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn remove_key(
                &self,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx().sudo().remove_key();
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn remove_key_wait(
                &self,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx().sudo().remove_key();
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
        }
    }
    pub mod multisig {
        use super::*;
        impl MultisigClient<crate::chain::TestNet> {
            pub fn storage(&self) -> MultisigStorage<crate::chain::TestNet> {
                MultisigStorage {
                    client: self.client.clone(),
                    block: None,
                    _pd: PhantomData,
                }
            }
            pub fn storage_at(&self, block_hash: H256) -> MultisigStorage<crate::chain::TestNet> {
                MultisigStorage {
                    client: self.client.clone(),
                    block: Some(block_hash),
                    _pd: PhantomData,
                }
            }
        }
        impl MultisigStorage<crate::chain::TestNet> {
            pub async fn multisigs_get(
                &self,
                key1: &::subxt::ext::subxt_core::utils::AccountId32,
                key2: &[::core::primitive::u8; 32usize],
            ) -> crate::Result<
                Option<
                    runtime_types::pallet_multisig::Multisig<
                        ::core::primitive::u64,
                        ::core::primitive::u128,
                        ::subxt::ext::subxt_core::utils::AccountId32,
                    >,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .multisig()
                    .multisigs(key1, key2);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn multisigs_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        (
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            [::core::primitive::u8; 32usize],
                        ),
                        runtime_types::pallet_multisig::Multisig<
                            ::core::primitive::u64,
                            ::core::primitive::u128,
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        >,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .multisig()
                    .multisigs_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let pair = <(
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            [::core::primitive::u8; 32usize],
                        ) as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((pair, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn multisigs_iter1(
                &self,
                key1: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        (
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            [::core::primitive::u8; 32usize],
                        ),
                        runtime_types::pallet_multisig::Multisig<
                            ::core::primitive::u64,
                            ::core::primitive::u128,
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        >,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .multisig()
                    .multisigs_iter1(key1);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let pair = <(
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            [::core::primitive::u8; 32usize],
                        ) as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((pair, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
        }
        impl MultisigClient<crate::chain::TestNet> {
            pub fn calls(&self) -> MultisigCalls<crate::chain::TestNet> {
                MultisigCalls {
                    client: self.client.clone(),
                    _pd: PhantomData,
                }
            }
        }
        impl MultisigCalls<crate::chain::TestNet> {
            pub async fn as_multi_threshold_1(
                &self,
                other_signatories: crate::interfaces::testnet::api::multisig::calls::types::as_multi_threshold1::OtherSignatories,
                call: crate::interfaces::testnet::api::multisig::calls::types::as_multi_threshold1::Call,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .multisig()
                    .as_multi_threshold_1(other_signatories, call);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn as_multi_threshold_1_wait(
                &self,
                other_signatories: crate::interfaces::testnet::api::multisig::calls::types::as_multi_threshold1::OtherSignatories,
                call: crate::interfaces::testnet::api::multisig::calls::types::as_multi_threshold1::Call,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .multisig()
                    .as_multi_threshold_1(other_signatories, call);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn as_multi(
                &self,
                threshold: crate::interfaces::testnet::api::multisig::calls::types::as_multi::Threshold,
                other_signatories: crate::interfaces::testnet::api::multisig::calls::types::as_multi::OtherSignatories,
                maybe_timepoint: crate::interfaces::testnet::api::multisig::calls::types::as_multi::MaybeTimepoint,
                call: crate::interfaces::testnet::api::multisig::calls::types::as_multi::Call,
                max_weight: crate::interfaces::testnet::api::multisig::calls::types::as_multi::MaxWeight,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx().multisig().as_multi(
                    threshold,
                    other_signatories,
                    maybe_timepoint,
                    call,
                    max_weight,
                );
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn as_multi_wait(
                &self,
                threshold: crate::interfaces::testnet::api::multisig::calls::types::as_multi::Threshold,
                other_signatories: crate::interfaces::testnet::api::multisig::calls::types::as_multi::OtherSignatories,
                maybe_timepoint: crate::interfaces::testnet::api::multisig::calls::types::as_multi::MaybeTimepoint,
                call: crate::interfaces::testnet::api::multisig::calls::types::as_multi::Call,
                max_weight: crate::interfaces::testnet::api::multisig::calls::types::as_multi::MaxWeight,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx().multisig().as_multi(
                    threshold,
                    other_signatories,
                    maybe_timepoint,
                    call,
                    max_weight,
                );
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn approve_as_multi(
                &self,
                threshold: crate::interfaces::testnet::api::multisig::calls::types::approve_as_multi::Threshold,
                other_signatories: crate::interfaces::testnet::api::multisig::calls::types::approve_as_multi::OtherSignatories,
                maybe_timepoint: crate::interfaces::testnet::api::multisig::calls::types::approve_as_multi::MaybeTimepoint,
                call_hash: crate::interfaces::testnet::api::multisig::calls::types::approve_as_multi::CallHash,
                max_weight: crate::interfaces::testnet::api::multisig::calls::types::approve_as_multi::MaxWeight,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .multisig()
                    .approve_as_multi(
                        threshold,
                        other_signatories,
                        maybe_timepoint,
                        call_hash,
                        max_weight,
                    );
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn approve_as_multi_wait(
                &self,
                threshold: crate::interfaces::testnet::api::multisig::calls::types::approve_as_multi::Threshold,
                other_signatories: crate::interfaces::testnet::api::multisig::calls::types::approve_as_multi::OtherSignatories,
                maybe_timepoint: crate::interfaces::testnet::api::multisig::calls::types::approve_as_multi::MaybeTimepoint,
                call_hash: crate::interfaces::testnet::api::multisig::calls::types::approve_as_multi::CallHash,
                max_weight: crate::interfaces::testnet::api::multisig::calls::types::approve_as_multi::MaxWeight,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .multisig()
                    .approve_as_multi(
                        threshold,
                        other_signatories,
                        maybe_timepoint,
                        call_hash,
                        max_weight,
                    );
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn cancel_as_multi(
                &self,
                threshold: crate::interfaces::testnet::api::multisig::calls::types::cancel_as_multi::Threshold,
                other_signatories: crate::interfaces::testnet::api::multisig::calls::types::cancel_as_multi::OtherSignatories,
                timepoint: crate::interfaces::testnet::api::multisig::calls::types::cancel_as_multi::Timepoint,
                call_hash: crate::interfaces::testnet::api::multisig::calls::types::cancel_as_multi::CallHash,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .multisig()
                    .cancel_as_multi(threshold, other_signatories, timepoint, call_hash);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn cancel_as_multi_wait(
                &self,
                threshold: crate::interfaces::testnet::api::multisig::calls::types::cancel_as_multi::Threshold,
                other_signatories: crate::interfaces::testnet::api::multisig::calls::types::cancel_as_multi::OtherSignatories,
                timepoint: crate::interfaces::testnet::api::multisig::calls::types::cancel_as_multi::Timepoint,
                call_hash: crate::interfaces::testnet::api::multisig::calls::types::cancel_as_multi::CallHash,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .multisig()
                    .cancel_as_multi(threshold, other_signatories, timepoint, call_hash);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
        }
    }
    pub mod ethereum {
        use super::*;
        impl EthereumClient<crate::chain::TestNet> {
            pub fn storage(&self) -> EthereumStorage<crate::chain::TestNet> {
                EthereumStorage {
                    client: self.client.clone(),
                    block: None,
                    _pd: PhantomData,
                }
            }
            pub fn storage_at(&self, block_hash: H256) -> EthereumStorage<crate::chain::TestNet> {
                EthereumStorage {
                    client: self.client.clone(),
                    block: Some(block_hash),
                    _pd: PhantomData,
                }
            }
        }
        impl EthereumStorage<crate::chain::TestNet> {
            pub async fn current_receipts(
                &self,
            ) -> crate::Result<
                Option<
                    ::subxt::ext::subxt_core::alloc::vec::Vec<
                        runtime_types::ethereum::receipt::ReceiptV3,
                    >,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .ethereum()
                    .current_receipts();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn current_block(
                &self,
            ) -> crate::Result<
                Option<
                    runtime_types::ethereum::block::Block<
                        runtime_types::ethereum::transaction::TransactionV2,
                    >,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .ethereum()
                    .current_block();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn pending(
                &self,
            ) -> crate::Result<
                Option<
                    ::subxt::ext::subxt_core::alloc::vec::Vec<(
                        runtime_types::ethereum::transaction::TransactionV2,
                        runtime_types::fp_rpc::TransactionStatus,
                        runtime_types::ethereum::receipt::ReceiptV3,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .ethereum()
                    .pending();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn current_transaction_statuses(
                &self,
            ) -> crate::Result<
                Option<
                    ::subxt::ext::subxt_core::alloc::vec::Vec<
                        runtime_types::fp_rpc::TransactionStatus,
                    >,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .ethereum()
                    .current_transaction_statuses();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn block_hash_get(
                &self,
                key: &runtime_types::primitive_types::U256,
            ) -> crate::Result<Option<::subxt::ext::subxt_core::utils::H256>> {
                let call = crate::interfaces::testnet::api::storage()
                    .ethereum()
                    .block_hash(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn block_hash_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        runtime_types::primitive_types::U256,
                        ::subxt::ext::subxt_core::utils::H256,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .ethereum()
                    .block_hash_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <runtime_types::primitive_types::U256 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
        }
        impl EthereumClient<crate::chain::TestNet> {
            pub fn calls(&self) -> EthereumCalls<crate::chain::TestNet> {
                EthereumCalls {
                    client: self.client.clone(),
                    _pd: PhantomData,
                }
            }
        }
        impl EthereumCalls<crate::chain::TestNet> {
            pub async fn transact(
                &self,
                transaction: crate::interfaces::testnet::api::ethereum::calls::types::transact::Transaction,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .ethereum()
                    .transact(transaction);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn transact_wait(
                &self,
                transaction: crate::interfaces::testnet::api::ethereum::calls::types::transact::Transaction,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .ethereum()
                    .transact(transaction);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
        }
    }
    pub mod evm {
        use super::*;
        impl EvmClient<crate::chain::TestNet> {
            pub fn storage(&self) -> EvmStorage<crate::chain::TestNet> {
                EvmStorage {
                    client: self.client.clone(),
                    block: None,
                    _pd: PhantomData,
                }
            }
            pub fn storage_at(&self, block_hash: H256) -> EvmStorage<crate::chain::TestNet> {
                EvmStorage {
                    client: self.client.clone(),
                    block: Some(block_hash),
                    _pd: PhantomData,
                }
            }
        }
        impl EvmStorage<crate::chain::TestNet> {
            pub async fn account_codes_get(
                &self,
                key: &::subxt::ext::subxt_core::utils::H160,
            ) -> crate::Result<
                Option<::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>>,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .evm()
                    .account_codes(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn account_codes_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::subxt::ext::subxt_core::utils::H160,
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .evm()
                    .account_codes_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::subxt::ext::subxt_core::utils::H160 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn account_codes_metadata_get(
                &self,
                key: &::subxt::ext::subxt_core::utils::H160,
            ) -> crate::Result<Option<runtime_types::pallet_evm::CodeMetadata>> {
                let call = crate::interfaces::testnet::api::storage()
                    .evm()
                    .account_codes_metadata(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn account_codes_metadata_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::subxt::ext::subxt_core::utils::H160,
                        runtime_types::pallet_evm::CodeMetadata,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .evm()
                    .account_codes_metadata_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::subxt::ext::subxt_core::utils::H160 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn suicided_get(
                &self,
                key: &::subxt::ext::subxt_core::utils::H160,
            ) -> crate::Result<Option<()>> {
                let call = crate::interfaces::testnet::api::storage()
                    .evm()
                    .suicided(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn suicided_iter(
                &self,
            ) -> crate::Result<
                impl Stream<Item = crate::Result<(::subxt::ext::subxt_core::utils::H160, ())>>,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .evm()
                    .suicided_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::subxt::ext::subxt_core::utils::H160 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn account_storages_get(
                &self,
                key1: &::subxt::ext::subxt_core::utils::H160,
                key2: &::subxt::ext::subxt_core::utils::H256,
            ) -> crate::Result<Option<::subxt::ext::subxt_core::utils::H256>> {
                let call = crate::interfaces::testnet::api::storage()
                    .evm()
                    .account_storages(key1, key2);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn account_storages_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        (
                            ::subxt::ext::subxt_core::utils::H160,
                            ::subxt::ext::subxt_core::utils::H256,
                        ),
                        ::subxt::ext::subxt_core::utils::H256,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .evm()
                    .account_storages_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let pair = <(
                            ::subxt::ext::subxt_core::utils::H160,
                            ::subxt::ext::subxt_core::utils::H256,
                        ) as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((pair, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn account_storages_iter1(
                &self,
                key1: &::subxt::ext::subxt_core::utils::H160,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        (
                            ::subxt::ext::subxt_core::utils::H160,
                            ::subxt::ext::subxt_core::utils::H256,
                        ),
                        ::subxt::ext::subxt_core::utils::H256,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .evm()
                    .account_storages_iter1(key1);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let pair = <(
                            ::subxt::ext::subxt_core::utils::H160,
                            ::subxt::ext::subxt_core::utils::H256,
                        ) as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((pair, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
        }
        impl EvmClient<crate::chain::TestNet> {
            pub fn calls(&self) -> EvmCalls<crate::chain::TestNet> {
                EvmCalls {
                    client: self.client.clone(),
                    _pd: PhantomData,
                }
            }
        }
        impl EvmCalls<crate::chain::TestNet> {
            pub async fn withdraw(
                &self,
                address: crate::interfaces::testnet::api::evm::calls::types::withdraw::Address,
                value: crate::interfaces::testnet::api::evm::calls::types::withdraw::Value,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .evm()
                    .withdraw(address, value);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn withdraw_wait(
                &self,
                address: crate::interfaces::testnet::api::evm::calls::types::withdraw::Address,
                value: crate::interfaces::testnet::api::evm::calls::types::withdraw::Value,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .evm()
                    .withdraw(address, value);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn call(
                &self,
                source: crate::interfaces::testnet::api::evm::calls::types::call::Source,
                target: crate::interfaces::testnet::api::evm::calls::types::call::Target,
                input: crate::interfaces::testnet::api::evm::calls::types::call::Input,
                value: crate::interfaces::testnet::api::evm::calls::types::call::Value,
                gas_limit: crate::interfaces::testnet::api::evm::calls::types::call::GasLimit,
                max_fee_per_gas: crate::interfaces::testnet::api::evm::calls::types::call::MaxFeePerGas,
                max_priority_fee_per_gas: crate::interfaces::testnet::api::evm::calls::types::call::MaxPriorityFeePerGas,
                nonce: crate::interfaces::testnet::api::evm::calls::types::call::Nonce,
                access_list: crate::interfaces::testnet::api::evm::calls::types::call::AccessList,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx().evm().call(
                    source,
                    target,
                    input,
                    value,
                    gas_limit,
                    max_fee_per_gas,
                    max_priority_fee_per_gas,
                    nonce,
                    access_list,
                );
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn call_wait(
                &self,
                source: crate::interfaces::testnet::api::evm::calls::types::call::Source,
                target: crate::interfaces::testnet::api::evm::calls::types::call::Target,
                input: crate::interfaces::testnet::api::evm::calls::types::call::Input,
                value: crate::interfaces::testnet::api::evm::calls::types::call::Value,
                gas_limit: crate::interfaces::testnet::api::evm::calls::types::call::GasLimit,
                max_fee_per_gas: crate::interfaces::testnet::api::evm::calls::types::call::MaxFeePerGas,
                max_priority_fee_per_gas: crate::interfaces::testnet::api::evm::calls::types::call::MaxPriorityFeePerGas,
                nonce: crate::interfaces::testnet::api::evm::calls::types::call::Nonce,
                access_list: crate::interfaces::testnet::api::evm::calls::types::call::AccessList,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx().evm().call(
                    source,
                    target,
                    input,
                    value,
                    gas_limit,
                    max_fee_per_gas,
                    max_priority_fee_per_gas,
                    nonce,
                    access_list,
                );
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn create(
                &self,
                source: crate::interfaces::testnet::api::evm::calls::types::create::Source,
                init: crate::interfaces::testnet::api::evm::calls::types::create::Init,
                value: crate::interfaces::testnet::api::evm::calls::types::create::Value,
                gas_limit: crate::interfaces::testnet::api::evm::calls::types::create::GasLimit,
                max_fee_per_gas: crate::interfaces::testnet::api::evm::calls::types::create::MaxFeePerGas,
                max_priority_fee_per_gas: crate::interfaces::testnet::api::evm::calls::types::create::MaxPriorityFeePerGas,
                nonce: crate::interfaces::testnet::api::evm::calls::types::create::Nonce,
                access_list: crate::interfaces::testnet::api::evm::calls::types::create::AccessList,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx().evm().create(
                    source,
                    init,
                    value,
                    gas_limit,
                    max_fee_per_gas,
                    max_priority_fee_per_gas,
                    nonce,
                    access_list,
                );
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn create_wait(
                &self,
                source: crate::interfaces::testnet::api::evm::calls::types::create::Source,
                init: crate::interfaces::testnet::api::evm::calls::types::create::Init,
                value: crate::interfaces::testnet::api::evm::calls::types::create::Value,
                gas_limit: crate::interfaces::testnet::api::evm::calls::types::create::GasLimit,
                max_fee_per_gas: crate::interfaces::testnet::api::evm::calls::types::create::MaxFeePerGas,
                max_priority_fee_per_gas: crate::interfaces::testnet::api::evm::calls::types::create::MaxPriorityFeePerGas,
                nonce: crate::interfaces::testnet::api::evm::calls::types::create::Nonce,
                access_list: crate::interfaces::testnet::api::evm::calls::types::create::AccessList,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx().evm().create(
                    source,
                    init,
                    value,
                    gas_limit,
                    max_fee_per_gas,
                    max_priority_fee_per_gas,
                    nonce,
                    access_list,
                );
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn create2(
                &self,
                source: crate::interfaces::testnet::api::evm::calls::types::create2::Source,
                init: crate::interfaces::testnet::api::evm::calls::types::create2::Init,
                salt: crate::interfaces::testnet::api::evm::calls::types::create2::Salt,
                value: crate::interfaces::testnet::api::evm::calls::types::create2::Value,
                gas_limit: crate::interfaces::testnet::api::evm::calls::types::create2::GasLimit,
                max_fee_per_gas: crate::interfaces::testnet::api::evm::calls::types::create2::MaxFeePerGas,
                max_priority_fee_per_gas: crate::interfaces::testnet::api::evm::calls::types::create2::MaxPriorityFeePerGas,
                nonce: crate::interfaces::testnet::api::evm::calls::types::create2::Nonce,
                access_list: crate::interfaces::testnet::api::evm::calls::types::create2::AccessList,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx().evm().create2(
                    source,
                    init,
                    salt,
                    value,
                    gas_limit,
                    max_fee_per_gas,
                    max_priority_fee_per_gas,
                    nonce,
                    access_list,
                );
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn create2_wait(
                &self,
                source: crate::interfaces::testnet::api::evm::calls::types::create2::Source,
                init: crate::interfaces::testnet::api::evm::calls::types::create2::Init,
                salt: crate::interfaces::testnet::api::evm::calls::types::create2::Salt,
                value: crate::interfaces::testnet::api::evm::calls::types::create2::Value,
                gas_limit: crate::interfaces::testnet::api::evm::calls::types::create2::GasLimit,
                max_fee_per_gas: crate::interfaces::testnet::api::evm::calls::types::create2::MaxFeePerGas,
                max_priority_fee_per_gas: crate::interfaces::testnet::api::evm::calls::types::create2::MaxPriorityFeePerGas,
                nonce: crate::interfaces::testnet::api::evm::calls::types::create2::Nonce,
                access_list: crate::interfaces::testnet::api::evm::calls::types::create2::AccessList,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx().evm().create2(
                    source,
                    init,
                    salt,
                    value,
                    gas_limit,
                    max_fee_per_gas,
                    max_priority_fee_per_gas,
                    nonce,
                    access_list,
                );
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
        }
    }
    pub mod governance {
        use super::*;
        impl GovernanceClient<crate::chain::TestNet> {
            pub fn storage(&self) -> GovernanceStorage<crate::chain::TestNet> {
                GovernanceStorage {
                    client: self.client.clone(),
                    block: None,
                    _pd: PhantomData,
                }
            }
            pub fn storage_at(&self, block_hash: H256) -> GovernanceStorage<crate::chain::TestNet> {
                GovernanceStorage {
                    client: self.client.clone(),
                    block: Some(block_hash),
                    _pd: PhantomData,
                }
            }
        }
        impl GovernanceStorage<crate::chain::TestNet> {
            pub async fn agents_frozen(&self) -> crate::Result<Option<::core::primitive::bool>> {
                let call = crate::interfaces::testnet::api::storage()
                    .governance()
                    .agents_frozen();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn proposals_get(
                &self,
                key: &::core::primitive::u64,
            ) -> crate::Result<Option<runtime_types::pallet_governance::proposal::Proposal>>
            {
                let call = crate::interfaces::testnet::api::storage()
                    .governance()
                    .proposals(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn proposals_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::core::primitive::u64,
                        runtime_types::pallet_governance::proposal::Proposal,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .governance()
                    .proposals_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::core::primitive::u64 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn not_delegating_voting_power(
                &self,
            ) -> crate::Result<
                Option<
                    runtime_types::bounded_collections::bounded_btree_set::BoundedBTreeSet<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                    >,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .governance()
                    .not_delegating_voting_power();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn global_governance_config(
                &self,
            ) -> crate::Result<
                Option<runtime_types::pallet_governance::config::GovernanceConfiguration>,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .governance()
                    .global_governance_config();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn unrewarded_proposals_get(
                &self,
                key: &::core::primitive::u64,
            ) -> crate::Result<Option<runtime_types::pallet_governance::proposal::UnrewardedProposal>>
            {
                let call = crate::interfaces::testnet::api::storage()
                    .governance()
                    .unrewarded_proposals(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn unrewarded_proposals_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::core::primitive::u64,
                        runtime_types::pallet_governance::proposal::UnrewardedProposal,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .governance()
                    .unrewarded_proposals_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::core::primitive::u64 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn dao_treasury_address(
                &self,
            ) -> crate::Result<Option<::subxt::ext::subxt_core::utils::AccountId32>> {
                let call = crate::interfaces::testnet::api::storage()
                    .governance()
                    .dao_treasury_address();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn agent_applications_get(
                &self,
                key: &::core::primitive::u32,
            ) -> crate::Result<
                Option<runtime_types::pallet_governance::application::AgentApplication>,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .governance()
                    .agent_applications(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn agent_applications_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::core::primitive::u32,
                        runtime_types::pallet_governance::application::AgentApplication,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .governance()
                    .agent_applications_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::core::primitive::u32 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn treasury_emission_fee(
                &self,
            ) -> crate::Result<Option<runtime_types::sp_arithmetic::per_things::Percent>>
            {
                let call = crate::interfaces::testnet::api::storage()
                    .governance()
                    .treasury_emission_fee();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn namespaces_frozen(
                &self,
            ) -> crate::Result<Option<::core::primitive::bool>> {
                let call = crate::interfaces::testnet::api::storage()
                    .governance()
                    .namespaces_frozen();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn allocators_get(
                &self,
                key: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<Option<()>> {
                let call = crate::interfaces::testnet::api::storage()
                    .governance()
                    .allocators(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn allocators_iter(
                &self,
            ) -> crate::Result<
                impl Stream<Item = crate::Result<(::subxt::ext::subxt_core::utils::AccountId32, ())>>,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .governance()
                    .allocators_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::subxt::ext::subxt_core::utils::AccountId32 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn whitelist_get(
                &self,
                key: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<Option<()>> {
                let call = crate::interfaces::testnet::api::storage()
                    .governance()
                    .whitelist(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn whitelist_iter(
                &self,
            ) -> crate::Result<
                impl Stream<Item = crate::Result<(::subxt::ext::subxt_core::utils::AccountId32, ())>>,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .governance()
                    .whitelist_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::subxt::ext::subxt_core::utils::AccountId32 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
        }
        impl GovernanceClient<crate::chain::TestNet> {
            pub fn calls(&self) -> GovernanceCalls<crate::chain::TestNet> {
                GovernanceCalls {
                    client: self.client.clone(),
                    _pd: PhantomData,
                }
            }
        }
        impl GovernanceCalls<crate::chain::TestNet> {
            pub async fn add_allocator(
                &self,
                key: crate::interfaces::testnet::api::governance::calls::types::add_allocator::Key,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .governance()
                    .add_allocator(key);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn add_allocator_wait(
                &self,
                key: crate::interfaces::testnet::api::governance::calls::types::add_allocator::Key,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .governance()
                    .add_allocator(key);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn remove_allocator(
                &self,
                key: crate::interfaces::testnet::api::governance::calls::types::remove_allocator::Key,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .governance()
                    .remove_allocator(key);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn remove_allocator_wait(
                &self,
                key: crate::interfaces::testnet::api::governance::calls::types::remove_allocator::Key,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .governance()
                    .remove_allocator(key);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn add_to_whitelist(
                &self,
                key: crate::interfaces::testnet::api::governance::calls::types::add_to_whitelist::Key,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .governance()
                    .add_to_whitelist(key);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn add_to_whitelist_wait(
                &self,
                key: crate::interfaces::testnet::api::governance::calls::types::add_to_whitelist::Key,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .governance()
                    .add_to_whitelist(key);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn remove_from_whitelist(
                &self,
                key: crate::interfaces::testnet::api::governance::calls::types::remove_from_whitelist::Key,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .governance()
                    .remove_from_whitelist(key);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn remove_from_whitelist_wait(
                &self,
                key: crate::interfaces::testnet::api::governance::calls::types::remove_from_whitelist::Key,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .governance()
                    .remove_from_whitelist(key);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn accept_application(
                &self,
                application_id: crate::interfaces::testnet::api::governance::calls::types::accept_application::ApplicationId,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .governance()
                    .accept_application(application_id);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn accept_application_wait(
                &self,
                application_id: crate::interfaces::testnet::api::governance::calls::types::accept_application::ApplicationId,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .governance()
                    .accept_application(application_id);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn deny_application(
                &self,
                application_id: crate::interfaces::testnet::api::governance::calls::types::deny_application::ApplicationId,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .governance()
                    .deny_application(application_id);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn deny_application_wait(
                &self,
                application_id: crate::interfaces::testnet::api::governance::calls::types::deny_application::ApplicationId,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .governance()
                    .deny_application(application_id);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn penalize_agent(
                &self,
                agent_key: crate::interfaces::testnet::api::governance::calls::types::penalize_agent::AgentKey,
                percentage: crate::interfaces::testnet::api::governance::calls::types::penalize_agent::Percentage,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .governance()
                    .penalize_agent(agent_key, percentage);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn penalize_agent_wait(
                &self,
                agent_key: crate::interfaces::testnet::api::governance::calls::types::penalize_agent::AgentKey,
                percentage: crate::interfaces::testnet::api::governance::calls::types::penalize_agent::Percentage,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .governance()
                    .penalize_agent(agent_key, percentage);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn submit_application(
                &self,
                agent_key: crate::interfaces::testnet::api::governance::calls::types::submit_application::AgentKey,
                metadata: crate::interfaces::testnet::api::governance::calls::types::submit_application::Metadata,
                removing: crate::interfaces::testnet::api::governance::calls::types::submit_application::Removing,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .governance()
                    .submit_application(agent_key, metadata, removing);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn submit_application_wait(
                &self,
                agent_key: crate::interfaces::testnet::api::governance::calls::types::submit_application::AgentKey,
                metadata: crate::interfaces::testnet::api::governance::calls::types::submit_application::Metadata,
                removing: crate::interfaces::testnet::api::governance::calls::types::submit_application::Removing,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .governance()
                    .submit_application(agent_key, metadata, removing);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn add_global_params_proposal(
                &self,
                data: crate::interfaces::testnet::api::governance::calls::types::add_global_params_proposal::Data,
                metadata: crate::interfaces::testnet::api::governance::calls::types::add_global_params_proposal::Metadata,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .governance()
                    .add_global_params_proposal(data, metadata);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn add_global_params_proposal_wait(
                &self,
                data: crate::interfaces::testnet::api::governance::calls::types::add_global_params_proposal::Data,
                metadata: crate::interfaces::testnet::api::governance::calls::types::add_global_params_proposal::Metadata,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .governance()
                    .add_global_params_proposal(data, metadata);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn add_global_custom_proposal(
                &self,
                metadata: crate::interfaces::testnet::api::governance::calls::types::add_global_custom_proposal::Metadata,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .governance()
                    .add_global_custom_proposal(metadata);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn add_global_custom_proposal_wait(
                &self,
                metadata: crate::interfaces::testnet::api::governance::calls::types::add_global_custom_proposal::Metadata,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .governance()
                    .add_global_custom_proposal(metadata);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn add_dao_treasury_transfer_proposal(
                &self,
                value: crate::interfaces::testnet::api::governance::calls::types::add_dao_treasury_transfer_proposal::Value,
                destination_key: crate::interfaces::testnet::api::governance::calls::types::add_dao_treasury_transfer_proposal::DestinationKey,
                data: crate::interfaces::testnet::api::governance::calls::types::add_dao_treasury_transfer_proposal::Data,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .governance()
                    .add_dao_treasury_transfer_proposal(value, destination_key, data);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn add_dao_treasury_transfer_proposal_wait(
                &self,
                value: crate::interfaces::testnet::api::governance::calls::types::add_dao_treasury_transfer_proposal::Value,
                destination_key: crate::interfaces::testnet::api::governance::calls::types::add_dao_treasury_transfer_proposal::DestinationKey,
                data: crate::interfaces::testnet::api::governance::calls::types::add_dao_treasury_transfer_proposal::Data,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .governance()
                    .add_dao_treasury_transfer_proposal(value, destination_key, data);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn vote_proposal(
                &self,
                proposal_id: crate::interfaces::testnet::api::governance::calls::types::vote_proposal::ProposalId,
                agree: crate::interfaces::testnet::api::governance::calls::types::vote_proposal::Agree,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .governance()
                    .vote_proposal(proposal_id, agree);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn vote_proposal_wait(
                &self,
                proposal_id: crate::interfaces::testnet::api::governance::calls::types::vote_proposal::ProposalId,
                agree: crate::interfaces::testnet::api::governance::calls::types::vote_proposal::Agree,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .governance()
                    .vote_proposal(proposal_id, agree);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn remove_vote_proposal(
                &self,
                proposal_id: crate::interfaces::testnet::api::governance::calls::types::remove_vote_proposal::ProposalId,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .governance()
                    .remove_vote_proposal(proposal_id);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn remove_vote_proposal_wait(
                &self,
                proposal_id: crate::interfaces::testnet::api::governance::calls::types::remove_vote_proposal::ProposalId,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .governance()
                    .remove_vote_proposal(proposal_id);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn enable_vote_delegation(
                &self,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .governance()
                    .enable_vote_delegation();
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn enable_vote_delegation_wait(
                &self,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .governance()
                    .enable_vote_delegation();
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn disable_vote_delegation(
                &self,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .governance()
                    .disable_vote_delegation();
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn disable_vote_delegation_wait(
                &self,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .governance()
                    .disable_vote_delegation();
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn add_emission_proposal(
                &self,
                recycling_percentage: crate::interfaces::testnet::api::governance::calls::types::add_emission_proposal::RecyclingPercentage,
                treasury_percentage: crate::interfaces::testnet::api::governance::calls::types::add_emission_proposal::TreasuryPercentage,
                incentives_ratio: crate::interfaces::testnet::api::governance::calls::types::add_emission_proposal::IncentivesRatio,
                data: crate::interfaces::testnet::api::governance::calls::types::add_emission_proposal::Data,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .governance()
                    .add_emission_proposal(
                        recycling_percentage,
                        treasury_percentage,
                        incentives_ratio,
                        data,
                    );
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn add_emission_proposal_wait(
                &self,
                recycling_percentage: crate::interfaces::testnet::api::governance::calls::types::add_emission_proposal::RecyclingPercentage,
                treasury_percentage: crate::interfaces::testnet::api::governance::calls::types::add_emission_proposal::TreasuryPercentage,
                incentives_ratio: crate::interfaces::testnet::api::governance::calls::types::add_emission_proposal::IncentivesRatio,
                data: crate::interfaces::testnet::api::governance::calls::types::add_emission_proposal::Data,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .governance()
                    .add_emission_proposal(
                        recycling_percentage,
                        treasury_percentage,
                        incentives_ratio,
                        data,
                    );
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn set_emission_params(
                &self,
                recycling_percentage: crate::interfaces::testnet::api::governance::calls::types::set_emission_params::RecyclingPercentage,
                treasury_percentage: crate::interfaces::testnet::api::governance::calls::types::set_emission_params::TreasuryPercentage,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .governance()
                    .set_emission_params(recycling_percentage, treasury_percentage);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn set_emission_params_wait(
                &self,
                recycling_percentage: crate::interfaces::testnet::api::governance::calls::types::set_emission_params::RecyclingPercentage,
                treasury_percentage: crate::interfaces::testnet::api::governance::calls::types::set_emission_params::TreasuryPercentage,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .governance()
                    .set_emission_params(recycling_percentage, treasury_percentage);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn toggle_agent_freezing(
                &self,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .governance()
                    .toggle_agent_freezing();
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn toggle_agent_freezing_wait(
                &self,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .governance()
                    .toggle_agent_freezing();
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn toggle_namespace_freezing(
                &self,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .governance()
                    .toggle_namespace_freezing();
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn toggle_namespace_freezing_wait(
                &self,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .governance()
                    .toggle_namespace_freezing();
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
        }
    }
    pub mod torus0 {
        use super::*;
        impl Torus0Client<crate::chain::TestNet> {
            pub fn storage(&self) -> Torus0Storage<crate::chain::TestNet> {
                Torus0Storage {
                    client: self.client.clone(),
                    block: None,
                    _pd: PhantomData,
                }
            }
            pub fn storage_at(&self, block_hash: H256) -> Torus0Storage<crate::chain::TestNet> {
                Torus0Storage {
                    client: self.client.clone(),
                    block: Some(block_hash),
                    _pd: PhantomData,
                }
            }
        }
        impl Torus0Storage<crate::chain::TestNet> {
            pub async fn min_validator_stake(
                &self,
            ) -> crate::Result<Option<::core::primitive::u128>> {
                let call = crate::interfaces::testnet::api::storage()
                    .torus0()
                    .min_validator_stake();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn burn(&self) -> crate::Result<Option<::core::primitive::u128>> {
                let call = crate::interfaces::testnet::api::storage().torus0().burn();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn max_agent_url_length(
                &self,
            ) -> crate::Result<Option<::core::primitive::u16>> {
                let call = crate::interfaces::testnet::api::storage()
                    .torus0()
                    .max_agent_url_length();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn dividends_participation_weight(
                &self,
            ) -> crate::Result<Option<runtime_types::sp_arithmetic::per_things::Percent>>
            {
                let call = crate::interfaces::testnet::api::storage()
                    .torus0()
                    .dividends_participation_weight();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn agent_update_cooldown(
                &self,
            ) -> crate::Result<Option<::core::primitive::u64>> {
                let call = crate::interfaces::testnet::api::storage()
                    .torus0()
                    .agent_update_cooldown();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn staked_by_get(
                &self,
                key1: &::subxt::ext::subxt_core::utils::AccountId32,
                key2: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<Option<::core::primitive::u128>> {
                let call = crate::interfaces::testnet::api::storage()
                    .torus0()
                    .staked_by(key1, key2);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn staked_by_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        (
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        ),
                        ::core::primitive::u128,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .torus0()
                    .staked_by_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let pair = <(
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        ) as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((pair, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn staked_by_iter1(
                &self,
                key1: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        (
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        ),
                        ::core::primitive::u128,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .torus0()
                    .staked_by_iter1(key1);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let pair = <(
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        ) as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((pair, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn burn_config(
                &self,
            ) -> crate::Result<Option<runtime_types::pallet_torus0::burn::BurnConfiguration>>
            {
                let call = crate::interfaces::testnet::api::storage()
                    .torus0()
                    .burn_config();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn min_allowed_stake(
                &self,
            ) -> crate::Result<Option<::core::primitive::u128>> {
                let call = crate::interfaces::testnet::api::storage()
                    .torus0()
                    .min_allowed_stake();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn min_name_length(&self) -> crate::Result<Option<::core::primitive::u16>> {
                let call = crate::interfaces::testnet::api::storage()
                    .torus0()
                    .min_name_length();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn namespace_count_get(
                &self,
                key: &runtime_types::pallet_torus0::namespace::NamespaceOwnership,
            ) -> crate::Result<Option<::core::primitive::u32>> {
                let call = crate::interfaces::testnet::api::storage()
                    .torus0()
                    .namespace_count(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn namespace_count_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        runtime_types::pallet_torus0::namespace::NamespaceOwnership,
                        ::core::primitive::u32,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .torus0()
                    .namespace_count_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <runtime_types::pallet_torus0::namespace::NamespaceOwnership as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn max_name_length(&self) -> crate::Result<Option<::core::primitive::u16>> {
                let call = crate::interfaces::testnet::api::storage()
                    .torus0()
                    .max_name_length();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn fee_constraints(
                &self,
            ) -> crate::Result<Option<runtime_types::pallet_torus0::fee::ValidatorFeeConstraints>>
            {
                let call = crate::interfaces::testnet::api::storage()
                    .torus0()
                    .fee_constraints();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn namespace_pricing_config(
                &self,
            ) -> crate::Result<
                Option<runtime_types::pallet_torus0::namespace::NamespacePricingConfig>,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .torus0()
                    .namespace_pricing_config();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn staking_to_get(
                &self,
                key1: &::subxt::ext::subxt_core::utils::AccountId32,
                key2: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<Option<::core::primitive::u128>> {
                let call = crate::interfaces::testnet::api::storage()
                    .torus0()
                    .staking_to(key1, key2);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn staking_to_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        (
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        ),
                        ::core::primitive::u128,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .torus0()
                    .staking_to_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let pair = <(
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        ) as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((pair, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn staking_to_iter1(
                &self,
                key1: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        (
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        ),
                        ::core::primitive::u128,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .torus0()
                    .staking_to_iter1(key1);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let pair = <(
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        ) as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((pair, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn max_registrations_per_block(
                &self,
            ) -> crate::Result<Option<::core::primitive::u16>> {
                let call = crate::interfaces::testnet::api::storage()
                    .torus0()
                    .max_registrations_per_block();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn namespaces_get(
                &self,
                key1: &runtime_types::pallet_torus0::namespace::NamespaceOwnership,
                key2: &runtime_types::pallet_torus0_api::NamespacePath,
            ) -> crate::Result<Option<runtime_types::pallet_torus0::namespace::NamespaceMetadata>>
            {
                let call = crate::interfaces::testnet::api::storage()
                    .torus0()
                    .namespaces(key1, key2);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn namespaces_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        (
                            runtime_types::pallet_torus0::namespace::NamespaceOwnership,
                            runtime_types::pallet_torus0_api::NamespacePath,
                        ),
                        runtime_types::pallet_torus0::namespace::NamespaceMetadata,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .torus0()
                    .namespaces_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let pair = <(
                            runtime_types::pallet_torus0::namespace::NamespaceOwnership,
                            runtime_types::pallet_torus0_api::NamespacePath,
                        ) as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((pair, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn namespaces_iter1(
                &self,
                key1: &runtime_types::pallet_torus0::namespace::NamespaceOwnership,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        (
                            runtime_types::pallet_torus0::namespace::NamespaceOwnership,
                            runtime_types::pallet_torus0_api::NamespacePath,
                        ),
                        runtime_types::pallet_torus0::namespace::NamespaceMetadata,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .torus0()
                    .namespaces_iter1(key1);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let pair = <(
                            runtime_types::pallet_torus0::namespace::NamespaceOwnership,
                            runtime_types::pallet_torus0_api::NamespacePath,
                        ) as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((pair, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn agents_get(
                &self,
                key: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<Option<runtime_types::pallet_torus0::agent::Agent>> {
                let call = crate::interfaces::testnet::api::storage()
                    .torus0()
                    .agents(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn agents_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        runtime_types::pallet_torus0::agent::Agent,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .torus0()
                    .agents_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::subxt::ext::subxt_core::utils::AccountId32 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn total_stake(&self) -> crate::Result<Option<::core::primitive::u128>> {
                let call = crate::interfaces::testnet::api::storage()
                    .torus0()
                    .total_stake();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn registrations_this_block(
                &self,
            ) -> crate::Result<Option<::core::primitive::u16>> {
                let call = crate::interfaces::testnet::api::storage()
                    .torus0()
                    .registrations_this_block();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn max_allowed_validators(
                &self,
            ) -> crate::Result<Option<::core::primitive::u16>> {
                let call = crate::interfaces::testnet::api::storage()
                    .torus0()
                    .max_allowed_validators();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn reward_interval(&self) -> crate::Result<Option<::core::primitive::u16>> {
                let call = crate::interfaces::testnet::api::storage()
                    .torus0()
                    .reward_interval();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn registrations_this_interval(
                &self,
            ) -> crate::Result<Option<::core::primitive::u16>> {
                let call = crate::interfaces::testnet::api::storage()
                    .torus0()
                    .registrations_this_interval();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
        }
        impl Torus0Client<crate::chain::TestNet> {
            pub fn calls(&self) -> Torus0Calls<crate::chain::TestNet> {
                Torus0Calls {
                    client: self.client.clone(),
                    _pd: PhantomData,
                }
            }
        }
        impl Torus0Calls<crate::chain::TestNet> {
            pub async fn add_stake(
                &self,
                agent_key: crate::interfaces::testnet::api::torus0::calls::types::add_stake::AgentKey,
                amount: crate::interfaces::testnet::api::torus0::calls::types::add_stake::Amount,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .torus0()
                    .add_stake(agent_key, amount);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn add_stake_wait(
                &self,
                agent_key: crate::interfaces::testnet::api::torus0::calls::types::add_stake::AgentKey,
                amount: crate::interfaces::testnet::api::torus0::calls::types::add_stake::Amount,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .torus0()
                    .add_stake(agent_key, amount);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn remove_stake(
                &self,
                agent_key: crate::interfaces::testnet::api::torus0::calls::types::remove_stake::AgentKey,
                amount: crate::interfaces::testnet::api::torus0::calls::types::remove_stake::Amount,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .torus0()
                    .remove_stake(agent_key, amount);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn remove_stake_wait(
                &self,
                agent_key: crate::interfaces::testnet::api::torus0::calls::types::remove_stake::AgentKey,
                amount: crate::interfaces::testnet::api::torus0::calls::types::remove_stake::Amount,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .torus0()
                    .remove_stake(agent_key, amount);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn transfer_stake(
                &self,
                agent_key: crate::interfaces::testnet::api::torus0::calls::types::transfer_stake::AgentKey,
                new_agent_key: crate::interfaces::testnet::api::torus0::calls::types::transfer_stake::NewAgentKey,
                amount: crate::interfaces::testnet::api::torus0::calls::types::transfer_stake::Amount,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .torus0()
                    .transfer_stake(agent_key, new_agent_key, amount);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn transfer_stake_wait(
                &self,
                agent_key: crate::interfaces::testnet::api::torus0::calls::types::transfer_stake::AgentKey,
                new_agent_key: crate::interfaces::testnet::api::torus0::calls::types::transfer_stake::NewAgentKey,
                amount: crate::interfaces::testnet::api::torus0::calls::types::transfer_stake::Amount,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .torus0()
                    .transfer_stake(agent_key, new_agent_key, amount);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn register_agent(
                &self,
                agent_key: crate::interfaces::testnet::api::torus0::calls::types::register_agent::AgentKey,
                name: crate::interfaces::testnet::api::torus0::calls::types::register_agent::Name,
                url: crate::interfaces::testnet::api::torus0::calls::types::register_agent::Url,
                metadata: crate::interfaces::testnet::api::torus0::calls::types::register_agent::Metadata,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .torus0()
                    .register_agent(agent_key, name, url, metadata);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn register_agent_wait(
                &self,
                agent_key: crate::interfaces::testnet::api::torus0::calls::types::register_agent::AgentKey,
                name: crate::interfaces::testnet::api::torus0::calls::types::register_agent::Name,
                url: crate::interfaces::testnet::api::torus0::calls::types::register_agent::Url,
                metadata: crate::interfaces::testnet::api::torus0::calls::types::register_agent::Metadata,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .torus0()
                    .register_agent(agent_key, name, url, metadata);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn unregister_agent(
                &self,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .torus0()
                    .unregister_agent();
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn unregister_agent_wait(
                &self,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .torus0()
                    .unregister_agent();
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn update_agent(
                &self,
                url: crate::interfaces::testnet::api::torus0::calls::types::update_agent::Url,
                metadata: crate::interfaces::testnet::api::torus0::calls::types::update_agent::Metadata,
                staking_fee: crate::interfaces::testnet::api::torus0::calls::types::update_agent::StakingFee,
                weight_control_fee: crate::interfaces::testnet::api::torus0::calls::types::update_agent::WeightControlFee,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx().torus0().update_agent(
                    url,
                    metadata,
                    staking_fee,
                    weight_control_fee,
                );
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn update_agent_wait(
                &self,
                url: crate::interfaces::testnet::api::torus0::calls::types::update_agent::Url,
                metadata: crate::interfaces::testnet::api::torus0::calls::types::update_agent::Metadata,
                staking_fee: crate::interfaces::testnet::api::torus0::calls::types::update_agent::StakingFee,
                weight_control_fee: crate::interfaces::testnet::api::torus0::calls::types::update_agent::WeightControlFee,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx().torus0().update_agent(
                    url,
                    metadata,
                    staking_fee,
                    weight_control_fee,
                );
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn set_agent_update_cooldown(
                &self,
                new_cooldown: crate::interfaces::testnet::api::torus0::calls::types::set_agent_update_cooldown::NewCooldown,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .torus0()
                    .set_agent_update_cooldown(new_cooldown);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn set_agent_update_cooldown_wait(
                &self,
                new_cooldown: crate::interfaces::testnet::api::torus0::calls::types::set_agent_update_cooldown::NewCooldown,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .torus0()
                    .set_agent_update_cooldown(new_cooldown);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn create_namespace(
                &self,
                path: crate::interfaces::testnet::api::torus0::calls::types::create_namespace::Path,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .torus0()
                    .create_namespace(path);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn create_namespace_wait(
                &self,
                path: crate::interfaces::testnet::api::torus0::calls::types::create_namespace::Path,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .torus0()
                    .create_namespace(path);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn delete_namespace(
                &self,
                path: crate::interfaces::testnet::api::torus0::calls::types::delete_namespace::Path,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .torus0()
                    .delete_namespace(path);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn delete_namespace_wait(
                &self,
                path: crate::interfaces::testnet::api::torus0::calls::types::delete_namespace::Path,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .torus0()
                    .delete_namespace(path);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
        }
    }
    pub mod emission0 {
        use super::*;
        impl Emission0Client<crate::chain::TestNet> {
            pub fn storage(&self) -> Emission0Storage<crate::chain::TestNet> {
                Emission0Storage {
                    client: self.client.clone(),
                    block: None,
                    _pd: PhantomData,
                }
            }
            pub fn storage_at(&self, block_hash: H256) -> Emission0Storage<crate::chain::TestNet> {
                Emission0Storage {
                    client: self.client.clone(),
                    block: Some(block_hash),
                    _pd: PhantomData,
                }
            }
        }
        impl Emission0Storage<crate::chain::TestNet> {
            pub async fn incentives_ratio(
                &self,
            ) -> crate::Result<Option<runtime_types::sp_arithmetic::per_things::Percent>>
            {
                let call = crate::interfaces::testnet::api::storage()
                    .emission0()
                    .incentives_ratio();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn pending_emission(&self) -> crate::Result<Option<::core::primitive::u128>> {
                let call = crate::interfaces::testnet::api::storage()
                    .emission0()
                    .pending_emission();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn weight_control_delegation_get(
                &self,
                key: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<Option<::subxt::ext::subxt_core::utils::AccountId32>> {
                let call = crate::interfaces::testnet::api::storage()
                    .emission0()
                    .weight_control_delegation(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn weight_control_delegation_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        ::subxt::ext::subxt_core::utils::AccountId32,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .emission0()
                    .weight_control_delegation_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::subxt::ext::subxt_core::utils::AccountId32 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn emission_recycling_percentage(
                &self,
            ) -> crate::Result<Option<runtime_types::sp_arithmetic::per_things::Percent>>
            {
                let call = crate::interfaces::testnet::api::storage()
                    .emission0()
                    .emission_recycling_percentage();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn consensus_members_get(
                &self,
                key: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<Option<runtime_types::pallet_emission0::ConsensusMember>>
            {
                let call = crate::interfaces::testnet::api::storage()
                    .emission0()
                    .consensus_members(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn consensus_members_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        runtime_types::pallet_emission0::ConsensusMember,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .emission0()
                    .consensus_members_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::subxt::ext::subxt_core::utils::AccountId32 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
        }
        impl Emission0Client<crate::chain::TestNet> {
            pub fn calls(&self) -> Emission0Calls<crate::chain::TestNet> {
                Emission0Calls {
                    client: self.client.clone(),
                    _pd: PhantomData,
                }
            }
        }
        impl Emission0Calls<crate::chain::TestNet> {
            pub async fn set_weights(
                &self,
                weights: crate::interfaces::testnet::api::emission0::calls::types::set_weights::Weights,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .emission0()
                    .set_weights(weights);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn set_weights_wait(
                &self,
                weights: crate::interfaces::testnet::api::emission0::calls::types::set_weights::Weights,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .emission0()
                    .set_weights(weights);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn delegate_weight_control(
                &self,
                target: crate::interfaces::testnet::api::emission0::calls::types::delegate_weight_control::Target,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .emission0()
                    .delegate_weight_control(target);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn delegate_weight_control_wait(
                &self,
                target: crate::interfaces::testnet::api::emission0::calls::types::delegate_weight_control::Target,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .emission0()
                    .delegate_weight_control(target);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn regain_weight_control(
                &self,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .emission0()
                    .regain_weight_control();
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn regain_weight_control_wait(
                &self,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .emission0()
                    .regain_weight_control();
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
        }
    }
    pub mod permission0 {
        use super::*;
        impl Permission0Client<crate::chain::TestNet> {
            pub fn storage(&self) -> Permission0Storage<crate::chain::TestNet> {
                Permission0Storage {
                    client: self.client.clone(),
                    block: None,
                    _pd: PhantomData,
                }
            }
            pub fn storage_at(
                &self,
                block_hash: H256,
            ) -> Permission0Storage<crate::chain::TestNet> {
                Permission0Storage {
                    client: self.client.clone(),
                    block: Some(block_hash),
                    _pd: PhantomData,
                }
            }
        }
        impl Permission0Storage<crate::chain::TestNet> {
            pub async fn permissions_get(
                &self,
                key: &::subxt::ext::subxt_core::utils::H256,
            ) -> crate::Result<
                Option<runtime_types::pallet_permission0::permission::PermissionContract>,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .permission0()
                    .permissions(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn permissions_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::subxt::ext::subxt_core::utils::H256,
                        runtime_types::pallet_permission0::permission::PermissionContract,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .permission0()
                    .permissions_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::subxt::ext::subxt_core::utils::H256 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn enforcement_tracking_get(
                &self,
                key1: &::subxt::ext::subxt_core::utils::H256,
                key2: &runtime_types::pallet_permission0::permission::EnforcementReferendum,
            ) -> crate::Result<
                Option<
                    runtime_types::bounded_collections::bounded_btree_set::BoundedBTreeSet<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                    >,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .permission0()
                    .enforcement_tracking(key1, key2);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn enforcement_tracking_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        (
                            ::subxt::ext::subxt_core::utils::H256,
                            runtime_types::pallet_permission0::permission::EnforcementReferendum,
                        ),
                        runtime_types::bounded_collections::bounded_btree_set::BoundedBTreeSet<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        >,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .permission0()
                    .enforcement_tracking_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let pair = <(
                            ::subxt::ext::subxt_core::utils::H256,
                            runtime_types::pallet_permission0::permission::EnforcementReferendum,
                        ) as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((pair, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn enforcement_tracking_iter1(
                &self,
                key1: &::subxt::ext::subxt_core::utils::H256,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        (
                            ::subxt::ext::subxt_core::utils::H256,
                            runtime_types::pallet_permission0::permission::EnforcementReferendum,
                        ),
                        runtime_types::bounded_collections::bounded_btree_set::BoundedBTreeSet<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        >,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .permission0()
                    .enforcement_tracking_iter1(key1);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let pair = <(
                            ::subxt::ext::subxt_core::utils::H256,
                            runtime_types::pallet_permission0::permission::EnforcementReferendum,
                        ) as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((pair, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn permissions_by_grantee_get(
                &self,
                key: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<
                Option<
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::subxt::ext::subxt_core::utils::H256,
                    >,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .permission0()
                    .permissions_by_grantee(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn permissions_by_grantee_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::subxt::ext::subxt_core::utils::H256,
                        >,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .permission0()
                    .permissions_by_grantee_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::subxt::ext::subxt_core::utils::AccountId32 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn permissions_by_grantor_get(
                &self,
                key: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<
                Option<
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::subxt::ext::subxt_core::utils::H256,
                    >,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .permission0()
                    .permissions_by_grantor(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn permissions_by_grantor_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::subxt::ext::subxt_core::utils::H256,
                        >,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .permission0()
                    .permissions_by_grantor_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::subxt::ext::subxt_core::utils::AccountId32 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            /// Get storage n-map value by keys
            pub async fn accumulated_stream_amounts_get(
                &self,
                account_id32_1: ::subxt::ext::subxt_core::utils::AccountId32,
                h256_2: ::subxt::ext::subxt_core::utils::H256,
                h256_3: ::subxt::ext::subxt_core::utils::H256,
            ) -> crate::Result<Option<::core::primitive::u128>> {
                let call = crate::interfaces::testnet::api::storage()
                    .permission0()
                    .accumulated_stream_amounts(account_id32_1, h256_2, h256_3);
                let storage = self.client.storage().at_latest().await?;
                Ok(storage.fetch(&call).await?)
            }
            /// Query all entries in storage n-map
            pub async fn accumulated_stream_amounts_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        (
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            ::subxt::ext::subxt_core::utils::H256,
                            ::subxt::ext::subxt_core::utils::H256,
                        ),
                        ::core::primitive::u128,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .permission0()
                    .accumulated_stream_amounts_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let tuple = <(
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            ::subxt::ext::subxt_core::utils::H256,
                            ::subxt::ext::subxt_core::utils::H256,
                        ) as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((tuple, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn accumulated_stream_amounts_iter1(
                &self,
                key0: ::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        ::core::primitive::u128,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .permission0()
                    .accumulated_stream_amounts_iter1(key0);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let tuple =
                            <::subxt::ext::subxt_core::utils::AccountId32 as Decode>::decode(
                                &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                            )?;
                        Ok((tuple, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn accumulated_stream_amounts_iter2(
                &self,
                key0: ::subxt::ext::subxt_core::utils::AccountId32,
                key1: ::subxt::ext::subxt_core::utils::H256,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        (
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            ::subxt::ext::subxt_core::utils::H256,
                        ),
                        ::core::primitive::u128,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .permission0()
                    .accumulated_stream_amounts_iter2(key0, key1);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let tuple = <(
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            ::subxt::ext::subxt_core::utils::H256,
                        ) as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((tuple, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn permissions_by_participants_get(
                &self,
                key1: &::subxt::ext::subxt_core::utils::AccountId32,
                key2: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<
                Option<
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::subxt::ext::subxt_core::utils::H256,
                    >,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .permission0()
                    .permissions_by_participants(key1, key2);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn permissions_by_participants_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        (
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        ),
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::subxt::ext::subxt_core::utils::H256,
                        >,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .permission0()
                    .permissions_by_participants_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let pair = <(
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        ) as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((pair, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn permissions_by_participants_iter1(
                &self,
                key1: &::subxt::ext::subxt_core::utils::AccountId32,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        (
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        ),
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::subxt::ext::subxt_core::utils::H256,
                        >,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .permission0()
                    .permissions_by_participants_iter1(key1);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let pair = <(
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        ) as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((pair, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
            pub async fn revocation_tracking_get(
                &self,
                key: &::subxt::ext::subxt_core::utils::H256,
            ) -> crate::Result<
                Option<
                    runtime_types::bounded_collections::bounded_btree_set::BoundedBTreeSet<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                    >,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .permission0()
                    .revocation_tracking(key);
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                Ok(api.fetch(&call).await?)
            }
            pub async fn revocation_tracking_iter(
                &self,
            ) -> crate::Result<
                impl Stream<
                    Item = crate::Result<(
                        ::subxt::ext::subxt_core::utils::H256,
                        runtime_types::bounded_collections::bounded_btree_set::BoundedBTreeSet<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        >,
                    )>,
                >,
            > {
                let call = crate::interfaces::testnet::api::storage()
                    .permission0()
                    .revocation_tracking_iter();
                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };
                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let key = <::subxt::ext::subxt_core::utils::H256 as Decode>::decode(
                            &mut res.key_bytes.get(32..).unwrap_or(&res.key_bytes[..]),
                        )?;
                        Ok((key, res.value))
                    })
                    .into_stream();
                Ok(stream)
            }
        }
        impl Permission0Client<crate::chain::TestNet> {
            pub fn calls(&self) -> Permission0Calls<crate::chain::TestNet> {
                Permission0Calls {
                    client: self.client.clone(),
                    _pd: PhantomData,
                }
            }
        }
        impl Permission0Calls<crate::chain::TestNet> {
            pub async fn grant_emission_permission(
                &self,
                grantee: crate::interfaces::testnet::api::permission0::calls::types::grant_emission_permission::Grantee,
                allocation: crate::interfaces::testnet::api::permission0::calls::types::grant_emission_permission::Allocation,
                targets: crate::interfaces::testnet::api::permission0::calls::types::grant_emission_permission::Targets,
                distribution: crate::interfaces::testnet::api::permission0::calls::types::grant_emission_permission::Distribution,
                duration: crate::interfaces::testnet::api::permission0::calls::types::grant_emission_permission::Duration,
                revocation: crate::interfaces::testnet::api::permission0::calls::types::grant_emission_permission::Revocation,
                enforcement: crate::interfaces::testnet::api::permission0::calls::types::grant_emission_permission::Enforcement,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .permission0()
                    .grant_emission_permission(
                        grantee,
                        allocation,
                        targets,
                        distribution,
                        duration,
                        revocation,
                        enforcement,
                    );
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn grant_emission_permission_wait(
                &self,
                grantee: crate::interfaces::testnet::api::permission0::calls::types::grant_emission_permission::Grantee,
                allocation: crate::interfaces::testnet::api::permission0::calls::types::grant_emission_permission::Allocation,
                targets: crate::interfaces::testnet::api::permission0::calls::types::grant_emission_permission::Targets,
                distribution: crate::interfaces::testnet::api::permission0::calls::types::grant_emission_permission::Distribution,
                duration: crate::interfaces::testnet::api::permission0::calls::types::grant_emission_permission::Duration,
                revocation: crate::interfaces::testnet::api::permission0::calls::types::grant_emission_permission::Revocation,
                enforcement: crate::interfaces::testnet::api::permission0::calls::types::grant_emission_permission::Enforcement,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .permission0()
                    .grant_emission_permission(
                        grantee,
                        allocation,
                        targets,
                        distribution,
                        duration,
                        revocation,
                        enforcement,
                    );
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn revoke_permission(
                &self,
                permission_id: crate::interfaces::testnet::api::permission0::calls::types::revoke_permission::PermissionId,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .permission0()
                    .revoke_permission(permission_id);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn revoke_permission_wait(
                &self,
                permission_id: crate::interfaces::testnet::api::permission0::calls::types::revoke_permission::PermissionId,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .permission0()
                    .revoke_permission(permission_id);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn execute_permission(
                &self,
                permission_id: crate::interfaces::testnet::api::permission0::calls::types::execute_permission::PermissionId,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .permission0()
                    .execute_permission(permission_id);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn execute_permission_wait(
                &self,
                permission_id: crate::interfaces::testnet::api::permission0::calls::types::execute_permission::PermissionId,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .permission0()
                    .execute_permission(permission_id);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn toggle_permission_accumulation(
                &self,
                permission_id: crate::interfaces::testnet::api::permission0::calls::types::toggle_permission_accumulation::PermissionId,
                accumulating: crate::interfaces::testnet::api::permission0::calls::types::toggle_permission_accumulation::Accumulating,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .permission0()
                    .toggle_permission_accumulation(permission_id, accumulating);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn toggle_permission_accumulation_wait(
                &self,
                permission_id: crate::interfaces::testnet::api::permission0::calls::types::toggle_permission_accumulation::PermissionId,
                accumulating: crate::interfaces::testnet::api::permission0::calls::types::toggle_permission_accumulation::Accumulating,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .permission0()
                    .toggle_permission_accumulation(permission_id, accumulating);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn enforcement_execute_permission(
                &self,
                permission_id: crate::interfaces::testnet::api::permission0::calls::types::enforcement_execute_permission::PermissionId,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .permission0()
                    .enforcement_execute_permission(permission_id);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn enforcement_execute_permission_wait(
                &self,
                permission_id: crate::interfaces::testnet::api::permission0::calls::types::enforcement_execute_permission::PermissionId,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .permission0()
                    .enforcement_execute_permission(permission_id);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn set_enforcement_authority(
                &self,
                permission_id: crate::interfaces::testnet::api::permission0::calls::types::set_enforcement_authority::PermissionId,
                enforcement: crate::interfaces::testnet::api::permission0::calls::types::set_enforcement_authority::Enforcement,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .permission0()
                    .set_enforcement_authority(permission_id, enforcement);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn set_enforcement_authority_wait(
                &self,
                permission_id: crate::interfaces::testnet::api::permission0::calls::types::set_enforcement_authority::PermissionId,
                enforcement: crate::interfaces::testnet::api::permission0::calls::types::set_enforcement_authority::Enforcement,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .permission0()
                    .set_enforcement_authority(permission_id, enforcement);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn grant_curator_permission(
                &self,
                grantee: crate::interfaces::testnet::api::permission0::calls::types::grant_curator_permission::Grantee,
                flags: crate::interfaces::testnet::api::permission0::calls::types::grant_curator_permission::Flags,
                cooldown: crate::interfaces::testnet::api::permission0::calls::types::grant_curator_permission::Cooldown,
                duration: crate::interfaces::testnet::api::permission0::calls::types::grant_curator_permission::Duration,
                revocation: crate::interfaces::testnet::api::permission0::calls::types::grant_curator_permission::Revocation,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .permission0()
                    .grant_curator_permission(grantee, flags, cooldown, duration, revocation);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn grant_curator_permission_wait(
                &self,
                grantee: crate::interfaces::testnet::api::permission0::calls::types::grant_curator_permission::Grantee,
                flags: crate::interfaces::testnet::api::permission0::calls::types::grant_curator_permission::Flags,
                cooldown: crate::interfaces::testnet::api::permission0::calls::types::grant_curator_permission::Cooldown,
                duration: crate::interfaces::testnet::api::permission0::calls::types::grant_curator_permission::Duration,
                revocation: crate::interfaces::testnet::api::permission0::calls::types::grant_curator_permission::Revocation,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .permission0()
                    .grant_curator_permission(grantee, flags, cooldown, duration, revocation);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn grant_namespace_permission(
                &self,
                grantee: crate::interfaces::testnet::api::permission0::calls::types::grant_namespace_permission::Grantee,
                paths: crate::interfaces::testnet::api::permission0::calls::types::grant_namespace_permission::Paths,
                duration: crate::interfaces::testnet::api::permission0::calls::types::grant_namespace_permission::Duration,
                revocation: crate::interfaces::testnet::api::permission0::calls::types::grant_namespace_permission::Revocation,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .permission0()
                    .grant_namespace_permission(grantee, paths, duration, revocation);
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn grant_namespace_permission_wait(
                &self,
                grantee: crate::interfaces::testnet::api::permission0::calls::types::grant_namespace_permission::Grantee,
                paths: crate::interfaces::testnet::api::permission0::calls::types::grant_namespace_permission::Paths,
                duration: crate::interfaces::testnet::api::permission0::calls::types::grant_namespace_permission::Duration,
                revocation: crate::interfaces::testnet::api::permission0::calls::types::grant_namespace_permission::Revocation,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .permission0()
                    .grant_namespace_permission(grantee, paths, duration, revocation);
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
            pub async fn update_emission_permission(
                &self,
                permission_id: crate::interfaces::testnet::api::permission0::calls::types::update_emission_permission::PermissionId,
                new_targets: crate::interfaces::testnet::api::permission0::calls::types::update_emission_permission::NewTargets,
                new_streams: crate::interfaces::testnet::api::permission0::calls::types::update_emission_permission::NewStreams,
                new_distribution_control: crate::interfaces::testnet::api::permission0::calls::types::update_emission_permission::NewDistributionControl,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .permission0()
                    .update_emission_permission(
                        permission_id,
                        new_targets,
                        new_streams,
                        new_distribution_control,
                    );
                Ok(self
                    .client
                    .tx()
                    .sign_and_submit_default(&call, &signer)
                    .await?)
            }
            pub async fn update_emission_permission_wait(
                &self,
                permission_id: crate::interfaces::testnet::api::permission0::calls::types::update_emission_permission::PermissionId,
                new_targets: crate::interfaces::testnet::api::permission0::calls::types::update_emission_permission::NewTargets,
                new_streams: crate::interfaces::testnet::api::permission0::calls::types::update_emission_permission::NewStreams,
                new_distribution_control: crate::interfaces::testnet::api::permission0::calls::types::update_emission_permission::NewDistributionControl,
                signer: impl subxt::tx::signer::Signer<subxt::PolkadotConfig>,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx()
                    .permission0()
                    .update_emission_permission(
                        permission_id,
                        new_targets,
                        new_streams,
                        new_distribution_control,
                    );
                let mut stream = self
                    .client
                    .tx()
                    .sign_and_submit_then_watch_default(&call, &signer)
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message).into());
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message).into());
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message).into());
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()).into())
            }
        }
    }
    pub mod faucet {
        use super::*;
        impl FaucetClient<crate::chain::TestNet> {
            pub fn calls(&self) -> FaucetCalls<crate::chain::TestNet> {
                FaucetCalls {
                    client: self.client.clone(),
                    _pd: PhantomData,
                }
            }
        }
        impl FaucetCalls<crate::chain::TestNet> {
            pub async fn faucet(
                &self,
                block_number: crate::interfaces::testnet::api::faucet::calls::types::faucet::BlockNumber,
                nonce: crate::interfaces::testnet::api::faucet::calls::types::faucet::Nonce,
                work: crate::interfaces::testnet::api::faucet::calls::types::faucet::Work,
                key: crate::interfaces::testnet::api::faucet::calls::types::faucet::Key,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx().faucet().faucet(
                    block_number,
                    nonce,
                    work,
                    key,
                );
                Ok(self.client.tx().create_unsigned(&call)?.submit())
            }
            pub async fn faucet_wait(
                &self,
                block_number: crate::interfaces::testnet::api::faucet::calls::types::faucet::BlockNumber,
                nonce: crate::interfaces::testnet::api::faucet::calls::types::faucet::Nonce,
                work: crate::interfaces::testnet::api::faucet::calls::types::faucet::Work,
                key: crate::interfaces::testnet::api::faucet::calls::types::faucet::Key,
            ) -> crate::Result<<subxt::PolkadotConfig as subxt::Config>::Hash> {
                let call = crate::interfaces::testnet::api::tx().faucet().faucet(
                    block_number,
                    nonce,
                    work,
                    key,
                );
                let stream = self
                    .client
                    .tx()
                    .create_unsigned(&call)?
                    .submit_and_watch()
                    .await?;
                while let Some(res) = stream.next().await {
                    match res? {
                        subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => {
                            return Ok(tx_in_block.extrinsic_hash());
                        }
                        subxt::tx::TxStatus::Error { message } => {
                            return Err(crate::error::CallError::Failed(message));
                        }
                        subxt::tx::TxStatus::Invalid { message } => {
                            return Err(crate::error::CallError::Invalid(message));
                        }
                        subxt::tx::TxStatus::Dropped { message } => {
                            return Err(crate::error::CallError::Dropped(message));
                        }
                        _ => {}
                    }
                }
                Err(crate::error::CallError::Dropped("Unknown".to_string()))
            }
        }
    }
}
