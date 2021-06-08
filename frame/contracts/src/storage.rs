use std::collections::btree_map::BTreeMap;
use crate::exec::{StorageKey, ContractKey};
use std::vec::Vec;
use std::sync::Mutex;

pub type DispatchResult = Result<(), sp_runtime::DispatchError>;

lazy_static! {
    pub static ref STORAGE: Mutex<MockStorage> = Mutex::new(MockStorage::new());
}

pub struct MockStorage {
    map: BTreeMap<(ContractKey, StorageKey), Option<Vec<u8>>>,
}

impl MockStorage {
    pub fn new() -> MockStorage {
        MockStorage {
            map: BTreeMap::new(),
        }
    }
}

pub struct Storage;
impl Storage {
    pub fn read(contract_key: ContractKey, key: &StorageKey) -> Option<Vec<u8>> {
        let ref storage = STORAGE.lock().unwrap();
        match storage.map.get(&(contract_key, *key)) {
            Some(val) => val.clone(),
            None => None,
        }
    }

    pub fn write(
        contract_key: ContractKey,
        key: &StorageKey,
        opt_new_value: Option<Vec<u8>>,
    ) -> DispatchResult {
        let ref mut storage = STORAGE.lock().unwrap();
        match opt_new_value {
            Some(new_value) => storage
                .map
                .insert((contract_key, key.clone()), Some(new_value)),
            None => storage.map.remove(&(contract_key, *key)),
        };

        Ok(())
    }
}
