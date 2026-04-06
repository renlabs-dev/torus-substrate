#![cfg(feature = "runtime-benchmarks")]

use polkadot_sdk::{
    frame_benchmarking::v2::*,
    frame_system::RawOrigin,
    sp_consensus_aura::sr25519::AuthorityId as AuraId,
    sp_core::{ed25519, sr25519},
};

use crate::*;

fn aura_authority(seed: u8) -> AuraId {
    sr25519::Public::from_raw([seed; 32]).into()
}

fn grandpa_authority(seed: u8) -> polkadot_sdk::sp_consensus_grandpa::AuthorityId {
    ed25519::Public::from_raw([seed; 32]).into()
}

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn set_authorities()
    where
        <T as pallet_aura::Config>::AuthorityId: From<AuraId>,
    {
        let aura_authorities = vec![aura_authority(1).into()];
        let grandpa_authorities = vec![(grandpa_authority(1), 1)];

        #[extrinsic_call]
        set_authorities(RawOrigin::Root, aura_authorities, grandpa_authorities)
    }
}
