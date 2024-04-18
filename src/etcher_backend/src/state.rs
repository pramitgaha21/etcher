use std::cell::RefCell;

use candid::{CandidType, Principal};
use ic_cdk::api::management_canister::{
    bitcoin::BitcoinNetwork,
    ecdsa::{EcdsaCurve, EcdsaKeyId},
};
use serde::{Deserialize, Serialize};

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

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum SchnorrKeyIds {
    DfxTestKey,
    TestKey1,
    DfxTestKeyEd25519,
    TestKey1Ed25519,
}

impl SchnorrKeyIds {
    pub fn to_key_id(&self) -> SchnorrKeyId {
        match self {
            Self::DfxTestKey => SchnorrKeyId {
                algorithm: SchnorrAlgorithm::Bip340Secp256k1,
                name: "dfx_test_key".to_string(),
            },
            Self::TestKey1 => SchnorrKeyId {
                algorithm: SchnorrAlgorithm::Bip340Secp256k1,
                name: "test_key_1".to_string(),
            },
            Self::DfxTestKeyEd25519 => SchnorrKeyId {
                algorithm: SchnorrAlgorithm::Ed25519,
                name: "dfx_test_key".to_string(),
            },
            Self::TestKey1Ed25519 => SchnorrKeyId {
                algorithm: SchnorrAlgorithm::Ed25519,
                name: "test_key_1".to_string(),
            },
        }
    }

    fn variants() -> Vec<SchnorrKeyIds> {
        vec![
            SchnorrKeyIds::DfxTestKey,
            SchnorrKeyIds::TestKey1,
            SchnorrKeyIds::DfxTestKeyEd25519,
            SchnorrKeyIds::TestKey1Ed25519,
        ]
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct State {
    pub auth: Option<Principal>,
    pub network: Option<BitcoinNetwork>,
    pub ecdsa_key: Option<EcdsaKeyIds>,
    pub ckbtc_ledger: Option<Principal>,
    pub ckbtc_minter: Option<Principal>,
}

thread_local! {
    pub static STATE: RefCell<State> = RefCell::default();
}
