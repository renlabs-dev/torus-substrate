use crate::{AccountIdOf, BalanceOf};
use polkadot_sdk::frame_support::traits::{Currency, ExistenceRequirement};
use polkadot_sdk::frame_support::{dispatch::DispatchResult, ensure};

pub fn transfer_balance<T: crate::Config>(
    key: AccountIdOf<T>,
    destination: AccountIdOf<T>,
    amount: BalanceOf<T>,
) -> DispatchResult {
    ensure!(amount > 0, crate::Error::<T>::InvalidAmount);

    <T as crate::Config>::Currency::transfer(
        &key,
        &destination,
        amount,
        ExistenceRequirement::AllowDeath,
    )
    .map_err(|_| crate::Error::<T>::NotEnoughBalanceToTransfer)?;

    Ok(())
}
