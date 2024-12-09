use polkadot_sdk::frame_support::traits::Currency;

pub(super) type BalanceOf<T> = <<T as crate::Config>::Currency as Currency<
    <T as polkadot_sdk::frame_system::Config>::AccountId,
>>::Balance;
