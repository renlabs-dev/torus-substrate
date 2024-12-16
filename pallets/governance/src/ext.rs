use polkadot_sdk::frame_support::{
    dispatch::DispatchResult,
    traits::{Currency, ExistenceRequirement, WithdrawReasons},
};

pub(super) type BalanceOf<T> = <<T as crate::Config>::Currency as Currency<
    <T as polkadot_sdk::frame_system::Config>::AccountId,
>>::Balance;

pub(super) type AccountIdOf<T> = <T as polkadot_sdk::frame_system::Config>::AccountId;

pub(super) type Block = u64;

pub(super) type BlockAmount = u64;

pub(super) fn get_balance<T: crate::Config>(key: &AccountIdOf<T>) -> BalanceOf<T> {
    <T as crate::Config>::Currency::free_balance(key)
}

pub(super) fn remove_balance<T: crate::Config>(
    key: &AccountIdOf<T>,
    amount: BalanceOf<T>,
) -> DispatchResult {
    let _ = <T as crate::Config>::Currency::withdraw(
        key,
        amount,
        WithdrawReasons::except(WithdrawReasons::TIP),
        ExistenceRequirement::KeepAlive,
    )
    .map_err(|_| crate::Error::<T>::NotEnoughBalanceToPropose)?; // TODO: change error
    Ok(())
}

pub(super) fn add_balance<T: crate::Config>(key: &AccountIdOf<T>, amount: BalanceOf<T>) {
    let _ = <T as crate::Config>::Currency::deposit_creating(&key, amount);
}

pub(super) fn transfer_balance<T: crate::Config>(
    from: &AccountIdOf<T>,
    to: &AccountIdOf<T>,
    amount: BalanceOf<T>,
) -> DispatchResult {
    // TODO: change error
    <T as crate::Config>::Currency::transfer(from, to, amount, ExistenceRequirement::KeepAlive)
        .map_err(|_| crate::Error::<T>::InternalError)?; // TODO: change error

    Ok(())
}
