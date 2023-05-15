use beserial::Deserialize;
use hex;
use nimiq_transaction::account::staking_contract::{
    IncomingStakingTransactionData, OutgoingStakingTransactionProof,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "parseTxData")]
pub fn parse_tx_data(value: String) -> Result<JsValue, JsValue> {
    let value = hex::decode(&value).map_err(|_| "Failed to decode hex string")?;

    let tx_type_incoming = IncomingStakingTransactionData::deserialize_from_vec(&value);
    if let Ok(tx_type_incoming) = tx_type_incoming {
        return Ok(serde_wasm_bindgen::to_value(&tx_type_incoming)?);
    }

    let tx_type_outgoing = OutgoingStakingTransactionProof::deserialize_from_vec(&value);
    if tx_type_outgoing.is_ok() {
        return Err(JsValue::from_str(
            "Parsed transaction data as OutgoingStakingTransactionProof, but failed to serialize",
        ));
    }

    Err(JsValue::from_str("Failed to parse transaction data"))
}
