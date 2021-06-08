mod exec;
mod storage;
mod wasm;

use pallet_contracts_primitives::{DispatchError};

#[macro_use]
extern crate lazy_static;

pub enum Error {
    /// A new schedule must have a greater version than the current one.
    InvalidScheduleVersion,
    /// An origin must be signed or inherent and auxiliary sender only provided on inherent.
    InvalidSurchargeClaim,
    /// Cannot restore from nonexisting or tombstone contract.
    InvalidSourceContract,
    /// Cannot restore to nonexisting or alive contract.
    InvalidDestinationContract,
    /// Tombstones don't match.
    InvalidTombstone,
    /// An origin TrieId written in the current block.
    InvalidContractOrigin,
    /// The executed contract exhausted its gas limit.
    OutOfGas,
    /// The output buffer supplied to a contract API call was too small.
    OutputBufferTooSmall,
    /// Performing the requested transfer would have brought the contract below
    /// the subsistence threshold. No transfer is allowed to do this in order to allow
    /// for a tombstone to be created. Use `seal_terminate` to remove a contract without
    /// leaving a tombstone behind.
    BelowSubsistenceThreshold,
    /// The newly created contract is below the subsistence threshold after executing
    /// its contructor. No contracts are allowed to exist below that threshold.
    NewContractNotFunded,
    /// Performing the requested transfer failed for a reason originating in the
    /// chosen currency implementation of the runtime. Most probably the balance is
    /// too low or locks are placed on it.
    TransferFailed,
    /// Performing a call was denied because the calling depth reached the limit
    /// of what is specified in the schedule.
    MaxCallDepthReached,
    /// The contract that was called is either no contract at all (a plain account)
    /// or is a tombstone.
    NotCallable,
    /// The code supplied to `instantiate_with_code` exceeds the limit specified in the
    /// current schedule.
    CodeTooLarge,
    /// No code could be found at the supplied code hash.
    CodeNotFound,
    /// A buffer outside of sandbox memory was passed to a contract API function.
    OutOfBounds,
    /// Input passed to a contract API function failed to decode as expected type.
    DecodingFailed,
    /// Contract trapped during execution.
    ContractTrapped,
    /// The size defined in `T::MaxValueSize` was exceeded.
    ValueTooLarge,
    /// The action performed is not allowed while the contract performing it is already
    /// on the call stack. Those actions are contract self destruction and restoration
    /// of a tombstone.
    ReentranceDenied,
    /// `seal_input` was called twice from the same contract execution context.
    InputAlreadyRead,
    /// The subject passed to `seal_random` exceeds the limit.
    RandomSubjectTooLong,
    /// The amount of topics passed to `seal_deposit_events` exceeds the limit.
    TooManyTopics,
    /// The topics passed to `seal_deposit_events` contains at least one duplicate.
    DuplicateTopics,
    /// The chain does not provide a chain extension. Calling the chain extension results
    /// in this error. Note that this usually  shouldn't happen as deploying such contracts
    /// is rejected.
    NoChainExtension,
    /// Removal of a contract failed because the deletion queue is full.
    ///
    /// This can happen when either calling [`Pallet::claim_surcharge`] or `seal_terminate`.
    /// The queue is filled by deleting contracts and emptied by a fixed amount each block.
    /// Trying again during another block is the only way to resolve this issue.
    DeletionQueueFull,
    /// A contract could not be evicted because it has enough balance to pay rent.
    ///
    /// This can be returned from [`Pallet::claim_surcharge`] because the target
    /// contract has enough balance to pay for its rent.
    ContractNotEvictable,
    /// A storage modification exhausted the 32bit type that holds the storage size.
    ///
    /// This can either happen when the accumulated storage in bytes is too large or
    /// when number of storage items is too large.
    StorageExhausted,
    /// A contract with the same AccountId already exists.
    DuplicateContract,
    /// A contract self destructed in its constructor.
    ///
    /// This can be triggered by a call to `seal_terminate` or `seal_restore_to`.
    TerminatedInConstructor,
}

impl From<Error> for DispatchError {
    fn from(err: Error) -> Self {
        DispatchError::Module{
            index: 0,
            error: 0,
            message: Some("DispatchError"),
        }
    }
}
