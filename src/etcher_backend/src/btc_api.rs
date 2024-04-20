use std::str::FromStr;

use bitcoin::{
    absolute::LockTime,
    hashes::Hash,
    opcodes,
    script::{Builder, PushBytes},
    secp256k1::{constants::SCHNORR_SIGNATURE_SIZE, schnorr, PublicKey, Secp256k1, XOnlyPublicKey},
    sighash::{EcdsaSighashType, Prevouts, SighashCache, TapSighashType},
    taproot::{ControlBlock, LeafVersion, Signature, TapLeafHash, TaprootBuilder},
    Address, Amount, FeeRate, Network, OutPoint, Script, ScriptBuf, Sequence, Transaction, TxIn,
    TxOut, Txid, Witness,
};
use hex::ToHex;
use ic_cdk::api::management_canister::bitcoin::{BitcoinNetwork, Utxo};
use ordinals::{Etching, Runestone};

use crate::{
    ecdsa_api::ecdsa_sign,
    schnorr_api,
    utils::{sec1_to_der, string_to_rune_and_spacer},
    EtchingArgs, STATE,
};

pub const SIG_HASH_TYPE: EcdsaSighashType = EcdsaSighashType::All;

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

pub async fn send_bitcoin_transaction(txn: Transaction) -> String {
    let transaction = bitcoin::consensus::serialize(&txn);
    let network = STATE.with_borrow(|state| *state.network.as_ref().unwrap());
    ic_cdk::api::management_canister::bitcoin::bitcoin_send_transaction(
        ic_cdk::api::management_canister::bitcoin::SendTransactionRequest {
            transaction,
            network,
        },
    )
    .await
    .unwrap();
    txn.txid().encode_hex()
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
        Amount::from_sat(
            (fee_rate.to_sat_per_kwu() as f64 * reveal_txn_clone.vsize() as f64).round() as u64,
        )
    };
    (reveal_txn, fee)
}

pub async fn build_and_sign_etching_transaction(
    derivation_path: &Vec<Vec<u8>>,
    owned_utxos: &[Utxo],
    ecdsa_public_key: &[u8],
    schnorr_public_key: &[u8],
    caller_p2pkh_address: String,
    etching_args: EtchingArgs,
) -> (Transaction, Transaction) {
    let mut reveal_input = vec![OutPoint::null()];
    let mut reveal_output = vec![];
    let (rune, spacers) = string_to_rune_and_spacer(&etching_args.rune);
    let symbol = match etching_args.symbol {
        None => None,
        Some(symbol) => {
            let symbol = char::from_u32(symbol).unwrap();
            Some(symbol)
        }
    };
    // TODO: setting up runestone
    let runestone = Runestone {
        edicts: vec![],
        etching: Some(Etching {
            divisibility: Some(etching_args.divisibility),
            premine: None,
            rune: Some(rune),
            spacers: None,
            symbol,
            terms: None,
            turbo: etching_args.turbo,
        }),
        mint: None,
        pointer: None,
    };
    let script_pubkey = runestone.encipher();
    if script_pubkey.len() > 82 {
        ic_cdk::trap("Exceeds OP_RETURN size of 82")
    }
    reveal_output.push(TxOut {
        script_pubkey,
        value: 0,
    });
    let commit_input_index = 0;
    let schnorr_public_key: XOnlyPublicKey =
        PublicKey::from_slice(schnorr_public_key).unwrap().into();
    // building of reveal script
    let secpk256k1 = Secp256k1::new();
    // TODO: adding rune as le bytes
    let builder = Builder::new()
        .push_slice(schnorr_public_key.serialize())
        .push_opcode(opcodes::all::OP_CHECKSIG);
    let reveal_script = builder.into_script();
    let taproot_send_info = TaprootBuilder::new()
        .add_leaf(0, reveal_script.clone())
        .expect("failed to add leaf")
        .finalize(&secpk256k1, schnorr_public_key)
        .expect("Failed to finalize taproot builder");

    let control_block = taproot_send_info
        .control_block(&(reveal_script.clone(), LeafVersion::TapScript))
        .expect("should compute control block");
    let network = STATE.with_borrow(|state| {
        let network = state.network.as_ref().unwrap();
        match network {
            BitcoinNetwork::Mainnet => Network::Bitcoin,
            BitcoinNetwork::Testnet => Network::Testnet,
            BitcoinNetwork::Regtest => Network::Regtest,
        }
    });
    let caller_address = Address::from_str(&caller_p2pkh_address)
        .unwrap()
        .assume_checked();
    let commit_tx_address = Address::p2tr_tweaked(taproot_send_info.output_key(), network);
    // TODO: calculating fee_rate
    let fee_rate = FeeRate::ZERO;
    let (_, reveal_fee) = build_reveal_transaction(
        commit_input_index,
        &control_block,
        fee_rate,
        reveal_output.clone(),
        reveal_input.clone(),
        &reveal_script,
    );

    let mut utxos_to_spend = vec![];
    let mut total_spent = 0;
    for utxo in owned_utxos.iter().rev() {
        total_spent += utxo.value;
        utxos_to_spend.push(utxo)
    }
    let input: Vec<TxIn> = utxos_to_spend
        .into_iter()
        .map(|utxo| TxIn {
            previous_output: OutPoint {
                txid: Txid::from_raw_hash(Hash::from_slice(&utxo.outpoint.txid).unwrap()),
                vout: utxo.outpoint.vout,
            },
            sequence: Sequence::from_height(Runestone::COMMIT_CONFIRMATIONS - 1),
            witness: Witness::new(),
            script_sig: Builder::new().into_script(),
        })
        .collect();
    let mut commit_tx = Transaction {
        input,
        output: vec![TxOut {
            script_pubkey: commit_tx_address.script_pubkey(),
            value: total_spent,
        }],
        lock_time: LockTime::ZERO,
        version: 2,
    };
    // We assume that we spend a single P2PKH output
    let sig_vbytes = 73;
    let commit_fee =
        FeeRate::from_sat_per_vb(fee_rate.to_sat_per_kwu() * commit_tx.vsize() as u64 + sig_vbytes)
            .unwrap();
    commit_tx.output[0].value = total_spent - commit_fee.to_sat_per_kwu();
    let commit_tx_clone = SighashCache::new(commit_tx.clone());
    for (i, input) in commit_tx.input.iter_mut().enumerate() {
        let sighash = commit_tx_clone
            .legacy_signature_hash(i, &caller_address.script_pubkey(), SIG_HASH_TYPE.to_u32())
            .unwrap();
        let signature = ecdsa_sign(sighash.to_byte_array().to_vec(), derivation_path.clone()).await;
        let der_signature = sec1_to_der(signature);
        let mut sig_with_hashtype = der_signature;
        sig_with_hashtype.push(EcdsaSighashType::All.to_u32() as u8);
        let sighash_as_push_bytes: &PushBytes = sig_with_hashtype.as_slice().try_into().unwrap();
        let ecdsa_public_key_as_push_bytes: &PushBytes = ecdsa_public_key.try_into().unwrap();
        input.script_sig = ScriptBuf::builder()
            .push_slice(sighash_as_push_bytes)
            .push_slice(ecdsa_public_key_as_push_bytes)
            .into_script();
        input.witness.clear();
    }
    let (vout, _commit_output) = commit_tx
        .output
        .iter()
        .enumerate()
        .find(|(_vout, output)| output.script_pubkey == commit_tx_address.script_pubkey())
        .expect("Failed to find");
    reveal_input[commit_input_index] = OutPoint {
        txid: commit_tx.txid(),
        vout: vout.try_into().unwrap(),
    };
    let (mut reveal_tx, _) = build_reveal_transaction(
        commit_input_index,
        &control_block,
        fee_rate,
        reveal_output,
        reveal_input,
        &reveal_script,
    );
    let mut prevouts = vec![];
    prevouts.push(commit_tx.output[vout].clone());
    let mut sighash_cache = SighashCache::new(&mut reveal_tx);
    let sighash = sighash_cache
        .taproot_script_spend_signature_hash(
            commit_input_index,
            &Prevouts::All(&prevouts),
            TapLeafHash::from_script(&reveal_script, LeafVersion::TapScript),
            TapSighashType::Default,
        )
        .expect("Failed to taproot spend");
    let msg = sighash.to_byte_array().to_vec();
    let schnorr_signature = schnorr_api::schnorr_sign(msg, derivation_path.clone()).await;
    let witness = sighash_cache
        .witness_mut(commit_input_index)
        .expect("failed getting mutable witness");
    witness.push(
        Signature {
            sig: schnorr::Signature::from_slice(schnorr_signature.as_slice())
                .expect("Failed to parse"),
            hash_ty: TapSighashType::Default,
        }
        .to_vec(),
    );
    witness.push(reveal_script);
    witness.push(&control_block.serialize());
    (commit_tx, reveal_tx)
}
