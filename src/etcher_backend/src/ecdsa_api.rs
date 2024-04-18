use crate::STATE;

pub async fn get_ecdsa_public_key(derivation_path: Vec<Vec<u8>>) -> Vec<u8> {
    let key_id = STATE.with_borrow(|state| state.ecdsa_key.as_ref().unwrap().to_key_id());
    ic_cdk::api::management_canister::ecdsa::ecdsa_public_key(
        ic_cdk::api::management_canister::ecdsa::EcdsaPublicKeyArgument {
            canister_id: None,
            derivation_path,
            key_id,
        },
    )
    .await
    .unwrap()
    .0
    .public_key
}

pub async fn ecdsa_sign(message_hash: Vec<u8>, derivation_path: Vec<Vec<u8>>) -> Vec<u8> {
    let key_id = STATE.with_borrow(|state| state.ecdsa_key.as_ref().unwrap().to_key_id());
    ic_cdk::api::management_canister::ecdsa::sign_with_ecdsa(
        ic_cdk::api::management_canister::ecdsa::SignWithEcdsaArgument {
            message_hash,
            derivation_path,
            key_id,
        },
    )
    .await
    .unwrap()
    .0
    .signature
}
