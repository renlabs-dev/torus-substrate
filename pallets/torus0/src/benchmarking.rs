#![cfg(feature = "runtime-benchmarks")]

use pallet_governance_api::GovernanceApi;
use pallet_torus0_api::Torus0Api;
use polkadot_sdk::{
    frame_benchmarking::{account, v2::*},
    frame_system::RawOrigin,
    sp_runtime::Percent,
};

use crate::*;

fn register_test_agent<T: Config>(id: &T::AccountId, name: &str) {
    let name = name.as_bytes().to_vec();
    Pallet::<T>::force_register_agent(id, name.clone(), name.clone(), name.clone())
        .expect("failed to register agent");
}

#[benchmarks]
mod benchmarks {

    use super::*;

    #[benchmark]
    fn add_stake() {
        let agent: T::AccountId = account("agent", 0, 1);
        let staker: T::AccountId = account("staker", 1, 1);
        let amount = MinAllowedStake::<T>::get();

        register_test_agent::<T>(&agent, "agent");

        let _ = <T::Currency>::deposit_creating(&staker, amount.saturating_mul(2));

        #[extrinsic_call]
        add_stake(RawOrigin::Signed(staker), agent, amount)
    }

    #[benchmark]
    fn remove_stake() {
        let agent: T::AccountId = account("agent", 0, 1);
        let staker: T::AccountId = account("staker", 1, 1);

        register_test_agent::<T>(&agent, "agent");

        let amount = MinAllowedStake::<T>::get();
        let _ = <T::Currency>::deposit_creating(&staker, amount.saturating_mul(2));
        Pallet::<T>::force_set_stake(&staker, &agent, amount).expect("failed to add stake");

        #[extrinsic_call]
        remove_stake(RawOrigin::Signed(staker), agent, amount)
    }

    #[benchmark]
    fn transfer_stake() {
        let agent_a: T::AccountId = account("agent-a", 0, 1);
        let agent_b: T::AccountId = account("agent-b", 1, 1);
        let staker: T::AccountId = account("staker", 2, 1);

        register_test_agent::<T>(&agent_a, "agent-a");
        register_test_agent::<T>(&agent_b, "agent-b");

        let amount = MinAllowedStake::<T>::get();
        let _ = <T::Currency>::deposit_creating(&staker, amount.saturating_mul(2));
        Pallet::<T>::force_set_stake(&staker, &agent_a, amount).expect("failed to add stake");

        #[extrinsic_call]
        transfer_stake(RawOrigin::Signed(staker), agent_a, agent_b, amount)
    }

    #[benchmark]
    fn register_agent() {
        let agent: T::AccountId = account("agent", 0, 1);
        <T::Governance>::force_set_whitelisted(&agent);

        let burn = crate::Burn::<T>::get();
        let _ = <T::Currency>::deposit_creating(&agent, burn.saturating_mul(2));

        let name = b"agent".to_vec();
        let url = b"agent".to_vec();
        let metadata = b"agent".to_vec();

        #[extrinsic_call]
        register_agent(RawOrigin::Signed(agent.clone()), name, url, metadata)
    }

    #[benchmark]
    fn deregister_agent() {
        let agent: T::AccountId = account("agent", 0, 1);
        register_test_agent::<T>(&agent, "agent");

        #[extrinsic_call]
        deregister_agent(RawOrigin::Signed(agent))
    }

    #[benchmark]
    fn update_agent() {
        let agent: T::AccountId = account("agent", 0, 1);
        register_test_agent::<T>(&agent, "agent");

        AgentUpdateCooldown::<T>::set(Default::default());

        let url = b"agent".to_vec();
        let metadata = Some(b"agent".to_vec());
        let staking_fee = Some(Percent::from_percent(10));
        let weight_control_fee = Some(Percent::from_percent(10));

        #[extrinsic_call]
        update_agent(
            RawOrigin::Signed(agent),
            url,
            metadata,
            staking_fee,
            weight_control_fee,
        )
    }

    #[benchmark]
    fn set_agent_update_cooldown() {
        let new_cooldown = 100u32.into();

        #[extrinsic_call]
        set_agent_update_cooldown(RawOrigin::Root, new_cooldown)
    }
}
