// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(not(feature = "std"), no_std)]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

extern crate alloc;

use alloc::{vec, vec::Vec};
use pallet_transaction_payment::{FeeDetails, RuntimeDispatchInfo};
use polkadot_sdk::{
    polkadot_sdk_frame::{self as frame, prelude::*, runtime::prelude::*},
    *,
};

pub mod apis;

/// The runtime version.
#[runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("torus-runtime"),
    impl_name: create_runtime_str!("torus-runtime"),
    authoring_version: 1,
    spec_version: 0,
    impl_version: 1,
    apis: apis::RUNTIME_API_VERSIONS,
    transaction_version: 1,
    state_version: 1,
};

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

/// The signed extensions that are added to the runtime.
type SignedExtra = (
    // Checks that the sender is not the zero address.
    frame_system::CheckNonZeroSender<Runtime>,
    // Checks that the runtime version is correct.
    frame_system::CheckSpecVersion<Runtime>,
    // Checks that the transaction version is correct.
    frame_system::CheckTxVersion<Runtime>,
    // Checks that the genesis hash is correct.
    frame_system::CheckGenesis<Runtime>,
    // Checks that the era is valid.
    frame_system::CheckEra<Runtime>,
    // Checks that the nonce is valid.
    frame_system::CheckNonce<Runtime>,
    // Checks that the weight is valid.
    frame_system::CheckWeight<Runtime>,
    // Ensures that the sender has enough funds to pay for the transaction
    // and deducts the fee from the sender's account.
    pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);

// Composes the runtime by adding all the used pallets and deriving necessary types.
#[frame_construct_runtime]
mod runtime {
    /// The main runtime type.
    #[runtime::runtime]
    #[runtime::derive(
        RuntimeCall,
        RuntimeEvent,
        RuntimeError,
        RuntimeOrigin,
        RuntimeFreezeReason,
        RuntimeHoldReason,
        RuntimeSlashReason,
        RuntimeLockId,
        RuntimeTask
    )]
    pub struct Runtime;

    /// Mandatory system pallet that should always be included in a FRAME runtime.
    #[runtime::pallet_index(0)]
    pub type System = frame_system::Pallet<Runtime>;

    /// Provides a way for consensus systems to set and check the onchain time.
    #[runtime::pallet_index(1)]
    pub type Timestamp = pallet_timestamp::Pallet<Runtime>;

    /// Provides the ability to keep track of balances.
    #[runtime::pallet_index(2)]
    pub type Balances = pallet_balances::Pallet<Runtime>;

    /// Provides a way to execute privileged functions.
    #[runtime::pallet_index(3)]
    pub type Sudo = pallet_sudo::Pallet<Runtime>;

    /// Provides the ability to charge for extrinsic execution.
    #[runtime::pallet_index(4)]
    pub type TransactionPayment = pallet_transaction_payment::Pallet<Runtime>;

    /// The Torus pallet.
    #[runtime::pallet_index(5)]
    pub type Torus0 = pallet_torus0::Pallet<Runtime>;
}

parameter_types! {
    pub const Version: RuntimeVersion = VERSION;
}

#[derive_impl(frame_system::config_preludes::SolochainDefaultConfig)]
impl frame_system::Config for Runtime {
    type Block = Block;
    type Version = Version;
    type AccountData = pallet_balances::AccountData<<Runtime as pallet_balances::Config>::Balance>;
}

#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
impl pallet_balances::Config for Runtime {
    type AccountStore = System;
}

#[derive_impl(pallet_sudo::config_preludes::TestDefaultConfig)]
impl pallet_sudo::Config for Runtime {}

#[derive_impl(pallet_timestamp::config_preludes::TestDefaultConfig)]
impl pallet_timestamp::Config for Runtime {}

#[derive_impl(pallet_transaction_payment::config_preludes::TestDefaultConfig)]
impl pallet_transaction_payment::Config for Runtime {
    type OnChargeTransaction = pallet_transaction_payment::FungibleAdapter<Balances, ()>;
    // Setting fee as independent of the weight of the extrinsic for demo purposes
    type WeightToFee = NoFee<<Self as pallet_balances::Config>::Balance>;
    // Setting fee as fixed for any length of the call data for demo purposes
    type LengthToFee = FixedFee<1, <Self as pallet_balances::Config>::Balance>;
}

// impl pallet_torus0::Config for Runtime {}

type Block = frame::runtime::types_common::BlockOf<Runtime, SignedExtra>;
type Header = HeaderFor<Runtime>;

type RuntimeExecutive =
    Executive<Runtime, Block, frame_system::ChainContext<Runtime>, Runtime, AllPalletsWithSystem>;

/// Some re-exports that the node side code needs to know. Some are useful in this context as well.
///
/// Other types should preferably be private.
// TODO: this should be standardized in some way, see:
// https://github.com/paritytech/substrate/issues/10579#issuecomment-1600537558
pub mod interface {
    use super::Runtime;
    use polkadot_sdk::{polkadot_sdk_frame as frame, *};

    pub type Block = super::Block;
    pub use frame::runtime::types_common::OpaqueBlock;
    pub type AccountId = <Runtime as frame_system::Config>::AccountId;
    pub type Nonce = <Runtime as frame_system::Config>::Nonce;
    pub type Hash = <Runtime as frame_system::Config>::Hash;
    pub type Balance = <Runtime as pallet_balances::Config>::Balance;
    pub type MinimumBalance = <Runtime as pallet_balances::Config>::ExistentialDeposit;
}
