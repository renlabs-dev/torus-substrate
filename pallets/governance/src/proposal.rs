use codec::{Decode, Encode, MaxEncodedLen};
use polkadot_sdk::{
    frame_support::{dispatch::DispatchResult, DebugNoBound},
    polkadot_sdk_frame::prelude::OriginFor,
    sp_core::ConstU32,
    sp_runtime::{BoundedBTreeMap, BoundedBTreeSet, BoundedVec},
    sp_std::vec::Vec,
};
use scale_info::TypeInfo;

pub type ProposalId = u64;

#[derive(DebugNoBound, TypeInfo, Decode, Encode, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct Proposal<T: crate::Config> {
    pub id: ProposalId,
    pub proposer: T::AccountId,
    pub expiration_block: u64,
    pub data: ProposalData<T>,
    pub status: ProposalStatus<T>,
    pub metadata: BoundedVec<u8, ConstU32<256>>,
    pub proposal_cost: u64,
    pub creation_block: u64,
}

#[derive(Clone, DebugNoBound, TypeInfo, Decode, Encode, MaxEncodedLen, PartialEq, Eq)]
#[scale_info(skip_type_params(T))]
pub enum ProposalStatus<T: crate::Config> {
    Open {
        votes_for: BoundedBTreeSet<T::AccountId, ConstU32<{ u32::MAX }>>,
        votes_against: BoundedBTreeSet<T::AccountId, ConstU32<{ u32::MAX }>>,
        stake_for: u64,
        stake_against: u64,
    },
    Accepted {
        block: u64,
        stake_for: u64,
        stake_against: u64,
    },
    Refused {
        block: u64,
        stake_for: u64,
        stake_against: u64,
    },
    Expired,
}

#[derive(DebugNoBound, TypeInfo, Decode, Encode, MaxEncodedLen, PartialEq, Eq)]
#[scale_info(skip_type_params(T))]
pub enum ProposalData<T: crate::Config> {
    GlobalCustom,
    TransferDaoTreasury { account: T::AccountId, amount: u64 },
}

#[derive(DebugNoBound, TypeInfo, Decode, Encode, MaxEncodedLen, PartialEq, Eq)]
#[scale_info(skip_type_params(T))]
pub struct UnrewardedProposal<T: crate::Config> {
    pub block: u64,
    pub votes_for: BoundedBTreeMap<T::AccountId, u64, ConstU32<{ u32::MAX }>>,
    pub votes_against: BoundedBTreeMap<T::AccountId, u64, ConstU32<{ u32::MAX }>>,
}

pub fn add_global_custom_proposal<T: crate::Config>(
    _origin: OriginFor<T>,
    _data: Vec<u8>,
) -> DispatchResult {
    todo!()
}

pub fn add_dao_treasury_transfer_proposal<T: crate::Config>(
    _origin: OriginFor<T>,
    _value: u64,
    _destination_key: T::AccountId,
    _data: Vec<u8>,
) -> DispatchResult {
    todo!()
}
