mod runtime_string;
pub use crate::runtime_string::*;
use codec::{Encode, Decode};

#[cfg(feature = "std")]
#[doc(hidden)]
pub use serde::{Serialize, Deserialize};

/// Description of what went wrong when trying to complete an operation on a token.
#[derive(Eq, PartialEq, Clone, Copy, Encode, Decode, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum TokenError {
	/// Funds are unavailable.
	NoFunds,
	/// Account that must exist would die.
	WouldDie,
	/// Account cannot exist with the funds that would be given.
	BelowMinimum,
	/// Account cannot be created.
	CannotCreate,
	/// The asset in question is unknown.
	UnknownAsset,
	/// Funds exist but are frozen.
	Frozen,
}

/// Arithmetic errors.
#[derive(Eq, PartialEq, Clone, Copy, Encode, Decode, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum ArithmeticError {
	/// Underflow.
	Underflow,
	/// Overflow.
	Overflow,
	/// Division by zero.
	DivisionByZero,
}

/// Reason why a dispatch call failed.
#[derive(Eq, Clone, Copy, Encode, Decode, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum DispatchError {
	/// Some error occurred.
	Other(#[codec(skip)] #[cfg_attr(feature = "std", serde(skip_deserializing))] &'static str),
	/// Failed to lookup some data.
	CannotLookup,
	/// A bad origin.
	BadOrigin,
	/// A custom error in a module.
	Module {
		/// Module index, matching the metadata module index.
		index: u8,
		/// Module specific error value.
		error: u8,
		/// Optional error message.
		#[codec(skip)]
		#[cfg_attr(feature = "std", serde(skip_deserializing))]
		message: Option<&'static str>,
	},
	/// At least one consumer is remaining so the account cannot be destroyed.
	ConsumerRemaining,
	/// There are no providers so the account cannot be created.
	NoProviders,
	/// An error to do with tokens.
	Token(TokenError),
	/// An arithmetic error.
	Arithmetic(ArithmeticError),
}

impl From<&'static str> for DispatchError {
	fn from(err: &'static str) -> DispatchError {
		Self::Other(err)
	}
}

impl PartialEq for DispatchError {
	fn eq(&self, other: &Self) -> bool {
		use DispatchError::*;

		match (self, other) {
			(CannotLookup, CannotLookup) |
			(BadOrigin, BadOrigin) |
			(ConsumerRemaining, ConsumerRemaining) |
			(NoProviders, NoProviders) => true,

			(Token(l), Token(r)) => l == r,
			(Other(l), Other(r)) => l == r,
			(Arithmetic(l), Arithmetic(r)) => l == r,

			(
				Module { index: index_l, error: error_l, .. },
				Module { index: index_r, error: error_r, .. },
			) => (index_l == index_r) && (error_l == error_r),

			_ => false,
		}
	}
}
