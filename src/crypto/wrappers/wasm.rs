use crate::crypto::*;

// Ergonomic Rust helpers that compose multiple steps

pub fn wrap_key_for_receiver(key_to_wrap: &AeadKey, receiver_public_key_b64: &str) -> Result<(String, String, String), CryptoError> {
    wrap_symmetric_key(key_to_wrap, receiver_public_key_b64)
}

pub fn unwrap_key_from_sender(wrapped_b64: &str, eph_pub_b64: &str, nonce_b64: &str, receiver_priv_b64: &str) -> Result<AeadKey, CryptoError> {
    unwrap_symmetric_key(wrapped_b64, eph_pub_b64, nonce_b64, receiver_priv_b64)
}

pub fn encrypt_text_with_secret(secret: &str, message: &str) -> Result<EncryptedMessage, CryptoError> {
    let key = derive_key_from_string(secret)?;
    encrypt_message(message, &key)
}

pub fn decrypt_text_with_secret(secret: &str, ciphertext_b64: &str, nonce_b64: &str) -> Result<String, CryptoError> {
    let key = derive_key_from_string(secret)?;
    decrypt_message(ciphertext_b64, nonce_b64, &key)
}

#[cfg(feature = "wasm")]
mod wasm {
    use super::*;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn wasm_generate_key_pair() -> JsValue {
        let kp = generate_key_pair(false);
        JsValue::from_serde(&kp).unwrap()
    }

    #[wasm_bindgen]
    pub fn wasm_generate_salt() -> String { generate_salt() }

    #[wasm_bindgen]
    pub fn wasm_generate_nonce() -> String { generate_nonce() }

    #[wasm_bindgen]
    pub fn wasm_encrypt_message(secret: &str, message: &str) -> Result<JsValue, JsValue> {
        encrypt_text_with_secret(secret, message)
            .map(|em| JsValue::from_serde(&em).unwrap())
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen]
    pub fn wasm_decrypt_message(secret: &str, ciphertext_b64: &str, nonce_b64: &str) -> Result<String, JsValue> {
        decrypt_text_with_secret(secret, ciphertext_b64, nonce_b64)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen]
    pub fn wasm_wrap_symmetric_key(key_raw_b64: &str, receiver_public_key_b64: &str) -> Result<JsValue, JsValue> {
        // key_raw_b64 is raw 32-byte AEAD key in Base64
        let key_bytes = base64::engine::general_purpose::STANDARD.decode(key_raw_b64).map_err(|e| JsValue::from_str(&e.to_string()))?;
        let aead = AeadKey::from_bytes(&key_bytes).map_err(|e| JsValue::from_str(&e.to_string()))?;
        let (wrapped, eph, nonce) = wrap_symmetric_key(&aead, receiver_public_key_b64).map_err(|e| JsValue::from_str(&e.to_string()))?;
        let obj = serde_json::json!({
            "wrappedKey": wrapped,
            "ephemeralPublicKey": eph,
            "nonce": nonce,
        });
        Ok(JsValue::from_serde(&obj).unwrap())
    }

    #[wasm_bindgen]
    pub fn wasm_unwrap_symmetric_key(wrapped_b64: &str, eph_pub_b64: &str, nonce_b64: &str, receiver_priv_b64: &str) -> Result<String, JsValue> {
        let key = unwrap_symmetric_key(wrapped_b64, eph_pub_b64, nonce_b64, receiver_priv_b64)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(base64::engine::general_purpose::STANDARD.encode(key.to_bytes()))
    }
}
