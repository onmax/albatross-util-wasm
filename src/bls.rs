use wasm_bindgen::prelude::*;

use nimiq_utils::key_rng::SecureGenerate;

#[wasm_bindgen]
pub struct KeyPair {
    #[wasm_bindgen(getter_with_clone, readonly, js_name = "secretKey")]
    pub secret_key: String,

    #[wasm_bindgen(getter_with_clone, readonly, js_name = "publicKey")]
    pub public_key: String,
}

#[wasm_bindgen(js_name = "generateBls")]
pub fn generate_bls() -> KeyPair {
    let rng = &mut rand::thread_rng();
    let keypair = nimiq_bls::KeyPair::generate(rng);

    KeyPair {
        secret_key: keypair.secret_key.to_string(),
        public_key: keypair.public_key.to_string(),
    }
}

