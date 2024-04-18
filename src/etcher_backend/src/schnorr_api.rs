use bip32::{Seed, XPrv};
use bitcoin::{
    key::{Secp256k1, UntweakedKeypair},
    secp256k1::Message,
};
use bitcoin_hashes::{sha256, Hash};
use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use ic_crypto_extended_bip32::{DerivationIndex, DerivationPath};
use serde::Serialize;
use serde_bytes::ByteBuf;

const MAX_VALUE_SIZE: u32 = 100;

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct SchnorrPublicKeyArgs {
    pub canister_id: Option<Principal>,
    pub derivation_path: Vec<ByteBuf>,
    pub key_id: SchnorrKeyId,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct SchnorrPublicKeyResult {
    pub public_key: ByteBuf,
    pub chain_code: ByteBuf,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct SignWithSchnorrArgs {
    pub message: ByteBuf,
    pub derivation_path: Vec<ByteBuf>,
    pub key_id: SchnorrKeyId,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct SignWithSchnorrResult {
    pub signature: ByteBuf,
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SchnorrAlgorithm {
    #[serde(rename = "bip340secp256k1")]
    Bip340Secp256k1,
    #[serde(rename = "ed25519")]
    Ed25519,
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SchnorrKeyId {
    algorithm: SchnorrAlgorithm,
    name: String,
}
