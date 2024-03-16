use candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum TransferError {
    NonExistingTokenId,
    InvalidRecipient,
    Unauthorized,
    TooOld,
    CreatedInFuture { ledger_time: u64 },
    Duplicate { duplicate_of: u128 },
    GenericError { error_code: u128, message: String },
    GenericBatchError { error_code: u128, message: String },
}

#[derive(CandidType, Clone)]
pub enum ApprovalError {
    Unauthorized { tokens_ids: Vec<u128> },
    TooOld,
    TemporaryUnavailable,
    NonExistingTokenId,
    InvalidSpender,
    GenericError { error_code: u128, msg: String },
    GenericBatchError { error_code: u128, message: String },
}
