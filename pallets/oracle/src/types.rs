//! Types for Oracle Pallet
use frame_support::pallet_prelude::*;
use codec::{Encode, Decode, MaxEncodedLen};
use scale_info::TypeInfo;

use crate::Config;

/// A feed event type of this Runtime.
pub type FeedEventOf<T> = FeedEvent<<T as frame_system::Config>::BlockNumber, <T as Config>::MaxEventBytes>;

/// A feed event that is stored in the pallet.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct FeedEvent<BlockNumber, MaxEventBytes> 
    where MaxEventBytes: Get<u32> + Clone
{
    /// Value of the event. Arbitrary bytes.
    pub value: BoundedVec<u8, MaxEventBytes>,
    /// Block number at which the event was recorded.
    pub block_number: BlockNumber,
}
