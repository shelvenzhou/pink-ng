#[doc(hidden)]
pub use sp_version::{RuntimeVersion, ApisVec};
#[doc(hidden)]
pub use codec::{Encode, Decode, DecodeLimit, self};
#[doc(hidden)]
pub use sp_runtime::RuntimeString;

#[derive(codec::Encode, codec::Decode)]
pub struct OldRuntimeVersion {
	pub spec_name: RuntimeString,
	pub impl_name: RuntimeString,
	pub authoring_version: u32,
	pub spec_version: u32,
	pub impl_version: u32,
	pub apis: ApisVec,
}

impl From<OldRuntimeVersion> for RuntimeVersion {
	fn from(x: OldRuntimeVersion) -> Self {
		Self {
			spec_name: x.spec_name,
			impl_name: x.impl_name,
			authoring_version: x.authoring_version,
			spec_version: x.spec_version,
			impl_version: x.impl_version,
			apis: x.apis,
			transaction_version: 1,
		}
	}
}

impl From<RuntimeVersion> for OldRuntimeVersion {
	fn from(x: RuntimeVersion) -> Self {
		Self {
			spec_name: x.spec_name,
			impl_name: x.impl_name,
			authoring_version: x.authoring_version,
			spec_version: x.spec_version,
			impl_version: x.impl_version,
			apis: x.apis,
		}
	}
}
