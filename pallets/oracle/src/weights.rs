
//! Autogenerated weights for pallet_oracle
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-31, STEPS: `100`, REPEAT: 100, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `MacBook-Air.local`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/node-template
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet-oracle
// --extrinsic
// *
// --execution
// wasm
// --wasm-execution
// compiled
// --output
// ./pallets/oracle/src/weights.rs
// --template
// ./.maintain/frame-weight-template.hbs
// --steps
// 100
// --repeat
// 100

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_oracle.
pub trait WeightInfo {
	fn submit_event() -> Weight;
}

/// Weights for pallet_oracle using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Oracle Feed (r:1 w:1)
	fn submit_event() -> Weight {
		// Minimum execution time:  nanoseconds.
		Weight::from_ref_time(74_000_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Oracle Feed (r:1 w:1)
	fn submit_event() -> Weight {
		// Minimum execution time:  nanoseconds.
		Weight::from_ref_time(74_000_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
