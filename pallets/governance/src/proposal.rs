use codec::{Decode, Encode, MaxEncodedLen};
use polkadot_sdk::{
    frame_support::{dispatch::DispatchResult, DebugNoBound},
    polkadot_sdk_frame::prelude::OriginFor,
    sp_core::ConstU32,
    sp_runtime::{BoundedBTreeMap, BoundedBTreeSet, BoundedVec},
    sp_std::vec::Vec,
};
use scale_info::TypeInfo;

use crate::{AccountIdOf, BalanceOf, Block};

pub type ProposalId = u64;

#[derive(DebugNoBound, TypeInfo, Decode, Encode, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct Proposal<T: crate::Config> {
    pub id: ProposalId,
    pub proposer: AccountIdOf<T>,
    pub expiration_block: Block,
    pub data: ProposalData<T>,
    pub status: ProposalStatus<T>,
    pub metadata: BoundedVec<u8, ConstU32<256>>,
    pub proposal_cost: BalanceOf<T>,
    pub creation_block: Block,
}

#[derive(Clone, DebugNoBound, TypeInfo, Decode, Encode, MaxEncodedLen, PartialEq, Eq)]
#[scale_info(skip_type_params(T))]
pub enum ProposalStatus<T: crate::Config> {
    Open {
        votes_for: BoundedBTreeSet<AccountIdOf<T>, ConstU32<{ u32::MAX }>>,
        votes_against: BoundedBTreeSet<AccountIdOf<T>, ConstU32<{ u32::MAX }>>,
        stake_for: BalanceOf<T>,
        stake_against: BalanceOf<T>,
    },
    Accepted {
        block: Block,
        stake_for: BalanceOf<T>,
        stake_against: BalanceOf<T>,
    },
    Refused {
        block: Block,
        stake_for: BalanceOf<T>,
        stake_against: BalanceOf<T>,
    },
    Expired,
}

#[derive(DebugNoBound, TypeInfo, Decode, Encode, MaxEncodedLen, PartialEq, Eq)]
#[scale_info(skip_type_params(T))]
pub enum ProposalData<T: crate::Config> {
    GlobalCustom,
    TransferDaoTreasury {
        account: AccountIdOf<T>,
        amount: BalanceOf<T>,
    },
}

#[derive(DebugNoBound, TypeInfo, Decode, Encode, MaxEncodedLen, PartialEq, Eq)]
#[scale_info(skip_type_params(T))]
pub struct UnrewardedProposal<T: crate::Config> {
    pub block: Block,
    pub votes_for: BoundedBTreeMap<AccountIdOf<T>, u64, ConstU32<{ u32::MAX }>>,
    pub votes_against: BoundedBTreeMap<AccountIdOf<T>, u64, ConstU32<{ u32::MAX }>>,
}

pub fn add_global_custom_proposal<T: crate::Config>(
    _origin: OriginFor<T>,
    _data: Vec<u8>,
) -> DispatchResult {
    todo!()
}

pub fn add_dao_treasury_transfer_proposal<T: crate::Config>(
    _origin: OriginFor<T>,
    _value: BalanceOf<T>,
    _destination_key: AccountIdOf<T>,
    _data: Vec<u8>,
) -> DispatchResult {
    todo!()
}
