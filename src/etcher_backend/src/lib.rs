#![warn(missing_debug_implementations)]

use std::cell::RefCell;

use candid::{CandidType, Principal};
use ic_cdk::{
    api::management_canister::{
        bitcoin::BitcoinNetwork,
        ecdsa::{EcdsaCurve, EcdsaKeyId},
    },
    init, update,
};
use schnorr_api::SchnorrKeyId;
use serde::Deserialize;
use utils::generate_subaccount;

use crate::{
    ecdsa_api::get_ecdsa_public_key,
    schnorr_api::get_schnorr_public_key,
    utils::{generate_derivation_path, public_key_to_p2pkh_address, validate_caller},
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
    pub ecdsa_key: EcdsaKeyIds,
}

#[init]
pub fn init(arg: InitArgs) {
    STATE.with_borrow_mut(|state| {
        state.network = Some(arg.network);
        state.ckbtc_minter = Some(arg.ckbtc_minter);
        state.ckbtc_ledger = Some(arg.ckbtc_ledger);
        state.ecdsa_key = Some(arg.ecdsa_key);
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

pub fn get_deposit_address_for_ckbtc() -> String {
    let caller = validate_caller();
    let subaccount = generate_subaccount(&caller);
    todo!()
}

pub async fn confirm_and_convert_ckbtc() {
    let caller = validate_caller();
    let subaccount = generate_subaccount(&caller);
    // TODO
    todo!()
}

pub async fn query_converstion_status() {}

#[derive(CandidType, Deserialize, Debug)]
pub struct EtchingArgs {
    pub divisibility: u8,
    pub symbol: Option<u32>,
    pub rune: String,
}

pub async fn etch_rune(args: EtchingArgs) -> (String, String) {
    let caller = validate_caller();
    let derivation_path = generate_derivation_path(&caller);
    let ecdsa_public_key = get_ecdsa_public_key(derivation_path.clone()).await;
    let schnorr_public_key = get_schnorr_public_key(derivation_path.clone()).await;
    let caller_p2pkh_address = public_key_to_p2pkh_address(&ecdsa_public_key);
    let balance = btc_api::get_balance_of(caller_p2pkh_address.clone()).await;
    todo!()
}
