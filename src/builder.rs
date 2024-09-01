// request builder
// used for signing orders and stuff

use std::str::FromStr;

use alloy::{primitives::{Address, B256, U256}, signers::{local::PrivateKeySigner, Signature, SignerSync}};

use crate::{create_limit_order, request::*, sign_order, AoriOrder};

use super::get_order_hash;

pub struct AoriRequestBuilder {
    signer: PrivateKeySigner,
}

impl AoriRequestBuilder {
    /// Wraps around a Private Key / Wallet to sign off on trades
    pub fn new(pkey_str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let wallet  = pkey_str.parse().unwrap();
        Ok(AoriRequestBuilder { signer: wallet })
    }

    /// Builds a partial RFQ
    pub async fn build_partial_rfq(
        &self,
        address: Option<Address>,
        input_token: String,
        output_token: String,
        input_amount: String,
        output_amount: Option<String>,
        chain_id: u64,
    ) -> Result<AoriRfqParams, Box<dyn std::error::Error>> {
        Ok(AoriRfqParams::Partial(AoriRfqPartialRequestParams {
            address: address.unwrap_or(self.signer.address()).to_string(),
            input_token,
            output_token,
            input_amount: input_amount,
            output_amount,
            chain_id,
            zone: None,
            deadline: None,
        }))
    }

    /// Builds a full RFQ
    pub async fn build_full_rfq(
        &self,
        address: Option<Address>,
        input_token: String,
        output_token: String,
        input_amount: String,
        output_amount: String,
        chain_id: u64,
        seat_id: Option<u64>,
    ) -> Result<AoriRfqParams, Box<dyn std::error::Error>> {

        let order = create_limit_order(
            address.unwrap_or(self.signer.address()),
            Address::from_str(&input_token).unwrap(),
            Address::from_str(&output_token).unwrap(),
            U256::from_str(&input_amount).unwrap(),
            U256::from_str(&output_amount).unwrap(),
            chain_id
        );

        let signature = sign_order(&self.signer, order.clone()).await.unwrap();

        Ok(AoriRfqParams::Full(AoriRfqFullRequestParams {
            order,
            signature,
            seat_id: Some(seat_id.unwrap_or(0)),
        }))
    }

    pub async fn respond(
        &self,
        rfq_id: String,
        order: AoriOrder,
    ) -> Result<AoriRespondParams, Box<dyn std::error::Error>> {

        let order_hash = get_order_hash(order.clone());
        let signature: Signature = self.signer.sign_hash_sync(&B256::from_slice(order_hash.as_slice()))?;

        Ok(AoriRespondParams {
            rfq_id,
            order,
            signature: format!("0x{:?}", signature.as_bytes()).into(),
        })
    }
}
