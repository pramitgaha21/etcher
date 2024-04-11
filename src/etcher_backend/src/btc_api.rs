use bitcoin::{Address, Transaction, Txid};
use candid::Principal;
use ic_cdk::api::{
    call::call_with_payment,
    management_canister::bitcoin::{
        BitcoinNetwork, GetBalanceRequest, GetCurrentFeePercentilesRequest, MillisatoshiPerByte,
    },
};

pub async fn get_balance_of(network: BitcoinNetwork, of: &str) -> u64 {
    ic_cdk::api::management_canister::bitcoin::bitcoin_get_balance(GetBalanceRequest {
        address: of.to_string(),
        network,
        min_confirmations: None,
    })
    .await
    .unwrap()
    .0
}

pub fn build_etching_transaction(network: BitcoinNetwork) -> Transaction {
    // TODO
    unimplemented!()
}

pub fn build_transfer_transaction() -> Transaction {
    todo!()
}

pub async fn sign_and_send_txn(txn: Transaction) -> Txid {
    todo!()
}
