use super::{Destination, SendAmount, UTXOSpendInfo};
use crate::wallet::WalletError;
use bitcoin::Transaction;
use bitcoind::bitcoincore_rpc::json::ListUnspentResultEntry;

pub trait TransactionBuilder {
    fn build_tx(&self, params: BuildTxParams) -> Result<Transaction, WalletError>;
    fn sign_tx(&self, tx: &mut Transaction, inputs: &[TxInput]) -> Result<(), WalletError>;
}

pub struct BuildTxParams {
    pub fee_rate: f64,
    pub amount: SendAmount,
    pub destination: Destination,
    pub coins_to_spend: Vec<(ListUnspentResultEntry, UTXOSpendInfo)>,
}

pub struct TxInput {
    pub utxo: ListUnspentResultEntry,
    pub spend_info: UTXOSpendInfo,
}

pub struct BasicTxBuilder;
pub struct FidelityTxBuilder;
pub struct ContractTxBuilder;

impl TransactionBuilder for BasicTxBuilder {
    fn build_tx() -> Result<Transaction, WalletError> {
        todo!()
    }

    fn sign_tx(&self, tx: &mut Transaction, inputs: &[TxInput]) -> Result<(), WalletError> {
        todo!()
    }
}

impl TransactionBuilder for FidelityTxBuilder {
    fn build_tx(&self, params: BuildTx) -> Result<Transaction, WalletError> {
        todo!()
    }

    fn sign_tx(&self, tx: &mut Transaction, inputs: &[TxInput]) -> Result<(), WalletError> {
        todo!()
    }
}
