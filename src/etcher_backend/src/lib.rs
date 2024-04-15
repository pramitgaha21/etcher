use candid::{CandidType, Principal};
use ckbtc_api::{RetrieveBtcStatusArgs, RetrieveBtcStatusV2};
use ic_cdk::{
    api::management_canister::bitcoin::BitcoinNetwork, export_candid, init, query, update,
};
use icrc_ledger_types::icrc1::account::Account;
use ordinals::Runestone;
use serde::Deserialize;
use state::STATE;
use types::CandidEtching;
use utils::{
    generate_derivation_path, generate_p2pkh_address, generate_subaccount, get_public_key,
    validate_caller,
};

use crate::{
    btc_api::{build_etching_transaction, get_utxos, sign_and_send_txn},
    ckbtc_api::{CkBTC, CkBTCMinter, RetrieveBtcArgs},
    state::EcdsaKeyIds,
    utils::always_fail,
};

pub mod btc_api;
pub mod ckbtc_api;
pub mod state;
pub mod types;
pub mod utils;

#[derive(CandidType, Deserialize)]
pub struct InitArgs {
    pub network: BitcoinNetwork,
    pub ckbtc_minter: Principal,
    pub ckbtc: Principal,
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

#[update]
pub async fn get_deposit_address_for_bitcoin() -> String {
    let caller = validate_caller();
    let (network, key_id) = STATE.with_borrow(|state| {
        (
            state.network.as_ref().unwrap().clone(),
            state.ecdsa_key.as_ref().unwrap().to_key_id(),
        )
    });
    let derivation_path = generate_derivation_path(&caller);
    let public_key = get_public_key(derivation_path, key_id).await;
    generate_p2pkh_address(network, &public_key)
}

#[query]
pub fn get_deposit_address_for_ckbtc() -> String {
    let caller = validate_caller();
    Account {
        owner: ic_cdk::id(),
        subaccount: Some(generate_subaccount(&caller)),
    }
    .to_string()
}

#[update]
pub async fn confirm_and_convert_deposit() -> (u64, RetrieveBtcStatusV2) {
    let caller = validate_caller();
    let (ckbtc_minter, ckbtc_ledger, network, key_id) = STATE.with_borrow(|state| {
        (
            CkBTCMinter::new(state.ckbtc_minter.as_ref().unwrap().clone()),
            CkBTC::new(state.ckbtc_ledger.as_ref().unwrap().clone()),
            state.network.as_ref().unwrap().clone(),
            state.ecdsa_key.as_ref().unwrap().to_key_id(),
        )
    });
    let deposit_address = Account {
        owner: ic_cdk::id(),
        subaccount: Some(generate_subaccount(&caller)),
    };
    // checking for the balance
    let balance = ckbtc_ledger.get_balance_of(deposit_address).await;
    let amount = 10000 + 2000 + 2558;
    if balance < amount {}
    let derivation_path = generate_derivation_path(&caller);
    let public_key = get_public_key(derivation_path, key_id).await;
    let address = generate_p2pkh_address(network.clone(), &public_key);
    let result = ckbtc_minter
        .retrieve_btc(RetrieveBtcArgs {
            address,
            amount: amount as u64,
        })
        .await;
    match result {
        Err(e) => {
            ic_cdk::println!("{:?}", e);
            let err_msg = format!("{:?}", e);
            ic_cdk::trap(&err_msg)
        }
        Ok(block_index) => (
            block_index.block_index,
            ckbtc_minter
                .retrieve_btc_status_v2(RetrieveBtcStatusArgs {
                    block_index: block_index.block_index,
                })
                .await,
        ),
    }
}

#[query(composite = true)]
pub async fn query_btc_retrieval_status(block_index: u64) -> RetrieveBtcStatusV2 {
    let ckbtc_minter =
        STATE.with_borrow(|state| CkBTCMinter::new(state.ckbtc_minter.as_ref().unwrap().clone()));
    ckbtc_minter
        .retrieve_btc_status_v2(RetrieveBtcStatusArgs { block_index })
        .await
}

#[update]
pub async fn query_btc_balance() -> u64 {
    let caller = validate_caller();
    let (network, key_id) = STATE.with_borrow(|state| {
        (
            state.network.as_ref().unwrap().clone(),
            state.ecdsa_key.as_ref().unwrap().to_key_id(),
        )
    });
    let derivation_path = generate_derivation_path(&caller);
    let public_key = get_public_key(derivation_path, key_id).await;
    let address = generate_p2pkh_address(network.clone(), &public_key);
    btc_api::get_balance_of(network, &address).await
}

#[update]
pub async fn etch_rune(arg: CandidEtching) -> Vec<u8> {
    let caller = validate_caller();
    let (network, key_id) = STATE.with_borrow(|state| {
        (
            state.network.as_ref().unwrap().clone(),
            state.ecdsa_key.as_ref().unwrap().to_key_id(),
        )
    });
    let derivation_path = generate_derivation_path(&caller);
    let public_key = get_public_key(derivation_path.clone(), key_id.clone()).await;
    let address = generate_p2pkh_address(network.clone(), &public_key);
    // checking for the balance
    let balance = btc_api::get_balance_of(network, &address).await;
    if balance < 10000 {
        ic_cdk::trap("Not Enough Balance")
    }
    let runestone: Runestone = arg.into();
    let own_utxos = get_utxos(network.clone(), address.clone()).await.utxos;
    let txn = build_etching_transaction(network.clone(), address.clone(), &own_utxos, runestone);
    let txid: [u8; 32] =
        *sign_and_send_txn(network, &public_key, address, txn, key_id, derivation_path)
            .await
            .as_ref();
    txid.to_vec()
}

export_candid!();
