#![cfg_attr(not(feature = "std"), no_std)]
// ! # Oracle Module
// ! 
// ! This is a dummy oracle module which demonstrates how to write a simple Substrate pallet.
// ! 
// ! ## Overview
// ! 
// ! This module demonstrates:
// ! - How to write a Substrate pallet, including hooks, storage, events, etc.
// ! - How to write unit tests for a pallet
// ! - How to benchmark a pallet
// ! 
// ! ## Interface
// ! 
// ! ### Dispatchable Functions
// ! 
// ! - `submit_event` - Submit a new feed event, only callable by the sudo account
// ! 
// ! ### Storage
// ! 
// ! - `Feed` - A bounded vector of feed events
// ! 
// ! ### Types
// ! 
// ! - `FeedEvent` - A feed event type is simply a struct with a value as arbitrary bytes and a block number tracking when the event was recorded
// ! 
// ! ### Events
// ! 
// ! - `NewFeedEvent` - A new feed event is recorded

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod types;
pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::{*, ValueQuery};
	use frame_system::pallet_prelude::*;
	use frame_support::sp_runtime::Saturating;
	use scale_info::TypeInfo;
	use sp_std::{vec, vec::Vec};
	use crate::types::FeedEventOf;
	use crate::weights::WeightInfo;

	/// Explicit block number type used by runtime.
	type BlockNumberFor<T> = <T as frame_system::Config>::BlockNumber;

	/// Explicit type for feed in BoundedVec.
	type FeedOf<T> = BoundedVec<FeedEventOf<T>, <T as Config>::MaxEventsInFeed>;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Origin that can submit new feed events
		type FeedOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
		/// Maximum number of events that can be stored in a feed.
		#[pallet::constant]
		type MaxEventsInFeed: Get<u32> + TypeInfo + PartialEq + Eq + Clone;
		/// Maximum number of blocks an event can be stored for.
		#[pallet::constant]
		type MaxEventAge: Get<BlockNumberFor<Self>>;
		/// Maximum length of an event value in bytes.
		#[pallet::constant]
		type MaxEventBytes: Get<u32> + TypeInfo + PartialEq + Eq + Clone;
	}

	#[pallet::type_value]
	pub fn DefaultFeed<T: Config>() -> FeedOf<T> {
		FeedOf::<T>::try_from(vec![]).expect("Empty vector is always valid")
	}
	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn feed)]
	/// A simple feed that consists of a list of events.
	pub type Feed<T> = StorageValue<_, FeedOf<T>, ValueQuery, DefaultFeed<T>>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new event was submitted to the feed.
		NewFeedEvent {
			/// The event value.
			value: Vec<u8>,
			/// The block number at which the event was recorded.
			block_number: BlockNumberFor<T>,
		}
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Event is too large.
		EventTooLarge,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Submit a new event to the feed.
		/// 
		/// The dispatch origin for this call must be `FeedOrigin`.
		/// 
		/// - `value`: The event value.
		#[pallet::weight(<T as Config>::WeightInfo::submit_event())]
		pub fn submit_event(origin: OriginFor<T>, value: BoundedVec<u8, T::MaxEventBytes>) -> DispatchResultWithPostInfo {
			T::FeedOrigin::ensure_origin(origin)?;

			// Do actually insert the event.
			Feed::<T>::try_mutate(|feed| -> DispatchResult {
				feed.try_push(FeedEventOf::<T> {
					value: value.clone(),
					block_number: frame_system::Pallet::<T>::block_number(),
				}).map_err(|_| Error::<T>::EventTooLarge)?;

				Ok(())
			})?;

			// Emit an event.
			Self::deposit_event(Event::NewFeedEvent {
				value: value.into_inner(),
				block_number: frame_system::Pallet::<T>::block_number(),
			});

			Ok(().into())
		}
	}

	/// Implement hooks on pallet lifecycle.
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(_n: T::BlockNumber) -> Weight {
			// Anything that needs to be done at the start of the block.
			// We don't do anything here.
			Weight::from_ref_time(0)
		}

		fn on_finalize(n: T::BlockNumber) {
			// Here we essentially filter out every event that is too old.
			// Formula is simple, `n - MaxEventAge` is oldest allowed block number.
			Feed::<T>::mutate(|feed| {
				let oldest_allowed_block = n.saturating_sub(T::MaxEventAge::get());
				feed.retain(|event| event.block_number >= oldest_allowed_block);
			});
		}
	}

}
