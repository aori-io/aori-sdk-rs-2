use std::env;

use alloy::primitives::{Address, U256};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{DetailsToExecute, SettledMatch, QuoteReceivedData, CalldataToExecuteData, QuoteRequestedData};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SubscriptionEvents {
    QuoteRequested,
    QuoteReceived,
    CalldataToExecute,
    TradeSettled,
}

impl ToString for SubscriptionEvents {
    fn to_string(&self) -> String {
        match self {
            SubscriptionEvents::QuoteRequested => "QuoteRequested",
            SubscriptionEvents::QuoteReceived => "QuoteReceived",
            SubscriptionEvents::CalldataToExecute => "CalldataToExecute",
            SubscriptionEvents::TradeSettled => "TradeSettled",
        }
        .to_string()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EventData {
    QuoteRequested(QuoteReceivedData),
    QuoteReceived(QuoteReceivedData),
    CalldataToExecute(CalldataToExecuteData),
    TradeSettled(SettledMatch),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionEventData {
    #[serde(rename = "rfqId")]
    pub rfq_id: String,
    pub r#type: SubscriptionEvents,
    pub data: EventData
}

pub async fn broadcast_subscription_event(event: SubscriptionEvents, data: serde_json::Value) {
    let data = serde_json::json!({
        "id": 1,
        "jsonrpc": "2.0",
        "method": "aori_broadcast",
        "params": vec![serde_json::json!({
            "secret": "",
            "data": serde_json::json!({
                "type": event.to_string(),
                "data": data
            })
        })]
    });

    let client = Client::new();
    let _ = client
        .post(env::var("AORI_BROADCAST_URL").expect("missing AORI_BROADCAST_URL"))
        .json::<Value>(&data)
        .send()
        .await;
}

