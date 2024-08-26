// request builder
// used for signing orders and stuff

use alloy::{primitives::{Address, B256}, signers::{local::PrivateKeySigner, Signature, SignerSync}};

use crate::{request::*, sign_order, AoriOrder};

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
        input_amount: Option<String>,
        output_amount: Option<String>,
        chain_id: i64,
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
        input_amount: Option<String>,
        output_amount: Option<String>,
        chain_id: i64,
    ) -> Result<AoriRfqParams, Box<dyn std::error::Error>> {
        Ok(AoriRfqParams::Full(AoriRfqFullRequestParams {

            // TODO: make an order order from 
            order: AoriOrder::default(),
            signature: "".to_string(),
            seat_id: None,
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
