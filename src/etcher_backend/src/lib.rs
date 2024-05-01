#![warn(missing_debug_implementations)]

use std::{cell::RefCell, collections::HashMap, time::Duration};

use bitcoin::Transaction;
use btc_api::check_etching;
use candid::{CandidType, Principal};
use ckbtc_api::{CkBTC, CkBTCMinter, RetrieveBtcStatusV2};
use hex::ToHex;
use ic_cdk::{
    api::management_canister::{
        bitcoin::BitcoinNetwork,
        ecdsa::{EcdsaCurve, EcdsaKeyId},
    },
    init, query, update,
};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    writer::Writer,
    DefaultMemoryImpl, Memory as _,
};
use icrc_ledger_types::icrc1::account::Account;
use ordinals::Runestone;
use schnorr_api::SchnorrKeyId;
use serde::{Deserialize, Serialize};
use slotmap::{Key, KeyData};
use utils::generate_subaccount;

use crate::{
    btc_api::build_and_sign_etching_transaction,
    ckbtc_api::RetrieveBtcArgs,
    ecdsa_api::get_ecdsa_public_key,
    schnorr_api::get_schnorr_public_key,
    utils::{always_fail, generate_derivation_path, public_key_to_p2pkh_address, validate_caller},
};

pub mod btc_api;
pub mod ckbtc_api;
pub mod ecdsa_api;
pub mod schnorr_api;
pub mod tags;
pub mod utils;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum EcdsaKeyIds {
    TestKey1,
    ProductionKey,
    TestKeyLocalDevelopment,
}

impl EcdsaKeyIds {
    pub fn to_key_id(&self) -> EcdsaKeyId {
        EcdsaKeyId {
            curve: EcdsaCurve::Secp256k1,
            name: match self {
                Self::TestKey1 => "test_key_1".into(),
                Self::ProductionKey => "key_1".into(),
                Self::TestKeyLocalDevelopment => "dfx_test_key".into(),
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueuedRevealTxn {
    pub reveal_txn: Transaction,
    pub timer_id: KeyData,
    pub commit_tx_address: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct State {
    pub ckbtc_ledger: Option<Principal>,
    pub ckbtc_minter: Option<Principal>,
    pub network: Option<BitcoinNetwork>,
    pub ecdsa_key: Option<EcdsaKeyIds>,
    pub schnorr_key: Option<SchnorrKeyId>,
    pub schnorr_canister: Option<Principal>,
    pub queue_count: u128,
    pub timer_for_reveal_txn: u32,
    pub reveal_txn_in_queue: HashMap<u128, QueuedRevealTxn>,
}

pub type Memory = VirtualMemory<DefaultMemoryImpl>;

pub fn get_upgrade_memory() -> Memory {
    MEMORY_MANAGER.with_borrow(|memory| memory.get(MemoryId::new(0)))
}

thread_local! {
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
    pub static STATE: RefCell<State> = RefCell::default();
}

#[derive(CandidType, Deserialize, Debug)]
pub struct InitArgs {
    pub ckbtc_ledger: Principal,
    pub ckbtc_minter: Principal,
    pub network: BitcoinNetwork,
    pub schnorr_canister: Principal,
    pub timer_for_reveal_txn: u32, // should be provided as mins
}

#[init]
pub fn init(arg: InitArgs) {
    getrandom::register_custom_getrandom!(always_fail);
    let (ecdsa_key_id, schnorr_key) = match arg.network {
        BitcoinNetwork::Mainnet => (
            EcdsaKeyIds::ProductionKey,
            SchnorrKeyId {
                name: "test_key_1".to_string(),
                algorithm: schnorr_api::SchnorrAlgorithm::Bip340Secp256k1,
            },
        ),
        BitcoinNetwork::Regtest => (
            EcdsaKeyIds::TestKeyLocalDevelopment,
            SchnorrKeyId {
                name: "dfx_test_key".to_string(),
                algorithm: schnorr_api::SchnorrAlgorithm::Bip340Secp256k1,
            },
        ),
        BitcoinNetwork::Testnet => (
            EcdsaKeyIds::TestKey1,
            SchnorrKeyId {
                name: "test_key_1".to_string(),
                algorithm: schnorr_api::SchnorrAlgorithm::Bip340Secp256k1,
            },
        ),
    };
    STATE.with_borrow_mut(|state| {
        state.network = Some(arg.network);
        state.ckbtc_minter = Some(arg.ckbtc_minter);
        state.ckbtc_ledger = Some(arg.ckbtc_ledger);
        state.ecdsa_key = Some(ecdsa_key_id);
        state.schnorr_canister = Some(arg.schnorr_canister);
        state.schnorr_key = Some(schnorr_key);
        state.timer_for_reveal_txn = arg.timer_for_reveal_txn;
    })
}

#[update]
pub async fn get_deposit_address_for_bitcoin() -> String {
    let caller = validate_caller();
    let derivation_path = generate_derivation_path(&caller);
    let ecdsa_public_key = get_ecdsa_public_key(derivation_path).await;
    public_key_to_p2pkh_address(&ecdsa_public_key)
}

#[update]
pub async fn get_btc_balance() -> u64 {
    let caller = validate_caller();
    let derivation_path = generate_derivation_path(&caller);
    let ecdsa_public_key = get_ecdsa_public_key(derivation_path).await;
    let address = public_key_to_p2pkh_address(&ecdsa_public_key);
    btc_api::get_balance_of(address).await
}

#[query]
pub fn get_deposit_address_for_ckbtc() -> String {
    let caller = validate_caller();
    let subaccount = generate_subaccount(&caller);
    Account {
        owner: ic_cdk::id(),
        subaccount: Some(subaccount),
    }
    .to_string()
}

#[update]
pub async fn confirm_and_convert_ckbtc() -> u64 {
    let caller = validate_caller();
    let subaccount = generate_subaccount(&caller);
    let account = Account {
        owner: ic_cdk::id(),
        subaccount: Some(subaccount),
    };
    let (ckbtc_minter, ckbtc_ledger) = STATE.with_borrow(|state| {
        let ckbtc_minter = *state.ckbtc_minter.as_ref().unwrap();
        let ckbtc_ledger = *state.ckbtc_ledger.as_ref().unwrap();
        (CkBTCMinter::new(ckbtc_minter), CkBTC::new(ckbtc_ledger))
    });
    let balance = ckbtc_ledger.get_balance_of(account).await;
    if balance < 20000 {
        ic_cdk::trap("Not enough balance")
    }
    let derivation_path = generate_derivation_path(&caller);
    let ecdsa_public_key = get_ecdsa_public_key(derivation_path.clone()).await;
    let caller_p2pkh_address = public_key_to_p2pkh_address(&ecdsa_public_key);
    let result = ckbtc_minter
        .retrieve_btc(RetrieveBtcArgs {
            address: caller_p2pkh_address,
            amount: balance as u64,
        })
        .await;
    match result {
        Err(e) => {
            let err = format!("{:?}", e);
            ic_cdk::trap(&err)
        }
        Ok(ok) => ok.block_index,
    }
}

#[query(composite = true)]
pub async fn query_converstion_status(block_index: u64) -> RetrieveBtcStatusV2 {
    let ckbtc_minter = STATE.with_borrow(|state| {
        let canister_id = *state.ckbtc_minter.as_ref().unwrap();
        CkBTCMinter::new(canister_id)
    });
    ckbtc_minter
        .retrieve_btc_status_v2(ckbtc_api::RetrieveBtcStatusArgs { block_index })
        .await
}

#[derive(CandidType, Deserialize, Debug)]
pub struct EtchingArgs {
    pub divisibility: u8,
    pub symbol: u32,
    pub rune: String,
    pub amount: u128,
    pub cap: u128,
    pub turbo: bool,
    pub premine: Option<u128>,
    pub height_start: u64,
    pub height_stop: u64,
    pub offset_start: u64,
    pub offset_stop: u64,
    pub fee_rate: Option<u64>,
}

#[update]
pub async fn etch_rune(mut args: EtchingArgs) -> (String, String) {
    let caller = validate_caller();
    args.rune = args.rune.to_ascii_uppercase();
    let derivation_path = generate_derivation_path(&caller);
    let ecdsa_public_key = get_ecdsa_public_key(derivation_path.clone()).await;
    let schnorr_public_key = get_schnorr_public_key(derivation_path.clone()).await;
    let caller_p2pkh_address = public_key_to_p2pkh_address(&ecdsa_public_key);
    let balance = btc_api::get_balance_of(caller_p2pkh_address.clone()).await;
    if balance < 10_000 {
        ic_cdk::trap("Not enough balance")
    }
    let utxos_response = btc_api::get_utxos_of(caller_p2pkh_address.clone()).await;
    check_etching(utxos_response.tip_height, &args);
    let (commit_tx_address, commit_tx, reveal_tx) = build_and_sign_etching_transaction(
        &derivation_path,
        &utxos_response.utxos,
        &ecdsa_public_key,
        &schnorr_public_key,
        caller_p2pkh_address,
        args,
    )
    .await;
    let commit_txid = btc_api::send_bitcoin_transaction(commit_tx).await;
    let id = STATE.with_borrow_mut(|state| {
        let id = state.queue_count;
        state.queue_count += 1;
        id
    });
    let time = STATE.with_borrow(|state| state.timer_for_reveal_txn as u64 * 60);
    let timer_id = ic_cdk_timers::set_timer_interval(Duration::from_secs(time), move || {
        ic_cdk::spawn(confirm_min_commitment_and_send_reveal_txn(id))
    });
    let queue_txn = QueuedRevealTxn {
        commit_tx_address: commit_tx_address.to_string(),
        reveal_txn: reveal_tx.clone(),
        timer_id: timer_id.data(),
    };
    STATE.with_borrow_mut(|state| state.reveal_txn_in_queue.insert(id, queue_txn));
    (commit_txid, reveal_tx.txid().encode_hex())
}

pub async fn confirm_min_commitment_and_send_reveal_txn(id: u128) {
    let reveal_txn = STATE.with_borrow(|state| state.reveal_txn_in_queue.get(&id).unwrap().clone());
    let utxos_response = btc_api::get_utxos_of(reveal_txn.commit_tx_address).await;
    let utxos = utxos_response.utxos;
    if utxos.is_empty() {
        ic_cdk::trap("No UTXOs Found")
    }
    if utxos_response.tip_height - utxos[0].height < Runestone::COMMIT_CONFIRMATIONS as u32 - 1 {
        ic_cdk::trap("Not enough commit confirmation")
    }
    btc_api::send_bitcoin_transaction(reveal_txn.reveal_txn).await;
    ic_cdk_timers::clear_timer(reveal_txn.timer_id.into());
    STATE.with_borrow_mut(|state| state.reveal_txn_in_queue.remove(&id));
}

ic_cdk::export_candid!();
