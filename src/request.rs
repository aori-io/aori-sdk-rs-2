use serde::{Deserialize, Serialize};

use crate::{AoriOrder, AoriV2::Order};

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct AoriPingParams(String);

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AoriRfqPartialRequestParams {
    pub address: String,
    pub input_token: String,
    pub output_token: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "inputAmount")]
    pub input_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "outputAmount")]
    pub output_amount: Option<String>,
    pub zone: Option<String>,
    #[serde(rename = "chainId")]
    pub chain_id: i64,
    pub deadline: Option<i64>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AoriRfqFullRequestParams {
    pub order: AoriOrder,
    pub signature: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "seatId")]
    pub seat_id: Option<i64>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum AoriRfqParams {
    Partial(AoriRfqPartialRequestParams),
    Full(AoriRfqFullRequestParams),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AoriRespondParams {
    #[serde(rename = "rfqId")]
    pub rfq_id: String,
    pub order: AoriOrder,
    pub signature: String,
}

///
///  Response Types
///

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AoriPingRequest {
    pub id: i64,
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AoriRfqPartialRequest {
    pub id: i64,
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<AoriRfqPartialRequestParams>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AoriRfqFullRequest {
    pub id: i64,
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<AoriRfqFullRequestParams>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AoriRespondRequest {
    pub id: i64,
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<AoriRespondParams>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AoriRequest {
    Ping(AoriPingRequest),
    Partial(AoriRfqPartialRequest),
    Full(AoriRfqFullRequest),
    Respond(AoriRespondRequest),
}

// TODO: rename from AoriRequestin to AoriGeneralRequest
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AoriRequestin {
    pub id: i64,
    pub jsonrpc: String,
    pub method: String,
    pub params: serde_json::Value,
}
