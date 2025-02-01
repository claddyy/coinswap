//! The Coinswap Wallet (unsecured). Used by both the Taker and Maker.

mod api;
mod direct_send;
mod error;
mod fidelity;
mod funding;
mod rpc;
mod storage;
mod swapcoin;
mod transaction;

pub(crate) use api::{UTXOSpendInfo, Wallet};
pub use direct_send::{Destination, SendAmount};
pub use error::WalletError;
pub(crate) use fidelity::{fidelity_redeemscript, FidelityBond, FidelityError};
pub use rpc::RPCConfig;
pub(crate) use swapcoin::{
    IncomingSwapCoin, OutgoingSwapCoin, SwapCoin, WalletSwapCoin, WatchOnlySwapCoin,
};
