use wasm_bindgen::prelude::*;

use beserial::Deserialize;
use nimiq_keys::{Address, PrivateKey, PublicKey, SecureGenerate};
use std::{error::Error, process};

fn parse_private_key(s: &str) -> Result<PrivateKey, Box<dyn Error>> {
    Ok(PrivateKey::deserialize_from_vec(&hex::decode(s)?)?)
}

#[wasm_bindgen]
pub struct Wallet {
    #[wasm_bindgen(getter_with_clone, readonly)]
    pub address: String,

    #[wasm_bindgen(getter_with_clone, readonly, js_name = "addressRaw")]
    pub address_raw: String,

    #[wasm_bindgen(getter_with_clone, readonly, js_name = "publicKey")]
    pub public_key: String,

    #[wasm_bindgen(getter_with_clone, readonly, js_name = "privateKey")]
    pub private_key: String,
}

#[wasm_bindgen(js_name = "generateWallet")]
pub fn generate_wallet(private_key: Option<String>) -> Wallet {
    let private_key = if let Some(p) = private_key {
        match parse_private_key(&p) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Error parsing private key: {e}");
                process::exit(1);
            }
        }
    } else {
        PrivateKey::generate_default_csprng()
    };
    let public_key = PublicKey::from(&private_key);
    let address = Address::from(&public_key);
    
    Wallet {
        address: address.to_user_friendly_address(),
        address_raw: address.to_hex(),
        public_key: public_key.to_hex(),
        private_key: hex::encode(private_key.as_bytes()),
    }
}
