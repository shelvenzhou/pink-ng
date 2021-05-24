use sc_executor::{self, CallInWasm};

use sc_executor::{WasmExecutionMethod, WasmExecutor};
use sp_externalities::{Extensions, Extension, Externalities, ExternalitiesExt};
use sp_runtime_interface::runtime_interface;
use sp_wasm_interface::{Function, HostFunctions};
use sp_storage::{ChildInfo, TrackedStorageKey};

use std::{
    any::{Any, TypeId},
    panic::{AssertUnwindSafe, UnwindSafe},
};

#[runtime_interface(no_tracing)]
trait MyInterface {
    fn say_hello_world(data: &str) {
        println!("Hello world from: {}", data);
    }
}
pub struct TestExternalities {
    pub extensions: Extensions,
}

impl TestExternalities {
    pub fn new() -> Self {
        TestExternalities {
            extensions: Extensions::default()
        }
    }
}

type StorageKey = Vec<u8>;
type StorageValue = Vec<u8>;

impl Externalities for TestExternalities {
    fn set_offchain_storage(&mut self, _key: &[u8], _value: Option<&[u8]>) {
		panic!("`set_offchain_storage`: should not be used in async externalities!")
	}

	fn storage(&self, _key: &[u8]) -> Option<StorageValue> {
		panic!("`storage`: should not be used in async externalities!")
	}

	fn storage_hash(&self, _key: &[u8]) -> Option<Vec<u8>> {
		panic!("`storage_hash`: should not be used in async externalities!")
	}

	fn child_storage(
		&self,
		_child_info: &ChildInfo,
		_key: &[u8],
	) -> Option<StorageValue> {
		panic!("`child_storage`: should not be used in async externalities!")
	}

	fn child_storage_hash(
		&self,
		_child_info: &ChildInfo,
		_key: &[u8],
	) -> Option<Vec<u8>> {
		panic!("`child_storage_hash`: should not be used in async externalities!")
	}

	fn next_storage_key(&self, _key: &[u8]) -> Option<StorageKey> {
		panic!("`next_storage_key`: should not be used in async externalities!")
	}

	fn next_child_storage_key(
		&self,
		_child_info: &ChildInfo,
		_key: &[u8],
	) -> Option<StorageKey> {
		panic!("`next_child_storage_key`: should not be used in async externalities!")
	}

	fn place_storage(&mut self, _key: StorageKey, _maybe_value: Option<StorageValue>) {
		panic!("`place_storage`: should not be used in async externalities!")
	}

	fn place_child_storage(
		&mut self,
		_child_info: &ChildInfo,
		_key: StorageKey,
		_value: Option<StorageValue>,
	) {
		panic!("`place_child_storage`: should not be used in async externalities!")
	}

	fn kill_child_storage(
		&mut self,
		_child_info: &ChildInfo,
		_limit: Option<u32>,
	) -> (bool, u32) {
		panic!("`kill_child_storage`: should not be used in async externalities!")
	}

	fn clear_prefix(&mut self, _prefix: &[u8]) {
		panic!("`clear_prefix`: should not be used in async externalities!")
	}

	fn clear_child_prefix(
		&mut self,
		_child_info: &ChildInfo,
		_prefix: &[u8],
	) {
		panic!("`clear_child_prefix`: should not be used in async externalities!")
	}

	fn storage_append(
		&mut self,
		_key: Vec<u8>,
		_value: Vec<u8>,
	) {
		panic!("`storage_append`: should not be used in async externalities!")
	}

	fn storage_root(&mut self) -> Vec<u8> {
		panic!("`storage_root`: should not be used in async externalities!")
	}

	fn child_storage_root(
		&mut self,
		_child_info: &ChildInfo,
	) -> Vec<u8> {
		panic!("`child_storage_root`: should not be used in async externalities!")
	}

	fn storage_changes_root(&mut self, _parent: &[u8]) -> Result<Option<Vec<u8>>, ()> {
		panic!("`storage_changes_root`: should not be used in async externalities!")
	}

	fn storage_start_transaction(&mut self) {
		unimplemented!("Transactions are not supported by TestExternalities");
	}

	fn storage_rollback_transaction(&mut self) -> Result<(), ()> {
		unimplemented!("Transactions are not supported by TestExternalities");
	}

	fn storage_commit_transaction(&mut self) -> Result<(), ()> {
		unimplemented!("Transactions are not supported by TestExternalities");
	}

	fn wipe(&mut self) {}

	fn commit(&mut self) {}

	fn read_write_count(&self) -> (u32, u32, u32, u32) {
		unimplemented!("read_write_count is not supported in TestExternalities")
	}

	fn reset_read_write_count(&mut self) {
		unimplemented!("reset_read_write_count is not supported in TestExternalities")
	}

	fn get_whitelist(&self) -> Vec<TrackedStorageKey> {
		unimplemented!("get_whitelist is not supported in TestExternalities")
	}

	fn set_whitelist(&mut self, _: Vec<TrackedStorageKey>) {
		unimplemented!("set_whitelist is not supported in TestExternalities")
	}
}


impl sp_externalities::ExtensionStore for TestExternalities {
	fn extension_by_type_id(&mut self, type_id: TypeId) -> Option<&mut dyn Any> {
		self.extensions.get_mut(type_id)
	}

	fn register_extension_with_type_id(
		&mut self,
		type_id: TypeId,
		extension: Box<dyn Extension>,
	) -> Result<(), sp_externalities::Error> {
		self.extensions.register_with_type_id(type_id, extension)
	}

	fn deregister_extension_by_type_id(&mut self, type_id: TypeId) -> Result<(), sp_externalities::Error> {
		if self.extensions.deregister(type_id) {
			Ok(())
		} else {
			Err(sp_externalities::Error::ExtensionIsNotRegistered(type_id))
		}
	}
}

fn main() {
    let executor = WasmExecutor::new(
        WasmExecutionMethod::Interpreted,
        Some(17),
        my_interface::HostFunctions::host_functions(),
        8,
        None,
    );

    let wasm_binary: [u8; 10] = [0; 10];
    let call_data: [u8; 1] = [0];

    executor.call_in_wasm(
        &wasm_binary,
        None,
        "my_function",
        &call_data,
        &mut TestExternalities::new(),
        sp_core::traits::MissingHostFunctions::Allow,
    );
}
