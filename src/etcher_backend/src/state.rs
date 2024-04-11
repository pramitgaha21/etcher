use std::{cell::RefCell, collections::HashMap};

use candid::{CandidType, Nat, Principal};
use ic_cdk::api::management_canister::{
    bitcoin::BitcoinNetwork,
    ecdsa::{EcdsaCurve, EcdsaKeyId},
};
use icrc_ledger_types::icrc1::account::{Account, Subaccount};
use serde::{Deserialize, Serialize};
use tiny_keccak::{Hasher, Sha3};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub user_principal: Principal,
    pub registered_at: u64,
    pub allocated_subaccount: Subaccount,
    pub ckbtc_balance: Nat,
    pub btc_balance: u64,
    pub p2pkh_address: String,
}

#[derive(CandidType)]
pub struct UserQuery {
    pub user_principal: Principal,
    pub registered_at: u64,
    pub encoded_icrc1_account: String,
    pub ckbtc_balance: Nat,
    pub btc_balance: u64,
    pub p2pkh_address: String,
}

impl User {
    pub fn generate_derivation_path(&self) -> Vec<Vec<u8>> {
        let mut hash = [0u8; 32];
        let mut hasher = Sha3::v256();
        hasher.update(self.user_principal.as_slice());
        hasher.update(self.registered_at.to_le_bytes().as_ref());
        hasher.finalize(&mut hash);
        vec![hash.to_vec()]
    }

    pub fn into_query(&self) -> UserQuery {
        UserQuery {
            user_principal: self.user_principal.clone(),
            registered_at: self.registered_at,
            encoded_icrc1_account: Account {
                owner: ic_cdk::id(),
                subaccount: Some(self.allocated_subaccount.clone()),
            }
            .to_string(),
            btc_balance: self.btc_balance,
            ckbtc_balance: self.ckbtc_balance.clone(),
            p2pkh_address: self.p2pkh_address.clone(),
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
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

#[derive(Serialize, Deserialize, Default)]
pub struct State {
    pub auth: Option<Principal>,
    pub network: Option<BitcoinNetwork>,
    pub ecdsa_key: Option<EcdsaKeyIds>,
    pub etch_fee: Option<u64>,
    pub users: HashMap<Principal, User>,
    pub runes_count: u128,
    pub runes: HashMap<u128, (String,)>,
}

thread_local! {
    pub static STATE: RefCell<State> = RefCell::default();
}
