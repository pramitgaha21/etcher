use bitcoin::{
    absolute::LockTime,
    secp256k1::constants::SCHNORR_SIGNATURE_SIZE,
    taproot::{ControlBlock, Signature},
    Amount, FeeRate, OutPoint, Script, Sequence, Transaction, TxIn, TxOut, Witness,
};
use ic_cdk::api::management_canister::bitcoin::Utxo;
use ordinals::{Etching, Runestone};

use crate::STATE;

pub async fn get_balance_of(address: String) -> u64 {
    let network = STATE.with_borrow(|state| *state.network.as_ref().unwrap());
    ic_cdk::api::management_canister::bitcoin::bitcoin_get_balance(
        ic_cdk::api::management_canister::bitcoin::GetBalanceRequest {
            address,
            network,
            min_confirmations: None,
        },
    )
    .await
    .unwrap()
    .0
}

pub async fn get_utxos_of(address: String) -> Vec<Utxo> {
    let network = STATE.with_borrow(|state| *state.network.as_ref().unwrap());
    ic_cdk::api::management_canister::bitcoin::bitcoin_get_utxos(
        ic_cdk::api::management_canister::bitcoin::GetUtxosRequest {
            address,
            network,
            filter: None,
        },
    )
    .await
    .unwrap()
    .0
    .utxos
}

pub fn build_reveal_transaction(
    commit_input_index: usize,
    control_block: &ControlBlock,
    fee_rate: FeeRate,
    output: Vec<TxOut>,
    input: Vec<OutPoint>,
    script: &Script,
) -> (Transaction, Amount) {
    let reveal_txn = Transaction {
        input: input
            .into_iter()
            .map(|previous_output| TxIn {
                previous_output,
                script_sig: Script::builder().into_script(),
                witness: Witness::new(),
                sequence: Sequence::from_height(Runestone::COMMIT_CONFIRMATIONS - 1),
            })
            .collect(),
        output,
        lock_time: LockTime::ZERO,
        version: 2,
    };
    let fee = {
        let mut reveal_txn_clone = reveal_txn.clone();
        for (current_index, txin) in reveal_txn_clone.input.iter_mut().enumerate() {
            if current_index == commit_input_index {
                txin.witness.push(
                    Signature::from_slice(&[0; SCHNORR_SIGNATURE_SIZE])
                        .unwrap()
                        .to_vec(),
                );
                txin.witness.push(script);
                txin.witness.push(control_block.serialize());
            } else {
                txin.witness = Witness::from_slice(&[&[0; SCHNORR_SIGNATURE_SIZE]]);
            }
        }
        todo!()
    };
    (reveal_txn, fee)
}

pub fn build_etching_transaction(
    etching: Etching,
    derivation_path: &Vec<Vec<u8>>,
) -> (Transaction, Transaction) {
    todo!()
}

pub fn sign_transaction() {}
