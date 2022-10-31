use crate::{mock::*, Config, Event, types::FeedEventOf};
use frame_support::{assert_noop, assert_ok, BoundedVec, traits::{OnFinalize, OnInitialize}};
use frame_system::RawOrigin;
use primitives::HOURS;
use sp_runtime::traits::BadOrigin;

/// Utility method that simulates block production
/// 
/// Thanks to Substrate official documentation :)
fn run_to_block(n: u64) {
	while System::block_number() < n {
	 if System::block_number() > 1 {
	  OracleModule::on_finalize(System::block_number());
	  System::on_finalize(System::block_number());
	 }
	 System::set_block_number(System::block_number() + 1);
	 System::on_initialize(System::block_number());
	 OracleModule::on_initialize(System::block_number());
	}
   }

/// Utility function to convert Vec to BoundedVec
fn vec_to_bounded_vec<T: Config>(vec: Vec<u8>) -> BoundedVec<u8, T::MaxEventBytes> {
	BoundedVec::try_from(vec).unwrap()
}

#[test]
fn test_basic_works() {
	new_test_ext().execute_with(|| {
		let dummy_value = b"Dummy value for testing".to_vec();

		// Dispatch a signed extrinsic.
		assert_noop!(
			OracleModule::submit_event(RuntimeOrigin::signed(1), vec_to_bounded_vec::<Test>(dummy_value.clone())),
			BadOrigin
		);

		// only the sudo account can submit a new feed event
		assert_ok!(OracleModule::submit_event(RawOrigin::Root.into(), vec_to_bounded_vec::<Test>(dummy_value.clone())));

		// Make sure event is emitted
		assert_eq!(System::events().last().map(|e| e.event.clone()).expect("Event expected. Should not fail"),
			Event::NewFeedEvent {
				value: dummy_value.clone(),
				block_number: 1,
			}.into()
		);

		// Read pallet storage and assert an expected result.
		let feed_event = OracleModule::feed();

		assert_eq!(feed_event.len(), 1);
		assert_eq!(feed_event[0], FeedEventOf::<Test> {
			value: vec_to_bounded_vec::<Test>(dummy_value),
			block_number: 1,
		});
	});
}

#[test]
fn test_max_events_age() {
	new_test_ext().execute_with(|| {
		// Dummy add 100 events into the feed
		for i in 0..100 {
			let dummy_value = format!("Dummy value for testing - {}", i).as_bytes().to_vec();
			assert_ok!(OracleModule::submit_event(RawOrigin::Root.into(), vec_to_bounded_vec::<Test>(dummy_value.clone())));
			// Increment block number, so that the event is not considered stale
			System::set_block_number(i + 1);
		}

		// Make sure the feed has 100 events
		let feed = OracleModule::feed();

		assert_eq!(feed.len(), 100);

		// Increment block number by 1 hour and 51 blocks, to see if events are removed
		run_to_block(51 + HOURS as u64);

		// Check if the first event was discarded
		let new_feed = OracleModule::feed();

		// We have 50 events in the feed, even though we increment by 51 blocks
		// This is because events are filtered out when block is finalized
		assert_eq!(new_feed.len(), 50);

		// Check the order of removed events, they should be in the order of oldest to newest
		for i in 0..50 {
			assert_eq!(new_feed[i], FeedEventOf::<Test> {
				value: vec_to_bounded_vec::<Test>(format!("Dummy value for testing - {}", i + 50).as_bytes().to_vec()),
				block_number: (i + 50) as u64,
			});
		}
	})
}
