use polkadot_sdk::{
    frame_support::{
        migrations::VersionedMigration, traits::UncheckedOnRuntimeUpgrade, weights::Weight,
    },
    sp_runtime::Percent,
};

use crate::{Config, Pallet};

pub mod v1 {
    use super::*;

    use crate::{Agent, Agents};

    pub type Migration<T, W> = VersionedMigration<0, 1, MigrateToV1<T>, Pallet<T>, W>;
    pub struct MigrateToV1<T>(core::marker::PhantomData<T>);

    impl<T: Config> UncheckedOnRuntimeUpgrade for MigrateToV1<T> {
        fn on_runtime_upgrade() -> Weight {
            Agents::<T>::translate(|_key, mut agent: Agent<T>| {
                agent.weight_penalty_factor = Percent::from_percent(0);
                Some(agent)
            });
            Weight::zero()
        }
    }
}

pub mod v2 {
    use super::*;
    use crate::{Agent, Agents};
    pub type Migration<T, W> = VersionedMigration<1, 2, MigrateToV1<T>, Pallet<T>, W>;
    pub struct MigrateToV1<T>(core::marker::PhantomData<T>);
    impl<T: Config> UncheckedOnRuntimeUpgrade for MigrateToV1<T> {
        fn on_runtime_upgrade() -> Weight {
            Agents::<T>::translate(|_key, mut agent: Agent<T>| {
                agent.fees = Default::default();
                Some(agent)
            });
            Weight::zero()
        }
    }
}

pub mod v3 {
    use super::*;
    use polkadot_sdk::{
        frame_support::traits::Currency,
        sp_tracing::{error, info},
    };

    pub type Migration<T, W> = VersionedMigration<2, 3, MigrateToV3<T>, Pallet<T>, W>;
    pub struct MigrateToV3<T>(core::marker::PhantomData<T>);

    impl<T: Config> UncheckedOnRuntimeUpgrade for MigrateToV3<T> {
        fn on_runtime_upgrade() -> Weight {
            refund_imbalance::<T>();

            Weight::default()
        }
    }

    /// A bug on the staking code cleared the stake maps before
    /// refunding the stake. Because the total stake was not updated in time,
    /// the imbalance refers exactly to the amount that needs to be refunded.
    ///
    /// This way we safely avoid minting new tokens.
    fn refund_imbalance<T: Config>() {
        fn ss58_to_account_id<T: Config>(ss58_address: &str) -> Option<T::AccountId> {
            use codec::{Decode, Encode};
            use polkadot_sdk::{sp_application_crypto::Ss58Codec, sp_runtime::AccountId32};

            let account_id = AccountId32::from_ss58check(ss58_address).ok()?;
            T::AccountId::decode(&mut account_id.encode().as_slice()).ok()
        }

        let account = "5Ef9VXKCmhXCWGdmqm6bm3eVf6WWckd4P1NZTeZLCTJBoduL";
        let Some(account) = ss58_to_account_id::<T>(account) else {
            error!("invalid account {account}");
            return;
        };

        let total_stake = crate::TotalStake::<T>::get();

        let mut sum_staking_to = 0u128;
        for (_, _, stake) in crate::StakingTo::<T>::iter() {
            sum_staking_to = sum_staking_to.saturating_add(stake);
        }

        let mut sum_staked_by = 0u128;
        for (_, _, stake) in crate::StakedBy::<T>::iter() {
            sum_staked_by = sum_staked_by.saturating_add(stake);
        }

        if sum_staking_to != sum_staked_by {
            error!("imbalance between StakingTo ({sum_staking_to}) and StakedBy ({sum_staked_by})");
            return;
        }

        let imbalance = total_stake.saturating_sub(sum_staking_to);
        if imbalance == 0 {
            error!("no imbalance between TotalStake and stake maps");
            return;
        }

        info!("imbalance between TotalStake - sum(stake) = {imbalance}");

        let _ = T::Currency::deposit_creating(&account, imbalance);
        crate::TotalStake::<T>::mutate(|total_stake| {
            *total_stake = total_stake.saturating_sub(imbalance);
        });

        let new_total_stake = crate::TotalStake::<T>::get();
        if new_total_stake != sum_staking_to {
            error!("imbalance remained, {new_total_stake} != {sum_staking_to}");
            return;
        };

        info!("imbalance refunded to {account:?}, {new_total_stake} == {sum_staking_to}");
    }
}
