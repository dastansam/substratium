//! Benchmarking setup for pallet-oracle

use super::*;

#[allow(unused)]
use crate::Pallet as Oracle;
use frame_benchmarking::benchmarks;
use frame_system::RawOrigin;
use frame_support::BoundedVec;
use sp_std::vec;

benchmarks! {
	submit_event {
		let raw_value = vec![1u8; 65534];
		let value = BoundedVec::try_from(raw_value).unwrap();
	}: _(RawOrigin::Root, value.clone())
	verify {
		assert_eq!(Oracle::<T>::feed().len(), 1);
		assert_eq!(Oracle::<T>::feed()[0].value, value);
	}

	impl_benchmark_test_suite!(Oracle, crate::mock::new_test_ext(), crate::mock::Test);
}
