
use alloy::{primitives::{keccak256, Address, Bytes, B256, U256}, sol, sol_types::SolValue};
use chrono::Utc;
use rand::random;
use serde::{Deserialize, Deserializer, Serialize};

use crate::constants::{ChainId, AORI_V2_SINGLE_CHAIN_ZONE_ADDRESSES};

use super::{
    constants::SUPPORTED_AORI_CHAINS,
    get_order_signer,
};

sol!(AoriV2, "src/abi/AoriV2.json");

sol!(
    #[derive(Default, Debug, Deserialize, Serialize)]
    struct AoriOrder {
        address offerer;
        // input
        address inputToken;
        uint256 inputAmount;
        uint256 inputChainId;
        address inputZone;
        // output
        address outputToken;
        uint256 outputAmount;
        uint256 outputChainId;
        address outputZone;
        // other
        uint256 startTime;
        uint256 endTime;
        uint256 salt;
        uint256 counter;
        bool toWithdraw;
    }

    #[derive(Default, Debug, Deserialize, Serialize)]
    struct AoriMatchingDetails {
        AoriOrder makerOrder;
        AoriOrder takerOrder;

        bytes makerSignature;
        bytes takerSignature;
        uint256 blockDeadline;

        uint256 seatNumber;
        address seatHolder;
        uint256 seatPercentOfFees;
    }

    #[derive(Default, Debug, Deserialize, Serialize)]
    struct AoriMatchingDetailsHashingData {
        bytes makerSignature;
        bytes takerSignature;

        uint256 blockDeadline;

        uint256 seatNumber;
        address seatHolder;
        uint256 seatPercentOfFees;
    }
);

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DetailsToExecute {
    pub matching_hash: B256,
    pub matching: AoriMatchingDetails,
    pub matching_signature: String,
    pub maker_order_hash: B256,
    pub maker_chain_id: U256,
    pub maker_zone: Address,
    pub taker_order_hash: B256,
    pub taker_chain_id: U256,
    pub taker_zone: Address,
    pub chain_id: U256,
    pub to: Address,
    pub value: U256,
    pub data: Vec<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taker_permit_signature: Option<String>,
    pub maker: Address,
    pub taker: Address,
    pub input_token: Address,
    pub input_amount: U256,
    pub output_token: Address,
    pub output_amount: U256,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SettledMatch {
    pub maker_order_hash: B256,
    pub taker_order_hash: B256,
    pub maker: Address,
    pub taker: Address,
    pub input_chain_id: U256,
    pub output_chain_id: U256,
    pub input_zone: Address,
    pub output_zone: Address,
    pub input_token: Address,
    pub output_token: Address,
    pub input_amount: U256,
    pub output_amount: U256,
    pub matching_hash: B256,

    // Details from the input chain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_hash: Option<B256>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_number: Option<U256>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<U256>,
}

pub fn deserialize_rate<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let rate: f64 = Deserialize::deserialize(deserializer)?;
    Ok(rate.to_string())
}

pub fn get_zone_address(chain_id: u64) -> Address {
    // Gets first address from the set of zone addresses for the given chain id
    let chain_id = ChainId::from(chain_id);
    AORI_V2_SINGLE_CHAIN_ZONE_ADDRESSES().get(&chain_id).unwrap().iter().next().unwrap().parse::<Address>().unwrap()
}

pub fn create_limit_order(
    offerer: Address,
    input_token: Address,
    output_token: Address,
    input_amount: U256,
    output_amount: U256,
    chain_id: u64,
) -> AoriOrder {
    AoriOrder {
        offerer,
        inputToken: input_token,
        inputAmount: input_amount,
        inputChainId: U256::from(chain_id),
        inputZone: get_zone_address(chain_id),
        outputToken: output_token,
        outputAmount: output_amount,
        outputChainId: U256::from(chain_id),
        outputZone: get_zone_address(chain_id),
        counter: U256::ZERO,
        startTime: U256::from(Utc::now().timestamp() - 300), // 5 mins ago in seconds
        endTime: U256::from(Utc::now().timestamp() + 604800), // 1 week from now in seconds
        salt: U256::from(random::<u64>()), // TODO: randomly generate
        toWithdraw: false,
    }
}

pub fn get_order_hash(order: AoriOrder) -> B256 {
    keccak256(order.abi_encode_packed())
}

pub fn to_details_to_execute(
    matching: AoriMatchingDetails,
    matching_signature: String,
    to: Address,
    value: U256,
    data: Vec<u8>,
) -> DetailsToExecute {
    let matching2 = matching.clone();
    let maker_order = matching.makerOrder.clone();
    let taker_order = matching.takerOrder.clone();

    DetailsToExecute {
        matching_hash: get_matching_hash(
            matching.makerSignature,
            matching.takerSignature,
            matching.blockDeadline,
            matching.seatNumber,
            matching.seatHolder,
            matching.seatPercentOfFees,
        ),
        matching_signature,
        maker_chain_id: maker_order.inputChainId,
        maker_zone: maker_order.inputZone,
        taker_chain_id: taker_order.inputChainId,
        taker_zone: taker_order.inputZone,
        chain_id: matching.blockDeadline,
        to,
        value,
        data,
        taker_permit_signature: None,
        maker: maker_order.clone().offerer,
        taker: taker_order.clone().offerer,
        input_token: maker_order.clone().inputToken,
        input_amount: maker_order.clone().inputAmount,
        output_token: taker_order.outputToken,
        output_amount: taker_order.outputAmount,
        maker_order_hash: get_order_hash(maker_order),
        taker_order_hash: get_order_hash(taker_order),
        matching: matching2,
    }
}

pub fn get_matching_hash(
    maker_signature: Bytes,
    taker_signature: Bytes,
    block_deadline: U256,
    seat_number: U256,
    seat_holder: Address,
    seat_percent_of_fees: U256,
) -> B256 {
    keccak256(
        (AoriMatchingDetailsHashingData {
            makerSignature: maker_signature,
            takerSignature: taker_signature,
            blockDeadline: block_deadline,

            seatNumber: seat_number,
            seatHolder: seat_holder,
            seatPercentOfFees: seat_percent_of_fees,
        })
        .abi_encode_packed(),
    )
}

pub fn calldata_to_settle_orders(matching: AoriMatchingDetails) -> Vec<u8> {
    let mut calldata = Vec::new();
    calldata.extend(matching.abi_encode_packed());
    calldata
}

// Note: Some() is used to return an error message if the order is invalid
pub async fn validate_order(order: AoriOrder, signature: String) -> Result<String, &'static str> {
    let order2 = order.clone();

    if !SUPPORTED_AORI_CHAINS().contains(&order2.inputChainId) {
        return Err("Input chain not supported");
    }

    if !SUPPORTED_AORI_CHAINS().contains(&order2.outputChainId) {
        return Err("Output chain not supported");
    }

    if signature == "" || signature == "0x" {
        return Err("No signature provided");
    }

    if order2.inputToken == order2.outputToken && order2.inputChainId == order2.outputChainId {
        return Err("Input token and output token must be different if they are on the same chain");
    }

    if order2.inputAmount == U256::ZERO {
        return Err("Input amount cannot be zero");
    }

    if order2.outputAmount == U256::ZERO {
        return Err("Output amount cannot be zero");
    }

    if order2.startTime > order2.endTime {
        return Err("Start time cannot be after end time");
    }

    if order2.endTime < U256::from(Utc::now().timestamp()) {
        return Err("End time cannot be in the past");
    }

    // Verify that the signature of the taker order is valid
    let order_message_signer = match get_order_signer(order, &signature).await {
        Ok(signer) => signer,
        Err(e) => return Err("Signature signer could not be retrieved"),
    };

    // TODO: add in isValidCall to validate the signature for vaults

    return Ok(order_message_signer.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn hash_order() {
        let order = AoriOrder {
            offerer: "0x0000000000000000000000000000000000000001".parse::<Address>().unwrap(),
            inputToken: "0x0000000000000000000000000000000000000002".parse::<Address>().unwrap(),
            inputAmount: U256::from(1000000000000000000_u64),
            inputChainId: U256::from(1),
            inputZone: "0x0000000000000000000000000000000000000003".parse::<Address>().unwrap(),
            outputToken: "0x0000000000000000000000000000000000000004".parse::<Address>().unwrap(),
            outputAmount: U256::from(2000000000000000000_u64),
            outputChainId: U256::from(1),
            outputZone: "0x0000000000000000000000000000000000000000".parse::<Address>().unwrap(),
            startTime: U256::from(1619827200),
            endTime: U256::from(1622428800),
            salt: U256::from(1),
            counter: U256::from(1),
            toWithdraw: false,
        };
        let packed = order.abi_encode_packed();
        let hash = keccak256(packed);
        println!("rust order hash {}", hash);
        assert_eq!(
            hash.to_string(),
            "0x214356d7e7b271d965916a29d61e111e8106f54a2b805a742c18bf93f9f2372e",
            "Order hash does not match expected value"
        );
    }

    #[test]
    fn serialize_aori_order() {
        let order = AoriOrder {
            offerer: "0x0000000000000000000000000000000000000001".parse::<Address>().unwrap(),
            inputToken: "0x0000000000000000000000000000000000000002".parse::<Address>().unwrap(),
            inputAmount: U256::from(1000000000000000000_u64),
            inputChainId: U256::from(1),
            inputZone: "0x0000000000000000000000000000000000000003".parse::<Address>().unwrap(),
            outputToken: "0x0000000000000000000000000000000000000004".parse::<Address>().unwrap(),
            outputAmount: U256::from(2000000000000000000_u64),
            outputChainId: U256::from(1),
            outputZone: "0x0000000000000000000000000000000000000000".parse::<Address>().unwrap(),
            startTime: U256::from(1619827200),
            endTime: U256::from(1622428800),
            salt: U256::from(1),
            counter: U256::from(1),
            toWithdraw: false,
        };

        let serialized = serde_json::to_string(&order).unwrap();
        println!("Serialized AoriOrder: {}", serialized);

        // Deserialize the order
        let deserialized: AoriOrder = serde_json::from_str(&serialized).unwrap();
        println!("Deserialized AoriOrder: {:?}", deserialized);
    }
}
