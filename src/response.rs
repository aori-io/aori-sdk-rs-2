use crate::{shared_types::AoriOrder, DetailsToExecute, SettledMatch};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriPingResponse {
    pub id: i64,
    pub result: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct QuoteRequestedData {
    pub rfq_id: String,
    pub address: String,
    pub input_token: String,
    pub output_token: String,
    pub input_amount: String,
    pub zone: String,
    pub chain_id: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QuoteReceivedData {
    pub rfq_id: String,
    pub input_token: String,
    pub output_token: String,
    pub input_amount: String,
    pub output_amount: String,
    pub zone: String,
    pub chain_id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AoriErrorData {
    pub code: i64,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AoriErrorResponse {
    pub id: i64,
    pub error: AoriErrorData,
}

////////////////////////////////////////////////////////////////
//                          EVENTS
////////////////////////////////////////////////////////////////

#[derive(Debug, Deserialize, Clone)]
pub struct AoriFeedEventWrapper {
    pub id: Option<String>,
    pub result: AoriFeedEvents,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", content = "data")]
pub enum AoriFeedEvents {
    QuoteRequested(Box<QuoteRequestedData>),
    QuoteReceived(Box<QuoteReceivedData>),
    CalldataToExecute(Box<CalldataToExecuteData>),
    OrderFulfilled(Box<SettledMatch>)
}


#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MatchingOrder {
    pub maker_order: AoriOrder,
    pub taker_order: AoriOrder,
    pub maker_signature: String,
    pub taker_signature: String,
    pub block_deadline: u64,
    pub seat_number: u64,
    pub seat_holder: String,
    pub seat_percent_of_fees: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalldataToExecuteData {
    pub rfq_id: String,
    pub data: DetailsToExecute,
}


pub fn deserialize_aori_feed_event(json_data: &str) -> Result<AoriFeedEvents, serde_json::Error> {
    let wrapper = serde_json::from_str::<AoriFeedEventWrapper>(json_data)?;
    Ok(wrapper.result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_aori_feed_event() {
        let json_data = r#"{
            "id": null,
            "result": {
                "type": "OrderToExecute",
                "data": {
                    "matchingHash":"0x726289b35d035541068cd3833f8dabc4a2f96580047c3a79ef0735fcba9aba64",
                    "matching":{
                        "makerOrder":{
                            "offerer":"0x00005a24e6254ab46a3ed093c6029caebb157fbd",
                            "inputToken":"0x36ebee707d6a0931a0b9d6fabec252fb9f2865ac",
                            "inputAmount":"499749874937468734",
                            "inputChainId":5,
                            "inputZone":"0xf886ade67ea1f0efc38ea667431acbdb06c91f09",
                            "outputToken":"0x0bc5f399265fa0fb95f5473c8ec1737d1dbb015c",
                            "outputAmount":"1000000000000000000",
                            "outputChainId":5,
                            "outputZone":"0xf886ade67ea1f0efc38ea667431acbdb06c91f09",
                            "startTime":"1706789607",
                            "endTime":"1706793207",
                            "salt":"0",
                            "counter":0,
                            "toWithdraw":false
                        },
                        "takerOrder":{
                            "offerer":"0x0789d82da2fd504138b66af923749b930f564f6b",
                            "inputToken":"0x0bc5f399265fa0fb95f5473c8ec1737d1dbb015c",
                            "inputAmount":"1000300000000000000",
                            "inputChainId":5,
                            "inputZone":"0xf886ade67ea1f0efc38ea667431acbdb06c91f09",
                            "outputToken":"0x36ebee707d6a0931a0b9d6fabec252fb9f2865ac",
                            "outputAmount":"1000000000000000000",
                            "outputChainId":5,
                            "outputZone":"0xf886ade67ea1f0efc38ea667431acbdb06c91f09",
                            "startTime":"1622505600",
                            "endTime":"1725107624",
                            "salt":"12345678",
                            "counter":0,
                            "toWithdraw":false
                        },
                        "makerSignature":"0x401ed3fe56cf2f53c28ed14d8dc7ae2c6255027327c0bcf614be245dcabb305165c2d47942d3747b1e80e346ab15131e6fec1c222aff680fe088f6aeb5ca4ff51b",
                        "takerSignature":"0x2c82886772fca876ed9f6287cdbacfbd2ea9061b54b75999caae52f609702a8b6dd00aaffb13728044a841642979a30d74e0b30b8cd765a1037d76ec91fc01df01",
                        "blockDeadline":10467419,
                        "seatNumber":0,
                        "seatHolder":"0x2EDEB6E06E81020F48d930FA7444a592ebE9FaB6",
                        "seatPercentOfFees":0
                    },
                    "matchingSignature":"0xae9f7fae04558cfbbe1c74991772df1576fb3c991fd2adeff52d9642e287dce6047f294332195605897af52f8206dc166c301e75fe480e5a4ef692a29a7888551c",
                    "makerOrderHash":"0x97b1bade4320158ee5ce751a4c5709634266139e7b82d7bb58343baae1069ac0",
                    "makerChainId":5,
                    "makerZone":"0xf886ade67ea1f0efc38ea667431acbdb06c91f09",
                    "takerOrderHash":"0x3ac5ceca0d753e6354b2c8b4b94f82a2cd59d2f4258a85811928a8f0b3a360b0",
                    "takerChainId":5,
                    "takerZone":"0xf886ade67ea1f0efc38ea667431acbdb06c91f09",
                    "chainId":5,
                    "to":"0xf886ade67ea1f0efc38ea667431acbdb06c91f09",
                    "value":"0",
                    "data": "0x",
                    "maker":"0x00005a24e6254ab46a3ed093c6029caebb157fbd",
                    "taker":"0x0789d82da2fd504138b66af923749b930f564f6b",
                    "inputToken":"0x36ebee707d6a0931a0b9d6fabec252fb9f2865ac",
                    "inputAmount":"499749874937468734",
                    "outputToken":"0x0bc5f399265fa0fb95f5473c8ec1737d1dbb015c",
                    "outputAmount":"1000300000000000000"
                }
            }
        }"#;
        let event = deserialize_aori_feed_event(json_data);
        println!("feed event {:?}", event);
        assert!(event.is_ok());
    }
}
