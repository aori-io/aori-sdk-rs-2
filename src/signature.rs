use std::str::FromStr;

use alloy::{primitives::{Address, Signature, B256}, signers::{ local::PrivateKeySigner, SignerSync}};
use anyhow::Result;

use super::{get_order_hash, AoriOrder};

pub async fn sign_order(signer: &PrivateKeySigner, order: AoriOrder) -> Result<String> {
    let order_hash = get_order_hash(order.clone());
    let signature: Signature = signer.sign_hash_sync(&B256::from_slice(order_hash.as_slice()))?;
    Ok(format!("0x{}", hex::encode(signature.as_bytes())))
}

// pub async fn sign_order(order: AoriOrder, key: &str) -> Result<String> {
//     let signer = PrivateKeySigner::from_str(key).unwrap();

//     let order_hash = get_order_hash(order.clone());
//     let signature: Signature = signer.sign_hash_sync(&B256::from_slice(order_hash.as_slice()))?;
//     Ok(format!("0x{}", hex::encode(signature.as_bytes())))
// }

pub async fn get_order_signer(order: AoriOrder, signature: &str) -> Result<Address> {
    let order_hash = get_order_hash(order.clone());
    let signature = Signature::from_str(signature)?;
    let signer_address = signature.recover_address_from_prehash(&order_hash)?;
    Ok(signer_address)
}