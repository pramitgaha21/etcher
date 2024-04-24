use crate::STATE;
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Deserialize, Serialize, Debug, Clone)]
pub enum SchnorrAlgorithm {
    #[serde(rename = "bip340secp256k1")]
    Bip340Secp256k1,
    #[serde(rename = "ed25519")]
    Ed25519,
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone)]
pub struct SchnorrKeyId {
    pub algorithm: SchnorrAlgorithm,
    pub name: String,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
struct SchnorrPublicKey {
    pub canister_id: Option<Principal>,
    pub derivation_path: Vec<Vec<u8>>,
    pub key_id: SchnorrKeyId,
}

#[derive(CandidType, Deserialize, Debug)]
struct SchnorrPublicKeyReply {
    pub public_key: Vec<u8>,
    pub chain_code: Vec<u8>,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
struct SignWithSchnorr {
    pub message: Vec<u8>,
    pub derivation_path: Vec<Vec<u8>>,
    pub key_id: SchnorrKeyId,
}

#[derive(CandidType, Deserialize, Debug)]
struct SignWithSchnorrReply {
    pub signature: Vec<u8>,
}

pub async fn get_schnorr_public_key(derivation_path: Vec<Vec<u8>>) -> Vec<u8> {
    let (schnorr_canister, key_id) = STATE.with_borrow(|state| {
        (
            *state.schnorr_canister.as_ref().unwrap(),
            state.schnorr_key.as_ref().unwrap().clone(),
        )
    });
    ic_cdk::call::<(SchnorrPublicKey,), (SchnorrPublicKeyReply,)>(
        schnorr_canister,
        "schnorr_public_key",
        (SchnorrPublicKey {
            canister_id: None,
            derivation_path,
            key_id,
        },),
    )
    .await
    .unwrap()
    .0
    .public_key
}

pub async fn schnorr_sign(message: Vec<u8>, derivation_path: Vec<Vec<u8>>) -> Vec<u8> {
    let (schnorr_canister, key_id) = STATE.with_borrow(|state| {
        (
            *state.schnorr_canister.as_ref().unwrap(),
            state.schnorr_key.as_ref().unwrap().clone(),
        )
    });
    ic_cdk::call::<(SignWithSchnorr,), (SignWithSchnorrReply,)>(
        schnorr_canister,
        "sign_with_schnorr",
        (SignWithSchnorr {
            message,
            derivation_path,
            key_id,
        },),
    )
    .await
    .unwrap()
    .0
    .signature
}
