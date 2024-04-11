use candid::{CandidType, Nat};
use ic_cdk::{
    api::management_canister::{bitcoin::BitcoinNetwork, ecdsa::EcdsaPublicKeyArgument},
    export_candid, init, query, update,
};
use serde::Deserialize;
use state::{User, UserQuery, STATE};
use types::DepositType;
use utils::{generate_subaccount, public_key_to_p2pkh_address, validate_caller};

use crate::{state::EcdsaKeyIds, utils::always_fail};

pub mod btc_api;
pub mod ckbtc_api;
pub mod state;
pub mod types;
pub mod utils;

#[derive(CandidType, Deserialize)]
pub struct InitArgs {
    pub network: BitcoinNetwork,
}

#[init]
pub fn init(arg: InitArgs) {
    let caller = ic_cdk::caller();
    getrandom::register_custom_getrandom!(always_fail);
    let key_id = match arg.network {
        BitcoinNetwork::Mainnet => EcdsaKeyIds::ProductionKey,
        BitcoinNetwork::Regtest => EcdsaKeyIds::TestKeyLocalDevelopment,
        BitcoinNetwork::Testnet => EcdsaKeyIds::TestKey1,
    };
    STATE.with_borrow_mut(|state| {
        state.network = Some(arg.network);
        state.ecdsa_key = Some(key_id);
        state.auth = Some(caller);
    });
}

pub fn pre_upgrade() {}

pub fn post_upgrade() {}

pub fn update_commission_receiver() {}

pub fn update_commission() {}

pub fn commission_detail() -> () {}

#[update]
pub async fn register() -> UserQuery {
    let caller = validate_caller();
    STATE.with_borrow(|state| {
        if state.users.contains_key(&caller) {
            ic_cdk::trap("User already registered")
        }
    });
    let current_time = ic_cdk::api::time();
    let allocated_subaccount = generate_subaccount(&caller, &current_time);
    let (network, key_id) = STATE.with_borrow(|state| {
        let network = state.network.as_ref().unwrap().clone();
        let key_id = state.ecdsa_key.as_ref().unwrap().to_key_id();
        (network, key_id)
    });
    let mut user = User {
        registered_at: current_time,
        user_principal: caller,
        allocated_subaccount,
        p2pkh_address: "".to_string(),
        btc_balance: 0,
        ckbtc_balance: Nat::from(0u128),
    };
    let derivation_path = user.generate_derivation_path();
    let public_key =
        ic_cdk::api::management_canister::ecdsa::ecdsa_public_key(EcdsaPublicKeyArgument {
            canister_id: None,
            derivation_path: derivation_path.clone(),
            key_id,
        })
        .await
        .unwrap()
        .0
        .public_key;
    let p2pkh_address = public_key_to_p2pkh_address(network, &public_key);
    user.p2pkh_address = p2pkh_address;
    STATE.with_borrow_mut(|state| {
        state.users.insert(caller, user.clone());
    });
    user.into_query()
}

#[query]
pub fn login() -> UserQuery {
    let caller = validate_caller();
    STATE.with_borrow(|state| match state.users.get(&caller) {
        None => ic_cdk::trap("User not Registered"),
        Some(user) => user.into_query(),
    })
}

pub async fn deposit(deposit_type: DepositType) -> UserQuery {
    let caller = validate_caller();
    let (mut user, network) = STATE.with_borrow(|state| match state.users.get(&caller) {
        None => ic_cdk::trap("User not Registered"),
        Some(user) => (user.clone(), state.network.as_ref().unwrap().clone()),
    });
    match deposit_type {
        DepositType::CKBTC => {}
        DepositType::NativeBTC => {
            let balance = btc_api::get_balance_of(network, user.p2pkh_address.as_ref()).await;
            user.btc_balance = balance;
        }
    }
    STATE.with_borrow_mut(|state| {
        state.users.insert(caller, user.clone());
        user.into_query()
    })
}

pub async fn etch_rune() {
    let caller = validate_caller();
}

pub fn list_runes() -> () {}

export_candid!();
