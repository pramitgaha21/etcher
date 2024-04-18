use bitcoin::{
    absolute::LockTime,
    hashes::Hash,
    script::PushBytes,
    sighash::{EcdsaSighashType, SighashCache},
    Address, Network, OutPoint, ScriptBuf, Transaction, TxIn, TxOut, Txid, Witness,
};
use hex::ToHex;
use ic_cdk::api::management_canister::{
    bitcoin::{BitcoinNetwork, GetBalanceRequest, GetUtxosRequest, GetUtxosResponse, Utxo},
    ecdsa::{EcdsaKeyId, SignWithEcdsaArgument},
};
use ordinals::Runestone;
use std::str::FromStr;

use crate::utils::sec1_to_der;

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

pub async fn get_utxos(network: BitcoinNetwork, address: String) -> GetUtxosResponse {
    ic_cdk::api::management_canister::bitcoin::bitcoin_get_utxos(GetUtxosRequest {
        address,
        network,
        filter: None,
    })
    .await
    .unwrap()
    .0
}

pub fn build_etching_transaction(
    network: BitcoinNetwork,
    caller_btc_address: String,
    own_utxos: &[Utxo],
    rune_stone: Runestone,
) -> Transaction {
    let mocked_network = match network {
        BitcoinNetwork::Regtest => Network::Regtest,
        BitcoinNetwork::Testnet => Network::Testnet,
        BitcoinNetwork::Mainnet => Network::Bitcoin,
    };
    let address = Address::from_str(&caller_btc_address)
        .unwrap()
        .require_network(mocked_network)
        .unwrap();
    let input: Vec<TxIn> = own_utxos
        .into_iter()
        .map(|utxo| TxIn {
            previous_output: OutPoint {
                txid: Txid::from_raw_hash(Hash::from_slice(&utxo.outpoint.txid).unwrap()),
                vout: utxo.outpoint.vout,
            },
            sequence: bitcoin::Sequence(0xffffffff),
            witness: Witness::new(),
            script_sig: ScriptBuf::new(),
        })
        .collect();
    let script_pubkey = rune_stone.encipher();
    Transaction {
        lock_time: LockTime::ZERO,
        version: 2,
        input,
        output: vec![
            TxOut {
                value: 0,
                script_pubkey,
            },
            TxOut {
                script_pubkey: address.script_pubkey(),
                value: 10000,
            },
        ],
    }
}

pub async fn sign_and_send_txn(
    network: BitcoinNetwork,
    own_public_key: &[u8],
    own_address: String,
    mut txn: Transaction,
    key_id: EcdsaKeyId,
    derivation_path: Vec<Vec<u8>>,
) -> String {
    let mocked_network = match network {
        BitcoinNetwork::Regtest => Network::Regtest,
        BitcoinNetwork::Testnet => Network::Testnet,
        BitcoinNetwork::Mainnet => Network::Bitcoin,
    };
    let address = Address::from_str(&own_address)
        .unwrap()
        .require_network(mocked_network)
        .unwrap();
    let tx_clone = SighashCache::new(txn.clone());
    ic_cdk::println!("loop started");
    for (index, input) in txn.input.iter_mut().enumerate() {
        let sighash = tx_clone
            .legacy_signature_hash(
                index,
                &address.script_pubkey(),
                EcdsaSighashType::All.to_u32(),
            )
            .unwrap();
        let signature =
            ic_cdk::api::management_canister::ecdsa::sign_with_ecdsa(SignWithEcdsaArgument {
                message_hash: sighash.to_byte_array().to_vec(),
                derivation_path: derivation_path.clone(),
                key_id: key_id.clone(),
            })
            .await
            .unwrap()
            .0
            .signature;
        let der_signature = sec1_to_der(signature);
        let mut sig_with_hashtype = der_signature;
        sig_with_hashtype.push(EcdsaSighashType::All.to_u32() as u8);
        let sighash_as_push_bytes: &PushBytes = sig_with_hashtype.as_slice().try_into().unwrap();
        let own_public_key: &PushBytes = own_public_key.try_into().unwrap();
        input.script_sig = ScriptBuf::builder()
            .push_slice(sighash_as_push_bytes)
            .push_slice(own_public_key)
            .into_script();
        input.witness.clear();
    }
    let txn_in_bytes = bitcoin::consensus::serialize(&txn);
    ic_cdk::api::management_canister::bitcoin::bitcoin_send_transaction(
        ic_cdk::api::management_canister::bitcoin::SendTransactionRequest {
            transaction: txn_in_bytes,
            network,
        },
    )
    .await
    .unwrap();
    txn.txid().encode_hex()
}
