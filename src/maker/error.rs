//! All Maker related errors.

use std::sync::{MutexGuard, PoisonError, RwLockReadGuard, RwLockWriteGuard};

use bitcoin::secp256k1;

use crate::{protocol::error::ContractError, wallet::WalletError};

/// Enum to handle Maker related errors.
#[derive(Debug)]
pub enum MakerError {
    IO(std::io::Error),
    UnexpectedMessage { expected: String, got: String },
    General(&'static str),
    MutexPossion,
    Secp(secp256k1::Error),
    ContractError(ContractError),
    Wallet(WalletError),
    Deserialize(serde_cbor::Error),
}

impl From<std::io::Error> for MakerError {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<serde_cbor::Error> for MakerError {
    fn from(value: serde_cbor::Error) -> Self {
        Self::Deserialize(value)
    }
}

impl<'a, T> From<PoisonError<RwLockReadGuard<'a, T>>> for MakerError {
    fn from(_: PoisonError<RwLockReadGuard<'a, T>>) -> Self {
        Self::MutexPossion
    }
}

impl<'a, T> From<PoisonError<RwLockWriteGuard<'a, T>>> for MakerError {
    fn from(_: PoisonError<RwLockWriteGuard<'a, T>>) -> Self {
        Self::MutexPossion
    }
}

impl<'a, T> From<PoisonError<MutexGuard<'a, T>>> for MakerError {
    fn from(_: PoisonError<MutexGuard<'a, T>>) -> Self {
        Self::MutexPossion
    }
}

impl From<secp256k1::Error> for MakerError {
    fn from(value: secp256k1::Error) -> Self {
        Self::Secp(value)
    }
}

impl From<ContractError> for MakerError {
    fn from(value: ContractError) -> Self {
        Self::ContractError(value)
    }
}

impl From<WalletError> for MakerError {
    fn from(value: WalletError) -> Self {
        Self::Wallet(value)
    }
}
