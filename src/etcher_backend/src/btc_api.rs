use std::str::FromStr;

use bitcoin::{
    absolute::LockTime,
    consensus,
    hashes::{sha256, Hash},
    opcodes,
    script::{Builder, PushBytes},
    secp256k1::{
        constants::SCHNORR_SIGNATURE_SIZE, schnorr, Message, PublicKey, Secp256k1, XOnlyPublicKey,
    },
    sighash::{EcdsaSighashType, Prevouts, SighashCache, TapSighashType},
    taproot::{ControlBlock, LeafVersion, Signature, TapLeafHash, TaprootBuilder},
    Address, Amount, FeeRate, Network, OutPoint, Script, ScriptBuf, Sequence, Transaction, TxIn,
    TxOut, Txid, Witness,
};
use hex::ToHex;
use ic_cdk::api::management_canister::bitcoin::{BitcoinNetwork, GetUtxosResponse, Utxo};
use ordinals::{Artifact, Etching, Runestone, SpacedRune, Terms};

use crate::{
    ecdsa_api::ecdsa_sign,
    schnorr_api,
    tags::Tag,
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

pub async fn get_utxos_of(address: String) -> GetUtxosResponse {
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

pub async fn build_and_sign_etching_transaction_v2(
    derivation_path: &Vec<Vec<u8>>,
    owned_utxos: &[Utxo],
    ecdsa_public_key: &[u8],
    schnorr_public_key: &[u8],
    caller_p2pkh_address: String,
    etching_args: EtchingArgs,
) -> (Transaction, Transaction) {
    let (rune, spacers) = string_to_rune_and_spacer(&etching_args.rune);
    // Building the reveal script
    let secp256k1 = Secp256k1::new();
    let schnorr_public_key: XOnlyPublicKey =
        PublicKey::from_slice(schnorr_public_key).unwrap().into();
    let reveal_script = Builder::new()
        .push_slice::<&PushBytes>(rune.commitment().as_slice().try_into().unwrap())
        .push_slice(schnorr_public_key.serialize())
        .push_opcode(opcodes::all::OP_CHECKSIG)
        .into_script();

    let taproot_send_info = TaprootBuilder::new()
        .add_leaf(0, reveal_script.clone())
        .unwrap()
        .finalize(&secp256k1, schnorr_public_key)
        .unwrap();
    let control_block = taproot_send_info
        .control_block(&(reveal_script.clone(), LeafVersion::TapScript))
        .unwrap();
    // convering address
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

    let mut reveal_input = vec![OutPoint::null()];
    let mut reveal_output = vec![];
    let symbol = match etching_args.symbol {
        None => None,
        Some(symbol) => {
            let symbol = char::from_u32(symbol).unwrap();
            Some(symbol)
        }
    };

    let runestone = Runestone {
        edicts: vec![],
        etching: Some(Etching {
            divisibility: Some(etching_args.divisibility),
            premine: None,
            rune: Some(rune),
            spacers: Some(spacers),
            terms: Some(Terms {
                amount: etching_args.amount,
                cap: etching_args.cap,
                height: (None, None),
                offset: (None, None),
            }),
            symbol,
            turbo: etching_args.turbo,
        }),
        pointer: None,
        mint: None,
    };
    reveal_output.push(TxOut {
        script_pubkey: runestone.encipher(),
        value: 0,
    });
    let fee_rate = FeeRate::from_sat_per_vb(10).unwrap();
    let (_, reveal_fee) = build_reveal_transaction(
        0,
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
        utxos_to_spend.push(utxo);
    }
    let input = utxos_to_spend
        .into_iter()
        .map(|utxo| TxIn {
            previous_output: OutPoint {
                txid: Txid::from_raw_hash(Hash::from_slice(&utxo.outpoint.txid).unwrap()),
                vout: utxo.outpoint.vout,
            },
            sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
            script_sig: ScriptBuf::new(),
            witness: Witness::new(),
        })
        .collect::<Vec<_>>();
    let mut commit_tx = Transaction {
        input,
        output: vec![TxOut {
            script_pubkey: commit_tx_address.script_pubkey(),
            value: total_spent,
        }],
        lock_time: LockTime::ZERO,
        version: 2,
    };

    let sig_vbytes = 73;
    let commit_fee =
        FeeRate::from_sat_per_vb(fee_rate.to_sat_per_kwu() * commit_tx.vsize() as u64 + sig_vbytes)
            .unwrap();

    commit_tx.output[0].value = total_spent - commit_fee.to_sat_per_kwu();
    let commit_tx_cache = SighashCache::new(commit_tx.clone());
    for (i, input) in commit_tx.input.iter_mut().enumerate() {
        let sighash = commit_tx_cache
            .legacy_signature_hash(i, &caller_address.script_pubkey(), SIG_HASH_TYPE.to_u32())
            .unwrap();
        let signature = ecdsa_sign(sighash.to_byte_array().to_vec(), derivation_path.clone()).await;
        let mut signature = sec1_to_der(signature);
        signature.push(EcdsaSighashType::All.to_u32() as u8);
        input.script_sig = ScriptBuf::builder()
            .push_slice::<&PushBytes>(signature.as_slice().try_into().unwrap())
            .push_slice::<&PushBytes>(ecdsa_public_key.try_into().unwrap())
            .into_script();
    }
    let (vout, _commit_output) = commit_tx
        .output
        .iter()
        .enumerate()
        .find(|(_vout, output)| output.script_pubkey == commit_tx_address.script_pubkey())
        .expect("should find sat commit/inscription output");
    reveal_input[0] = OutPoint {
        txid: commit_tx.txid(),
        vout: vout as u32,
    };
    reveal_output.push(TxOut {
        value: 10_000,
        script_pubkey: caller_address.script_pubkey(),
    });
    // let (mut reveal_tx, _) = build_reveal_transaction(
    //     0,
    //     &control_block,
    //     fee_rate,
    //     reveal_output.clone(),
    //     reveal_input.clone(),
    //     &reveal_script,
    // );
    let mut reveal_tx = Transaction {
        input: reveal_input
            .into_iter()
            .map(|outpoint| TxIn {
                previous_output: outpoint,
                script_sig: ScriptBuf::new(),
                witness: Witness::new(),
                sequence: Sequence::from_height(Runestone::COMMIT_CONFIRMATIONS - 1),
            })
            .collect(),
        output: reveal_output,
        lock_time: LockTime::ZERO,
        version: 2,
    };
    // let prevouts = vec![commit_tx.output[vout].clone()];
    let mut sighhash_cache = SighashCache::new(&mut reveal_tx);
    let mut signing_data = vec![];
    let leaf_hash = TapLeafHash::from_script(&reveal_script, LeafVersion::TapScript);
    sighhash_cache
        .taproot_encode_signing_data_to(
            &mut signing_data,
            0,
            &Prevouts::All(commit_tx.output.as_slice()),
            None,
            Some((leaf_hash, 0xFFFFFFFF)),
            TapSighashType::Default,
        )
        .unwrap();
    let tag = b"TapSighash";
    let mut hashed_tag = sha256::Hash::hash(tag).to_byte_array().to_vec();
    let mut prefix = hashed_tag.clone();

    prefix.append(&mut hashed_tag);
    let signing_data: Vec<_> = prefix.iter().chain(signing_data.iter()).cloned().collect();

    let schnorr_signature =
        schnorr_api::schnorr_sign(signing_data.clone(), derivation_path.clone()).await;

    // Verify the signature to be sure that signing works
    let secp = bitcoin::secp256k1::Secp256k1::verification_only();

    let sig_ = schnorr::Signature::from_slice(&schnorr_signature).unwrap();

    let digest = sha256::Hash::hash(&signing_data).to_byte_array();
    let msg = Message::from_slice(&digest).unwrap();

    assert!(secp
        .verify_schnorr(&sig_, &msg, &schnorr_public_key)
        .is_ok());
    let witness = sighhash_cache.witness_mut(0).unwrap();
    witness.push(
        Signature {
            sig: schnorr::Signature::from_slice(&schnorr_signature).unwrap(),
            hash_ty: TapSighashType::Default,
        }
        .to_vec(),
    );
    witness.push(reveal_script);
    witness.push(&control_block.serialize());
    if Runestone::decipher(&reveal_tx).unwrap() != Artifact::Runestone(runestone) {
        ic_cdk::trap("Runestone mismatched")
    }
    let commit_tx_bytes = consensus::serialize(&commit_tx);
    let reveal_tx_bytes = consensus::serialize(&reveal_tx);
    ic_cdk::println!("Signed commit txn: {}", hex::encode(commit_tx_bytes));
    ic_cdk::println!("Signed reveal txn: {}", hex::encode(reveal_tx_bytes));
    (commit_tx, reveal_tx)
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
            spacers: Some(spacers),
            symbol,
            terms: Some(Terms {
                amount: etching_args.amount,
                cap: etching_args.cap,
                offset: (None, None),
                height: (None, None),
            }),
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
        script_pubkey: script_pubkey.clone(),
        value: 0,
    });
    let commit_input_index = 0;
    let schnorr_public_key: XOnlyPublicKey =
        PublicKey::from_slice(schnorr_public_key).unwrap().into();
    // building of reveal script
    let secpk256k1 = Secp256k1::new();
    let mut runes_as_lebytes = rune.0.to_le_bytes().to_vec();
    let mut count = runes_as_lebytes.len() - 1;
    while count != 0 {
        if runes_as_lebytes[count] == 0 {
            runes_as_lebytes.pop();
            count -= 1;
            continue;
        } else {
            break;
        }
    }
    let runes_as_pushbytes: &PushBytes = runes_as_lebytes.as_slice().try_into().unwrap();
    let builder = Builder::new()
        .push_slice(runes_as_pushbytes)
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
    let fee_rate = FeeRate::from_sat_per_vb(10).unwrap();
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
            sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
            witness: Witness::new(),
            script_sig: ScriptBuf::new(),
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
    reveal_output = vec![TxOut {
        script_pubkey: caller_address.script_pubkey(),
        value: total_spent - commit_fee.to_sat_per_kwu() - reveal_fee.to_sat(),
    }];
    let (mut reveal_tx, _) = build_reveal_transaction(
        commit_input_index,
        &control_block,
        fee_rate,
        reveal_output,
        reveal_input,
        &reveal_script,
    );
    let leaf_hash = TapLeafHash::from_script(&reveal_script, LeafVersion::TapScript);
    let mut sighash_cache = SighashCache::new(&mut reveal_tx);
    let mut signing_data = vec![];
    sighash_cache
        .taproot_encode_signing_data_to(
            &mut signing_data,
            commit_input_index,
            &Prevouts::All(commit_tx.output.as_slice()),
            None,
            Some((leaf_hash, 0xFFFFFFFF)),
            TapSighashType::Default,
        )
        .unwrap();
    let tag = b"TapSighash";
    let mut hashed_tag = sha256::Hash::hash(tag).to_byte_array().to_vec();
    let mut prefix = hashed_tag.clone();
    prefix.append(&mut hashed_tag);
    let signing_data: Vec<_> = prefix.iter().chain(signing_data.iter()).cloned().collect();
    let schnorr_signature =
        schnorr_api::schnorr_sign(signing_data.clone(), derivation_path.clone()).await;

    // Verify the signature to be sure that signing works
    let secp = bitcoin::secp256k1::Secp256k1::verification_only();

    let sig_ = schnorr::Signature::from_slice(&schnorr_signature).unwrap();

    let digest = sha256::Hash::hash(&signing_data).to_byte_array();
    let msg = Message::from_slice(&digest).unwrap();

    assert!(secp
        .verify_schnorr(&sig_, &msg, &schnorr_public_key)
        .is_ok());

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
    if Runestone::decipher(&reveal_tx).unwrap() != Artifact::Runestone(runestone) {
        ic_cdk::trap("Runestone mismatched")
    }
    let reveal_tx_bytes = consensus::serialize(&reveal_tx);
    ic_cdk::println!("Signed reveal txn: {}", hex::encode(reveal_tx_bytes));
    (commit_tx, reveal_tx)
}

// pub fn build_schnorr_signed_etching_transaction(
//     derivation_path: &Vec<Vec<u8>>,
//     owned_utxos: &[Utxo],
//     schnorr_public_key: &[u8],
//     caller_p2pkh_address: String,
//     etching_args: EtchingArgs,
// ) -> Transaction {
//     let (rune, spacers) = string_to_rune_and_spacer(&etching_args.rune);
//     let symbol = match etching_args.symbol {
//         None => None,
//         Some(symbol) => {
//             let symbol = char::from_u32(symbol).unwrap();
//             Some(symbol)
//         }
//     };
//     let runestone = Runestone {
//         mint: None,
//         edicts: vec![],
//         pointer: None,
//         etching: Some(Etching {
//             divisibility: Some(etching_args.divisibility),
//             premine: None,
//             rune: Some(rune),
//             spacers: Some(spacers),
//             symbol,
//             terms: None,
//             turbo: etching_args.turbo,
//         }),
//     };
//     let mut input = vec![];
//     for utxo in owned_utxos {
//         input.push(TxIn {
//             previous_output: OutPoint::new(
//                 Txid::from_raw_hash(Hash::from_slice(&utxo.outpoint.txid).unwrap()),
//                 utxo.outpoint.vout,
//             ),
//             script_sig: ScriptBuf::new(),
//             sequence: Sequence::MAX,
//             witness: Witness::new(),
//         })
//     }
//     let mut tx = Transaction{
//         version: 2,
//         lock_time: LockTime::ZERO,
//
//     }
//     todo!()
// }

pub async fn build_and_sign_etching_transaction_v3(
    derivation_path: &Vec<Vec<u8>>,
    owned_utxos: &[Utxo],
    ecdsa_public_key: &[u8],
    schnorr_public_key: &[u8],
    caller_p2pkh_address: String,
    etching_args: EtchingArgs,
) -> (Address, Transaction, Transaction) {
    let SpacedRune { rune, spacers } = SpacedRune::from_str(&etching_args.rune).unwrap();
    ic_cdk::println!("rune: {}", rune);
    ic_cdk::println!("rune's commitment: {:?}", rune.commitment());
    let symbol = match etching_args.symbol {
        None => None,
        Some(symbol) => {
            let symbol = char::from_u32(symbol).unwrap();
            Some(symbol)
        }
    };
    // building the reveal script
    let secp256k1 = Secp256k1::new();
    let schnorr_public_key: XOnlyPublicKey =
        PublicKey::from_slice(schnorr_public_key).unwrap().into();
    const PROTOCOL_ID: [u8; 3] = *b"ord";
    let mut reveal_script = Builder::new()
        .push_slice(schnorr_public_key.serialize())
        .push_opcode(opcodes::all::OP_CHECKSIG)
        .push_opcode(opcodes::OP_FALSE)
        .push_opcode(opcodes::all::OP_IF)
        .push_slice(PROTOCOL_ID);

    Tag::Rune.encode(&mut reveal_script, &Some(rune.commitment()));

    let reveal_script = reveal_script
        .push_opcode(opcodes::all::OP_ENDIF)
        .into_script();

    let taproot_send_info = TaprootBuilder::new()
        .add_leaf(0, reveal_script.clone())
        .unwrap()
        .finalize(&secp256k1, schnorr_public_key)
        .unwrap();

    let control_block = taproot_send_info
        .control_block(&(reveal_script.clone(), LeafVersion::TapScript))
        .unwrap();

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

    let mut reveal_input = vec![OutPoint::null()];
    let mut reveal_output = vec![];

    let runestone = Runestone {
        etching: Some(Etching {
            rune: Some(rune),
            symbol,
            divisibility: Some(etching_args.divisibility),
            premine: None,
            spacers: Some(spacers),
            turbo: etching_args.turbo,
            terms: Some(Terms {
                cap: etching_args.cap,
                amount: etching_args.amount,
                height: (None, None),
                offset: (None, None),
            }),
        }),
        edicts: vec![],
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
    let fee_rate = FeeRate::from_sat_per_vb(etching_args.fee_rate.unwrap_or(10)).unwrap();
    let (_, reveal_fee) = build_reveal_transaction(
        0,
        &control_block,
        fee_rate,
        reveal_output.clone(),
        reveal_input.clone(),
        &reveal_script,
    );

    let mut utxos_to_spend = vec![];
    let mut total_spent = 0;
    owned_utxos.iter().for_each(|utxo| {
        total_spent += utxo.value;
        utxos_to_spend.push(utxo);
    });

    let input = utxos_to_spend
        .into_iter()
        .map(|utxo| TxIn {
            previous_output: OutPoint::new(
                Txid::from_raw_hash(Hash::from_slice(&utxo.outpoint.txid).unwrap()),
                utxo.outpoint.vout,
            ),
            sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
            witness: Witness::new(),
            script_sig: ScriptBuf::new(),
        })
        .collect::<Vec<TxIn>>();

    let mut commit_tx = Transaction {
        input,
        output: vec![TxOut {
            script_pubkey: commit_tx_address.script_pubkey(),
            value: total_spent,
        }],
        lock_time: LockTime::ZERO,
        version: 2,
    };

    let sig_bytes = 73;
    let commit_fee =
        FeeRate::from_sat_per_vb(fee_rate.to_sat_per_kwu() * commit_tx.vsize() as u64 + sig_bytes)
            .unwrap();
    commit_tx.output[0].value = total_spent - commit_fee.to_sat_per_kwu();

    // signing the commit_tx
    let commit_tx_cache = SighashCache::new(commit_tx.clone());
    for (index, input) in commit_tx.input.iter_mut().enumerate() {
        let sighash = commit_tx_cache
            .legacy_signature_hash(
                index,
                &caller_address.script_pubkey(),
                SIG_HASH_TYPE.to_u32(),
            )
            .unwrap();
        let signature = ecdsa_sign(sighash.to_byte_array().to_vec(), derivation_path.clone()).await;
        let der_signature = sec1_to_der(signature);
        let mut sig_with_hashtype = der_signature;
        sig_with_hashtype.push(SIG_HASH_TYPE.to_u32() as u8);
        input.script_sig = ScriptBuf::builder()
            .push_slice::<&PushBytes>(sig_with_hashtype.as_slice().try_into().unwrap())
            .push_slice::<&PushBytes>(ecdsa_public_key.try_into().unwrap())
            .into_script();
        input.witness.clear();
    }
    let (vout, _) = commit_tx
        .output
        .iter()
        .enumerate()
        .find(|(_vout, output)| output.script_pubkey == commit_tx_address.script_pubkey())
        .unwrap();
    reveal_input[0] = OutPoint {
        txid: commit_tx.txid(),
        vout: vout as u32,
    };
    reveal_output.push(TxOut {
        script_pubkey: caller_address.script_pubkey(),
        value: total_spent - commit_fee.to_sat_per_kwu() - reveal_fee.to_sat(),
    });

    // building the reveal txn
    let mut reveal_tx = Transaction {
        version: 2,
        lock_time: LockTime::ZERO,
        input: reveal_input
            .iter()
            .map(|outpoint| TxIn {
                previous_output: *outpoint,
                witness: Witness::new(),
                script_sig: ScriptBuf::new(),
                sequence: Sequence::from_height(Runestone::COMMIT_CONFIRMATIONS - 1),
            })
            .collect(),
        output: reveal_output,
    };

    for output in reveal_tx.output.iter() {
        if output.value < output.script_pubkey.dust_value().to_sat() {
            ic_cdk::trap("commit txn output would be dust")
        }
    }

    let mut sighash_cache = SighashCache::new(&mut reveal_tx);
    let mut signing_data = vec![];
    let leaf_hash = TapLeafHash::from_script(&reveal_script, LeafVersion::TapScript);
    sighash_cache
        .taproot_encode_signing_data_to(
            &mut signing_data,
            0,
            // &Prevouts::All(commit_tx.output.as_slice()),
            &Prevouts::All(&[commit_tx.output[vout].clone()]),
            None,
            Some((leaf_hash, 0xFFFFFFFF)),
            TapSighashType::Default,
        )
        .unwrap();
    let mut hashed_tag = sha256::Hash::hash(b"TapSighash").to_byte_array().to_vec();
    let mut prefix = hashed_tag.clone();
    prefix.append(&mut hashed_tag);

    let signing_data: Vec<_> = prefix.iter().chain(signing_data.iter()).cloned().collect();

    let schnorr_signature =
        schnorr_api::schnorr_sign(signing_data.clone(), derivation_path.clone()).await;
    ic_cdk::println!("sig size: {}", schnorr_signature.len());

    // Verify the signature to be sure that signing works
    let secp = bitcoin::secp256k1::Secp256k1::verification_only();

    let sig_ = schnorr::Signature::from_slice(&schnorr_signature).unwrap();
    let digest = sha256::Hash::hash(&signing_data).to_byte_array();
    let msg = Message::from_slice(&digest).unwrap();
    assert!(secp
        .verify_schnorr(&sig_, &msg, &schnorr_public_key)
        .is_ok());

    let witness = sighash_cache.witness_mut(0).unwrap();
    witness.push(
        Signature {
            sig: schnorr::Signature::from_slice(&schnorr_signature).unwrap(),
            hash_ty: TapSighashType::Default,
        }
        .to_vec(),
    );
    witness.push(reveal_script);
    witness.push(&control_block.serialize());
    if Runestone::decipher(&reveal_tx).unwrap() != Artifact::Runestone(runestone) {
        ic_cdk::trap("Runestone mismatched")
    }
    let commit_tx_bytes = consensus::serialize(&commit_tx);
    ic_cdk::println!("Signed commit txn: {}", hex::encode(commit_tx_bytes));
    let reveal_tx_bytes = consensus::serialize(&reveal_tx);
    ic_cdk::println!("Signed reveal txn: {}", hex::encode(reveal_tx_bytes));
    (commit_tx_address, commit_tx, reveal_tx)
}
