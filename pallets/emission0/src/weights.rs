
//! Autogenerated weights for `pallet_emission0`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 43.0.0
//! DATE: 2025-01-02, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Luizs-MacBook-Pro.local`, CPU: `<UNKNOWN>`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// ./target/release/torus-node
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet_emission0
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// pallets/emission0/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use polkadot_sdk::{
    frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}},
    *,
};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_emission0`.
pub trait WeightInfo {
	fn set_weights() -> Weight;
	fn delegate_weight_control() -> Weight;
	fn regain_weight_control() -> Weight;
}

/// Weights for `pallet_emission0` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Emission0::MaxAllowedWeights` (r:1 w:0)
	/// Proof: `Emission0::MaxAllowedWeights` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `Torus0::Agents` (r:2 w:0)
	/// Proof: `Torus0::Agents` (`max_values`: None, `max_size`: Some(849), added: 3324, mode: `MaxEncodedLen`)
	/// Storage: `Torus0::StakedBy` (r:1 w:0)
	/// Proof: `Torus0::StakedBy` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `Emission0::MinStakePerWeight` (r:1 w:0)
	/// Proof: `Emission0::MinStakePerWeight` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Emission0::ConsensusMembers` (r:1 w:1)
	/// Proof: `Emission0::ConsensusMembers` (`max_values`: None, `max_size`: Some(4294967295), added: 2474, mode: `MaxEncodedLen`)
	fn set_weights() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `277`
		//  Estimated: `7638`
		// Minimum execution time: 19_000_000 picoseconds.
		Weight::from_parts(20_000_000, 7638)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Torus0::Agents` (r:2 w:0)
	/// Proof: `Torus0::Agents` (`max_values`: None, `max_size`: Some(849), added: 3324, mode: `MaxEncodedLen`)
	/// Storage: `Emission0::WeightControlDelegation` (r:0 w:1)
	/// Proof: `Emission0::WeightControlDelegation` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
	fn delegate_weight_control() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `168`
		//  Estimated: `7638`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(13_000_000, 7638)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Emission0::WeightControlDelegation` (r:1 w:1)
	/// Proof: `Emission0::WeightControlDelegation` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
	fn regain_weight_control() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `199`
		//  Estimated: `3529`
		// Minimum execution time: 7_000_000 picoseconds.
		Weight::from_parts(8_000_000, 3529)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `Emission0::MaxAllowedWeights` (r:1 w:0)
	/// Proof: `Emission0::MaxAllowedWeights` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `Torus0::Agents` (r:2 w:0)
	/// Proof: `Torus0::Agents` (`max_values`: None, `max_size`: Some(849), added: 3324, mode: `MaxEncodedLen`)
	/// Storage: `Torus0::StakedBy` (r:1 w:0)
	/// Proof: `Torus0::StakedBy` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `Emission0::MinStakePerWeight` (r:1 w:0)
	/// Proof: `Emission0::MinStakePerWeight` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Emission0::ConsensusMembers` (r:1 w:1)
	/// Proof: `Emission0::ConsensusMembers` (`max_values`: None, `max_size`: Some(4294967295), added: 2474, mode: `MaxEncodedLen`)
	fn set_weights() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `277`
		//  Estimated: `7638`
		// Minimum execution time: 19_000_000 picoseconds.
		Weight::from_parts(20_000_000, 7638)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Torus0::Agents` (r:2 w:0)
	/// Proof: `Torus0::Agents` (`max_values`: None, `max_size`: Some(849), added: 3324, mode: `MaxEncodedLen`)
	/// Storage: `Emission0::WeightControlDelegation` (r:0 w:1)
	/// Proof: `Emission0::WeightControlDelegation` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
	fn delegate_weight_control() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `168`
		//  Estimated: `7638`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(13_000_000, 7638)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Emission0::WeightControlDelegation` (r:1 w:1)
	/// Proof: `Emission0::WeightControlDelegation` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
	fn regain_weight_control() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `199`
		//  Estimated: `3529`
		// Minimum execution time: 7_000_000 picoseconds.
		Weight::from_parts(8_000_000, 3529)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
