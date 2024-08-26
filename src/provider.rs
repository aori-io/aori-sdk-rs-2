use jsonrpsee::{
    http_client::{HttpClient, HttpClientBuilder},
    proc_macros::rpc,
};
use serde::{Deserialize, Serialize};

use super::AoriPingParams;

#[derive(Clone)]
pub struct ProviderClient {
    pub urls: Vec<String>,
}

////////////////////////////////////////////////////////////////
//                        PARAM STRUCTS
////////////////////////////////////////////////////////////////

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AoriGetBlockNumberParams {
    pub chain_id: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AoriGetNonceParams {
    pub address: String,
    pub chain_id: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AoriGetFeeDataParams {
    pub chain_id: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AoriEsimateGasParams {
    pub from: String,
    pub to: String,
    pub value: String,
    pub data: String,
    pub chain_id: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AoriEstimateGasParams {
    pub from: String,
    pub to: String,
    pub value: String,
    pub data: String,
    pub chain_id: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AoriGetTokenAllowanceParams {
    pub owner: String,
    pub spender: String,
    pub token: String,
    pub chain_id: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AoriGetTokenBalanceParams {
    pub owner: String,
    pub token: String,
    pub chain_id: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AoriGetTokenDetailsParams {
    pub token: String,
    pub chain_id: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AoriGetNativeBalanceParams {
    pub chain_id: u64,
    pub address: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AoriGetAoriCounterParams {
    pub chain_id: u64,
    pub address: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AoriGetSeatDetailsParams {
    pub seat_id: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AoriHasOrderSettledParams {
    pub order_hash: String,
    pub zone: String,
    pub chain_id: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AoriSimulateTransactionParams {
    pub signed_tx: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AoriSendTransactionParams {
    pub signed_tx: String,
}

////////////////////////////////////////////////////////////////
//                      RESPONSE STRUCTS
////////////////////////////////////////////////////////////////

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AoriGetBlockNumberResponse {
    pub id: u64,
    pub result: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AoriGetNonceResponse {
    pub id: u64,
    pub result: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AoriGetFeeDataResponse {
    pub id: u64,
    pub result: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AoriEstimateGasResponse {
    pub id: u64,
    pub result: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AoriGetTokenAllowanceResponse {
    pub id: u64,
    pub result: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AoriGetTokenBalanceResponse {
    pub id: u64,
    pub result: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AoriGetTokenDetailsResponse {
    pub id: u64,
    pub result: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AoriGetNativeBalanceResponse {
    pub id: u64,
    pub result: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AoriGetAoriCounterResponse {
    pub id: u64,
    pub result: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AoriGetSeatDetailsResponse {
    pub id: u64,
    pub result: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AoriHasOrderSettledResponse {
    pub id: u64,
    pub result: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AoriSimulateTransactionResponse {
    pub id: u64,
    pub result: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AoriSendTransactionResponse {
    pub id: u64,
    pub result: String,
}

#[rpc(client)]
pub trait AoriDataProviderRpc {
    #[method(name = "aori_ping")]
    async fn ping_provider(&self, parameters: AoriPingParams) -> RpcResult<String>;
    #[method(name = "aori_getBlockNumber")]
    async fn get_block_number(
        &self,
        parameters: AoriGetBlockNumberParams,
    ) -> RpcResult<AoriGetBlockNumberResponse>;
    #[method(name = "aori_getNonce")]
    async fn get_nonce(&self, parameters: AoriGetNonceParams) -> RpcResult<AoriGetNonceResponse>;
    #[method(name = "aori_getFeeData")]
    async fn get_fee_data(
        &self,
        parameters: AoriGetFeeDataParams,
    ) -> RpcResult<AoriGetFeeDataResponse>;
    #[method(name = "aori_estimateGas")]
    async fn estimate_gas(
        &self,
        parameters: AoriEstimateGasParams,
    ) -> RpcResult<AoriEstimateGasResponse>;
    #[method(name = "aori_getTokenAllowance")]
    async fn get_token_allowance(
        &self,
        parameters: AoriGetTokenAllowanceParams,
    ) -> RpcResult<AoriGetTokenAllowanceResponse>;
    #[method(name = "aori_getTokenBalance")]
    async fn get_token_balance(
        &self,
        parameters: AoriGetTokenBalanceParams,
    ) -> RpcResult<AoriGetTokenBalanceResponse>;
    #[method(name = "aori_getTokenDetails")]
    async fn get_token_details(
        &self,
        parameters: AoriGetTokenDetailsParams,
    ) -> RpcResult<AoriGetTokenDetailsResponse>;
    #[method(name = "aori_getNativeBalance")]
    async fn get_native_balance(
        &self,
        parameters: AoriGetNativeBalanceParams,
    ) -> RpcResult<AoriGetNativeBalanceResponse>;
    #[method(name = "aori_getAoriCounter")]
    async fn get_aori_counter(
        &self,
        parameters: AoriGetAoriCounterParams,
    ) -> RpcResult<AoriGetAoriCounterResponse>;
    #[method(name = "aori_getSeatDetails")]
    async fn get_seat_details(
        &self,
        parameters: AoriGetSeatDetailsParams,
    ) -> RpcResult<AoriGetSeatDetailsResponse>;
    #[method(name = "aori_hasOrderSettled")]
    async fn has_order_settled(
        &self,
        parameters: AoriHasOrderSettledParams,
    ) -> RpcResult<AoriHasOrderSettledResponse>;
    #[method(name = "aori_simulateTransaction")]
    async fn simulate_transaction(
        &self,
        parameters: AoriSimulateTransactionParams,
    ) -> RpcResult<AoriSimulateTransactionResponse>;
    #[method(name = "aori_sendTransaction")]
    async fn send_transaction(
        &self,
        parameters: AoriSendTransactionParams,
    ) -> RpcResult<AoriSendTransactionResponse>;
}

impl ProviderClient {
    pub fn new(urls: Vec<String>) -> Self {
        Self { urls }
    }

    pub fn get_client(&self) -> HttpClient {
        let url = "https://provider.aori.io";
        HttpClientBuilder::default().build(url).unwrap()
    }

    pub async fn ping(&self) -> Result<String, anyhow::Error> {
        let client = self.get_client();
        let request = AoriPingParams::default();
        Ok(client.ping_provider(request).await?)
    }

    pub async fn get_block_number(
        &self,
        chain_id: u64,
    ) -> Result<AoriGetBlockNumberResponse, anyhow::Error> {
        let client = self.get_client();
        Ok(client.get_block_number(AoriGetBlockNumberParams { chain_id }).await?)
    }

    pub async fn get_nonce(
        &self,
        address: String,
        chain_id: u64,
    ) -> Result<AoriGetNonceResponse, anyhow::Error> {
        let client = self.get_client();
        Ok(client.get_nonce(AoriGetNonceParams { address, chain_id }).await?)
    }

    pub async fn get_fee_data(
        &self,
        chain_id: u64,
    ) -> Result<AoriGetFeeDataResponse, anyhow::Error> {
        let client = self.get_client();
        Ok(client.get_fee_data(AoriGetFeeDataParams { chain_id }).await?)
    }

    pub async fn estimate_gas(
        &self,
        from: String,
        to: String,
        value: String,
        data: String,
        chain_id: u64,
    ) -> Result<AoriEstimateGasResponse, anyhow::Error> {
        let client = self.get_client();
        Ok(client.estimate_gas(AoriEstimateGasParams { from, to, value, data, chain_id }).await?)
    }

    pub async fn is_valid_signature() {}

    pub async fn has_order_settled(
        &self,
        order_hash: String,
        zone: String,
        chain_id: u64,
    ) -> Result<AoriHasOrderSettledResponse, anyhow::Error> {
        let client = self.get_client();
        Ok(client
            .has_order_settled(AoriHasOrderSettledParams { order_hash, zone, chain_id })
            .await?)
    }

    pub async fn get_aori_counter(
        &self,
        chain_id: u64,
        address: String,
    ) -> Result<AoriGetAoriCounterResponse, anyhow::Error> {
        let client = self.get_client();
        Ok(client.get_aori_counter(AoriGetAoriCounterParams { chain_id, address }).await?)
    }

    pub async fn get_native_balance(
        &self,
        chain_id: u64,
        address: String,
    ) -> Result<AoriGetNativeBalanceResponse, anyhow::Error> {
        let client = self.get_client();
        Ok(client.get_native_balance(AoriGetNativeBalanceParams { chain_id, address }).await?)
    }

    pub async fn get_token_balance(
        &self,
        chain_id: u64,
        owner: String,
        token: String,
    ) -> Result<AoriGetTokenBalanceResponse, anyhow::Error> {
        let client = self.get_client();
        Ok(client.get_token_balance(AoriGetTokenBalanceParams { owner, token, chain_id }).await?)
    }

    pub async fn get_token_allowance(
        &self,
        owner: String,
        spender: String,
        token: String,
        chain_id: u64,
    ) -> Result<AoriGetTokenAllowanceResponse, anyhow::Error> {
        let client = self.get_client();
        Ok(client
            .get_token_allowance(AoriGetTokenAllowanceParams { owner, spender, token, chain_id })
            .await?)
    }

    pub async fn get_token_details(
        &self,
        token: String,
        chain_id: u64,
    ) -> Result<AoriGetTokenDetailsResponse, anyhow::Error> {
        let client = self.get_client();
        Ok(client.get_token_details(AoriGetTokenDetailsParams { token, chain_id }).await?)
    }

    pub async fn simulate_transaction(
        &self,
        signed_tx: String,
    ) -> Result<AoriSimulateTransactionResponse, anyhow::Error> {
        let client = self.get_client();
        Ok(client.simulate_transaction(AoriSimulateTransactionParams { signed_tx }).await?)
    }

    // TODO: implement
    pub async fn get_seat_details(
        &self,
        seat_id: u64,
    ) -> Result<AoriGetSeatDetailsResponse, anyhow::Error> {
        let client = self.get_client();
        Ok(client.get_seat_details(AoriGetSeatDetailsParams { seat_id }).await?)
    }

    pub async fn send_transaction(
        &self,
        signed_tx: String,
    ) -> Result<AoriSendTransactionResponse, anyhow::Error> {
        let client = self.get_client();
        Ok(client.send_transaction(AoriSendTransactionParams { signed_tx }).await?)
    }

    // TODO: implement
    pub async fn static_call() {}

    // TODO: implement
    pub async fn compute_create3_address() {}

    // TODO: implement
    pub async fn is_contract() {}
}

#[cfg(test)]
mod provider_tests {
    use crate::{provider::AoriDataProviderRpcClient, AoriPingParams};
    use jsonrpsee::http_client::HttpClientBuilder;
    use tracing::info;

    #[tokio::test]
    async fn test_ping() {
        let url = "https://provider.aori.io";
        let client = HttpClientBuilder::default().build(url).unwrap();

        let request = AoriPingParams::default();
        let response = client.ping_provider(request).await;
        info!("PING RESPONSE: {:?}", response);
        assert!(response.is_ok(), "Expected Ok response, got {:?}", response);
    }
}
