use hex;

use nimiq_transaction::account::staking_contract::IncomingStakingTransactionData;
use wasm_bindgen::prelude::*;

use beserial::Deserialize;

#[wasm_bindgen(js_name = "parseTxData")]
pub fn parse_tx_data(data: String) -> Result<JsValue, JsValue> {
    let data = hex::decode(data).map_err(|_| "Failed to decode hex string")?;
    let tx_type = IncomingStakingTransactionData::deserialize_from_vec(&data)
    .map_err(|_| JsValue::from_str("Failed to parse transaction data"))?;
    let tx_type = serde_wasm_bindgen::to_value(&tx_type)?;
    Ok(tx_type)
}