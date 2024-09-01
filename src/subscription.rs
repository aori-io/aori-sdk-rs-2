use std::env;

use reqwest::Client;
use serde_json::Value;

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
