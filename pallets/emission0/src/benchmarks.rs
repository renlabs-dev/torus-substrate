use pallet_torus0_api::Torus0Api;
use polkadot_sdk::{
    frame_benchmarking::{account, benchmarks},
    frame_system::{self, RawOrigin},
    sp_std::vec,
};

use crate::*;

benchmarks! {
    set_weights {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);
        let module_key2: T::AccountId = account("ModuleKey2", 0, 3);

        <T::Torus>::force_register_agent(&module_key, vec![], vec![], vec![])?;
        <T::Torus>::force_register_agent(&module_key2, vec![], vec![], vec![])?;

        let _ = <T::Currency>::deposit_creating(&module_key2, <T::Torus>::min_validator_stake());

        let weights = vec![(module_key, 10)];
    }: _(RawOrigin::Signed(module_key2), weights)

    delegate_weight_control {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);
        let module_key2: T::AccountId = account("ModuleKey2", 0, 3);

        <T::Torus>::force_register_agent(&module_key, vec![], vec![], vec![])?;
        <T::Torus>::force_register_agent(&module_key2, vec![], vec![], vec![])?;

    }: _(RawOrigin::Signed(module_key), module_key2.clone())

    regain_weight_control {
        let module_key: T::AccountId = account("ModuleKey", 0, 2);
        let module_key2: T::AccountId = account("ModuleKey2", 0, 3);

        <T::Torus>::force_register_agent(&module_key, vec![], vec![], vec![])?;
        <T::Torus>::force_register_agent(&module_key2, vec![], vec![], vec![])?;

        Pallet::<T>::delegate_weight_control(RawOrigin::Signed(module_key.clone()).into(), module_key2.clone())?;

    }: _(RawOrigin::Signed(module_key))
}
