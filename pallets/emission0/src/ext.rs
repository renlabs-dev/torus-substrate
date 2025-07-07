use polkadot_sdk::{frame_support::traits::Currency, frame_system};

pub type BalanceOf<T> =
    <<T as crate::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

pub type NegativeImbalanceOf<T> = <<T as crate::Config>::Currency as Currency<
    <T as frame_system::Config>::AccountId,
>>::NegativeImbalance;
