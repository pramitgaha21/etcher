#![warn(missing_debug_implementations)]

use std::cell::RefCell;

use candid::{CandidType, Principal};
use ckbtc_api::{CkBTC, CkBTCMinter, RetrieveBtcStatusV2};
use ic_cdk::{
    api::management_canister::{
        bitcoin::BitcoinNetwork,
        ecdsa::{EcdsaCurve, EcdsaKeyId},
    },
    init, query, update,
};
use icrc_ledger_types::icrc1::account::Account;
use schnorr_api::SchnorrKeyId;
use serde::Deserialize;
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
pub mod utils;

#[derive(CandidType, Deserialize, Debug)]
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

#[derive(Default, Debug)]
pub struct State {
    pub ckbtc_ledger: Option<Principal>,
    pub ckbtc_minter: Option<Principal>,
    pub network: Option<BitcoinNetwork>,
    pub ecdsa_key: Option<EcdsaKeyIds>,
    pub schnorr_key: Option<SchnorrKeyId>,
    pub schnorr_canister: Option<Principal>,
}

thread_local! {
    pub static STATE: RefCell<State> = RefCell::default();
}

#[derive(CandidType, Deserialize, Debug)]
pub struct InitArgs {
    pub ckbtc_ledger: Principal,
    pub ckbtc_minter: Principal,
    pub network: BitcoinNetwork,
    pub schnorr_canister: Principal,
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
    pub symbol: Option<u32>,
    pub rune: String,
    pub amount: Option<u128>,
    pub cap: Option<u128>,
    pub turbo: bool,
}

#[update]
pub async fn etch_rune(args: EtchingArgs) -> (String, String) {
    let caller = validate_caller();
    let derivation_path = generate_derivation_path(&caller);
    let ecdsa_public_key = get_ecdsa_public_key(derivation_path.clone()).await;
    let schnorr_public_key = get_schnorr_public_key(derivation_path.clone()).await;
    let caller_p2pkh_address = public_key_to_p2pkh_address(&ecdsa_public_key);
    let balance = btc_api::get_balance_of(caller_p2pkh_address.clone()).await;
    if balance < 10_000 {
        ic_cdk::trap("Not enough balance")
    }
    let owned_utxos = btc_api::get_utxos_of(caller_p2pkh_address.clone()).await;
    let (commit_tx, reveal_tx) = build_and_sign_etching_transaction(
        &derivation_path,
        &owned_utxos,
        &ecdsa_public_key,
        &schnorr_public_key,
        caller_p2pkh_address,
        args,
    )
    .await;
    let commit_txid = btc_api::send_bitcoin_transaction(commit_tx).await;
    let reveal_txid = btc_api::send_bitcoin_transaction(reveal_tx).await;
    (commit_txid, reveal_txid)
}

ic_cdk::export_candid!();
