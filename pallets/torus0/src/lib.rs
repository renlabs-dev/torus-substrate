#![cfg_attr(not(feature = "std"), no_std)]

use polkadot_sdk::polkadot_sdk_frame as frame;

pub use pallet::*;

#[frame::pallet]
pub mod pallet {
    use super::*;

    #[pallet::config]
    pub trait Config: polkadot_sdk::frame_system::Config {}

    #[pallet::pallet]
    pub struct Pallet<T>(_);
}
