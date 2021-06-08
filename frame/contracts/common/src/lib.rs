use bitflags::bitflags;
use codec::{Decode, Encode};
use sp_core::Bytes;
use sp_runtime::{DispatchError};
use sp_std::prelude::*;

#[cfg(feature = "std")]
use serde::{Serialize, Deserialize};

bitflags! {
	/// Flags used by a contract to customize exit behaviour.
	#[derive(Encode, Decode)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "std", serde(rename_all = "camelCase", transparent))]
	pub struct ReturnFlags: u32 {
		/// If this bit is set all changes made by the contract execution are rolled back.
		const REVERT = 0x0000_0001;
	}
}

#[derive(PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct ExecReturnValue {
	/// Flags passed along by `seal_return`. Empty when `seal_return` was never called.
	pub flags: ReturnFlags,
	/// Buffer passed along by `seal_return`. Empty when `seal_return` was never called.
	pub data: Bytes,
}
