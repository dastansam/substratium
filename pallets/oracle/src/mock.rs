use crate as pallet_oracle;
use codec::{Decode, Encode};
use frame_support::{traits::{ConstU16, ConstU64}, parameter_types};
use frame_system as system;
use scale_info::TypeInfo;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};
use system::EnsureRoot;
use primitives::{HOURS, BlockNumber};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		OracleModule: pallet_oracle::{Pallet, Call, Storage, Event<T>},
	}
);

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	/// The maximum age of event in blocks.
	pub const MaxEventAge: BlockNumber = HOURS as BlockNumber;
	/// The maximum number of events that can be stored in a feed.
	/// It is used to bound the storage requirements of the pallet.
	#[derive(Debug, PartialEq, Eq, Clone, Encode, Decode, TypeInfo)]
	pub const MaxEventsInFeed: u32 = HOURS * 2;
	/// The maximum length of an event value in bytes.
	#[derive(Debug, PartialEq, Eq, Clone, TypeInfo, Encode, Decode)]
	pub const MaxEventBytes: u32 = 65536; // 64 KiB
}

impl pallet_oracle::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type FeedOrigin = EnsureRoot<Self::AccountId>;
	type WeightInfo = ();
	type MaxEventAge = MaxEventAge;
	type MaxEventBytes = MaxEventBytes;
	type MaxEventsInFeed = MaxEventsInFeed;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let t = system::GenesisConfig::default().build_storage::<Test>().unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));

	ext
}
