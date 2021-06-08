
use pallet_contracts_primitives::{ExecReturnValue};
use sp_runtime::{DispatchError};

pub type ContractKey = u64;
pub type StorageKey = [u8; 32];
pub type CodeHash = [u8; 32]; // FIXME: Placeholder Type
pub type ExecResult = Result<ExecReturnValue, ExecError>;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum ErrorOrigin {
	/// Caller error origin.
	///
	/// The error happened in the current exeuction context rather than in the one
	/// of the contract that is called into.
	Caller,
	/// The error happened during execution of the called contract.
	Callee,
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct ExecError {
	/// The reason why the execution failed.
	pub error: DispatchError,
	/// Origin of the error.
	pub origin: ErrorOrigin,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ExportedFunction {
	/// The constructor function which is executed on deployment of a contract.
	Constructor,
	/// The function which is executed when a contract is called.
	Call,
}

pub trait Ext {
    //type T: Config;

    /// Call (possibly transferring some amount of funds) into the specified account.
    ///
    /// Returns the original code size of the called contract.
    ///
    /// # Return Value
    ///
    /// Result<(ExecReturnValue, CodeSize), (ExecError, CodeSize)>
    fn call(
        &mut self,
        to: ContractKey,
        input_data: Vec<u8>,
    ) -> Result<(ExecReturnValue, u32), (ExecError, u32)>;

    /// Instantiate a contract from the given code.
    ///
    /// Returns the original code size of the called contract.
    /// The newly created account will be associated with `code`. `value` specifies the amount of value
    /// transferred from this to the newly created account (also known as endowment).
    ///
    /// # Return Value
    ///
    /// Result<(AccountId, ExecReturnValue, CodeSize), (ExecError, CodeSize)>
    fn instantiate(
        &mut self,
        code: ContractKey,
        input_data: Vec<u8>,
        salt: &[u8],
    ) -> Result<(ContractKey, ExecReturnValue, u32), (ExecError, u32)>;

    /// Transfer all funds to `beneficiary` and delete the contract.
    ///
    /// Returns the original code size of the terminated contract.
    /// Since this function removes the self contract eagerly, if succeeded, no further actions should
    /// be performed on this `Ext` instance.
    ///
    /// This function will fail if the same contract is present on the contract
    /// call stack.
    ///
    /// # Return Value
    ///
    /// Result<CodeSize, (DispatchError, CodeSize)>
    fn terminate(
        &mut self,
        beneficiary: &ContractKey,
    ) -> Result<u32, (DispatchError, u32)>;

    /// Restores the given destination contract sacrificing the current one.
    ///
    /// Since this function removes the self contract eagerly, if succeeded, no further actions should
    /// be performed on this `Ext` instance.
    ///
    /// This function will fail if the same contract is present
    /// on the contract call stack.
    ///
    /// # Return Value
    ///
    /// Result<(CallerCodeSize, DestCodeSize), (DispatchError, CallerCodeSize, DestCodesize)>
    /*
    fn restore_to(
        &mut self,
        dest: AccountIdOf<Self::T>,
        code_hash: CodeHash<Self::T>,
        rent_allowance: BalanceOf<Self::T>,
        delta: Vec<StorageKey>,
    ) -> Result<(u32, u32), (DispatchError, u32, u32)>;

    /// Transfer some amount of funds into the specified account.
    fn transfer(
        &mut self,
        to: &AccountIdOf<Self::T>,
        value: BalanceOf<Self::T>,
    ) -> DispatchResult;
    */
    /// Returns the storage entry of the executing account by the given `key`.
    ///
    /// Returns `None` if the `key` wasn't previously set by `set_storage` or
    /// was deleted.
    fn get_storage(&mut self, key: &StorageKey) -> Option<Vec<u8>>;

    /// Sets the storage entry by the given key to the specified value. If `value` is `None` then
    /// the storage entry is deleted.
    fn set_storage(&mut self, key: StorageKey, value: Option<Vec<u8>>);
    /*
    /// Returns a reference to the account id of the caller.
    fn caller(&self) -> &AccountIdOf<Self::T>;

    /// Returns a reference to the account id of the current contract.
    fn address(&self) -> &AccountIdOf<Self::T>;

    /// Returns the balance of the current contract.
    ///
    /// The `value_transferred` is already added.
    fn balance(&self) -> BalanceOf<Self::T>;

    /// Returns the value transferred along with this call or as endowment.
    fn value_transferred(&self) -> BalanceOf<Self::T>;

    /// Returns a reference to the timestamp of the current block
    fn now(&self) -> &MomentOf<Self::T>;

    /// Returns the minimum balance that is required for creating an account.
    fn minimum_balance(&self) -> BalanceOf<Self::T>;

    /// Returns the deposit required to create a tombstone upon contract eviction.
    fn tombstone_deposit(&self) -> BalanceOf<Self::T>;

    /// Returns a random number for the current block with the given subject.
    fn random(&self, subject: &[u8]) -> (SeedOf<Self::T>, BlockNumberOf<Self::T>);

    /// Deposit an event with the given topics.
    ///
    /// There should not be any duplicates in `topics`.
    fn deposit_event(&mut self, topics: Vec<TopicOf<Self::T>>, data: Vec<u8>);

    /// Set rent allowance of the contract
    fn set_rent_allowance(&mut self, rent_allowance: BalanceOf<Self::T>);

    /// Rent allowance of the contract
    fn rent_allowance(&mut self) -> BalanceOf<Self::T>;

    /// Returns the current block number.
    fn block_number(&self) -> BlockNumberOf<Self::T>;
    */
    /// Returns the maximum allowed size of a storage item.
    fn max_value_size(&self) -> u32;
    /*
    /// Returns the price for the specified amount of weight.
    fn get_weight_price(&self, weight: Weight) -> BalanceOf<Self::T>;

    /// Get a reference to the schedule used by the current call.
    fn schedule(&self) -> &Schedule<Self::T>;

    /// Information needed for rent calculations.
    fn rent_params(&self) -> &RentParams<Self::T>;

    /// Get a mutable reference to the nested gas meter.
    fn gas_meter(&mut self) -> &mut GasMeter<Self::T>;
    */
}

pub trait Executable: Sized {
	/// Load the executable from storage.
	fn from_storage(
		code_hash: CodeHash,
	) -> Result<Self, DispatchError>;

	/// Load the module from storage without re-instrumenting it.
	///
	/// A code module is re-instrumented on-load when it was originally instrumented with
	/// an older schedule. This skips this step for cases where the code storage is
	/// queried for purposes other than execution.
	// fn from_storage_noinstr(code_hash: CodeHash<T>) -> Result<Self, DispatchError>;

	/// Decrements the refcount by one and deletes the code if it drops to zero.
	//fn drop_from_storage(self);

	/// Increment the refcount by one. Fails if the code does not exist on-chain.
	///
	/// Returns the size of the original code.
	//fn add_user(code_hash: CodeHash<T>) -> Result<u32, DispatchError>;

	/// Decrement the refcount by one and remove the code when it drops to zero.
	///
	/// Returns the size of the original code.
	//fn remove_user(code_hash: CodeHash<T>) -> u32;

	/// Execute the specified exported function and return the result.
	///
	/// When the specified function is `Constructor` the executable is stored and its
	/// refcount incremented.
	///
	/// # Note
	///
	/// This functions expects to be executed in a storage transaction that rolls back
	/// all of its emitted storage changes.
	fn execute<Ext>(
		self,
		ext: &mut Ext,
		function: &ExportedFunction,
		input_data: Vec<u8>,
	) -> ExecResult;

	/// The code hash of the executable.
	fn code_hash(&self) -> &CodeHash;

	/// Size of the instrumented code in bytes.
	fn code_len(&self) -> u32;

	/// Sum of instrumented and pristine code len.
	fn aggregate_code_len(&self) -> u32;

	// The number of contracts using this executable.
	fn refcount(&self) -> u32;

	/// The storage that is occupied by the instrumented executable and its pristine source.
	///
	/// The returned size is already divided by the number of users who share the code.
	/// This is essentially `aggregate_code_len() / refcount()`.
	///
	/// # Note
	///
	/// This works with the current in-memory value of refcount. When calling any contract
	/// without refetching this from storage the result can be inaccurate as it might be
	/// working with a stale value. Usually this inaccuracy is tolerable.
	fn occupied_storage(&self) -> u32 {
		// We disregard the size of the struct itself as the size is completely
		// dominated by the code size.
		let len = self.aggregate_code_len();
		len.checked_div(self.refcount()).unwrap_or(len)
	}
}
