//! WebAssembly bindings для Ren SDK
//!
//! Предоставляет JavaScript-совместимый API для использования SDK в веб-приложениях

use wasm_bindgen::prelude::*;

use crate::client::RenClient;
use crate::network::api::types::auth::{LoginRequest, RegisterRequest};
use crate::network::api::types::chats::CreateChatRequest;
use crate::network::api::types::users::UpdateUsernameRequest;

/// Клиент SDK для WebAssembly
#[wasm_bindgen]
pub struct WasmClient {
    client: RenClient,
}

#[wasm_bindgen]
impl WasmClient {
    /// Создаёт новый клиент SDK
    #[wasm_bindgen(constructor)]
    pub fn new(base_url: String) -> WasmClient {
        WasmClient {
            client: RenClient::new(base_url),
        }
    }

    /// Устанавливает токен авторизации
    #[wasm_bindgen]
    pub fn set_token(&self, token: String) {
        self.client.set_token(token);
    }

    /// Получает токен
    #[wasm_bindgen]
    pub fn get_token(&self) -> Option<String> {
        self.client.get_token()
    }

    /// Выполняет вход в систему
    #[wasm_bindgen]
    pub async fn login(&self, login: String, password: String, remember_me: Option<bool>) -> Result<JsValue, JsValue> {
        let req = LoginRequest {
            login,
            password,
            remember_me,
        };

        // Для WASM reqwest уже поддерживает async через wasm_bindgen_futures
        match crate::network::api::auth::login(&self.client, req).await {
            Ok(resp) => {
                Ok(JsValue::from_serde(&resp).map_err(|e| JsValue::from_str(&e.to_string()))?)
            }
            Err(e) => Err(JsValue::from_str(&format!("Login failed: {}", e))),
        }
    }

    /// Регистрация нового пользователя
    #[wasm_bindgen]
    pub async fn register(
        &self,
        login: String,
        username: String,
        password: String,
        pkebymk: String,
        pkebyrk: String,
        salt: String,
        pk: String,
    ) -> Result<JsValue, JsValue> {
        let req = RegisterRequest {
            login,
            username,
            password,
            pkebymk,
            pkebyrk,
            salt,
            pk,
        };

        match crate::network::api::auth::register(&self.client, req, None, None).await {
            Ok(user) => {
                Ok(JsValue::from_serde(&user).map_err(|e| JsValue::from_str(&e.to_string()))?)
            }
            Err(e) => Err(JsValue::from_str(&format!("Registration failed: {}", e))),
        }
    }

    /// Получает профиль текущего пользователя
    #[wasm_bindgen]
    pub async fn get_me(&self) -> Result<JsValue, JsValue> {
        match crate::network::api::users::get_me(&self.client).await {
            Ok(user) => {
                Ok(JsValue::from_serde(&user).map_err(|e| JsValue::from_str(&e.to_string()))?)
            }
            Err(e) => Err(JsValue::from_str(&format!("Failed to get profile: {}", e))),
        }
    }

    /// Обновляет имя пользователя
    #[wasm_bindgen]
    pub async fn update_username(&self, username: String) -> Result<JsValue, JsValue> {
        let req = UpdateUsernameRequest { username };
        match crate::network::api::users::update_username(&self.client, req).await {
            Ok(user) => {
                Ok(JsValue::from_serde(&user).map_err(|e| JsValue::from_str(&e.to_string()))?)
            }
            Err(e) => Err(JsValue::from_str(&format!("Failed to update username: {}", e))),
        }
    }

    /// Получает список чатов
    #[wasm_bindgen]
    pub async fn get_chats(&self) -> Result<JsValue, JsValue> {
        match crate::network::api::chats::get_chats(&self.client).await {
            Ok(chats) => {
                Ok(JsValue::from_serde(&chats).map_err(|e| JsValue::from_str(&e.to_string()))?)
            }
            Err(e) => Err(JsValue::from_str(&format!("Failed to get chats: {}", e))),
        }
    }

    /// Создаёт чат
    #[wasm_bindgen]
    pub async fn create_chat(&self, kind: String, title: Option<String>, user_ids: Vec<i64>) -> Result<JsValue, JsValue> {
        let req = CreateChatRequest {
            kind,
            title,
            user_ids,
        };

        match crate::network::api::chats::create_chat(&self.client, req).await {
            Ok(chat) => {
                Ok(JsValue::from_serde(&chat).map_err(|e| JsValue::from_str(&e.to_string()))?)
            }
            Err(e) => Err(JsValue::from_str(&format!("Failed to create chat: {}", e))),
        }
    }

    /// Получает сообщения чата
    #[wasm_bindgen]
    pub async fn get_messages(&self, chat_id: i64) -> Result<JsValue, JsValue> {
        match crate::network::api::chats::get_messages(&self.client, chat_id).await {
            Ok(messages) => {
                Ok(JsValue::from_serde(&messages).map_err(|e| JsValue::from_str(&e.to_string()))?)
            }
            Err(e) => Err(JsValue::from_str(&format!("Failed to get messages: {}", e))),
        }
    }

    /// Получает публичный ключ пользователя
    #[wasm_bindgen]
    pub async fn get_public_key(&self, user_id: i64) -> Result<JsValue, JsValue> {
        match crate::network::api::users::get_public_key(&self.client, user_id).await {
            Ok(key) => {
                Ok(JsValue::from_serde(&key).map_err(|e| JsValue::from_str(&e.to_string()))?)
            }
            Err(e) => Err(JsValue::from_str(&format!("Failed to get public key: {}", e))),
        }
    }
}

/// Криптографические функции для WebAssembly

/// Генерирует пару ключей
#[wasm_bindgen]
pub fn wasm_generate_keypair() -> JsValue {
    let kp = crate::generate_key_pair(false);
    JsValue::from_serde(&kp).unwrap()
}

/// Генерирует соль
#[wasm_bindgen]
pub fn wasm_generate_salt() -> String {
    crate::generate_salt()
}

/// Генерирует nonce
#[wasm_bindgen]
pub fn wasm_generate_nonce() -> String {
    crate::generate_nonce()
}

/// Шифрует сообщение для нескольких получателей
#[wasm_bindgen]
pub fn wasm_encrypt_message_for_recipients(
    message: &str,
    recipient_keys_json: &str,
) -> Result<JsValue, JsValue> {
    use std::collections::HashMap;
    
    let recipient_keys: HashMap<i64, String> = serde_json::from_str(recipient_keys_json)
        .map_err(|e| JsValue::from_str(&format!("Invalid recipient keys JSON: {}", e)))?;

    match crate::e2ee::encrypt_message_for_recipients(message, &recipient_keys) {
        Ok((encrypted_msg, envelopes)) => {
            let result = serde_json::json!({
                "encrypted_message": encrypted_msg,
                "envelopes": envelopes,
            });
            Ok(JsValue::from_serde(&result).unwrap())
        }
        Err(e) => Err(JsValue::from_str(&format!("Encryption failed: {}", e))),
    }
}

/// Расшифровывает сообщение с конвертом
#[wasm_bindgen]
pub fn wasm_decrypt_message_with_envelope(
    encrypted_message: &str,
    envelope_json: &str,
    private_key_b64: &str,
) -> Result<String, JsValue> {
    use crate::network::api::types::messages::Envelope;
    
    let envelope: Envelope = serde_json::from_str(envelope_json)
        .map_err(|e| JsValue::from_str(&format!("Invalid envelope JSON: {}", e)))?;

    match crate::e2ee::decrypt_message_with_envelope(encrypted_message, &envelope, private_key_b64) {
        Ok(decrypted) => Ok(decrypted),
        Err(e) => Err(JsValue::from_str(&format!("Decryption failed: {}", e))),
    }
}

/// Инициализирует wasm модуль
#[wasm_bindgen(start)]
pub fn init() {
    // Инициализация логирования для wasm
    #[cfg(feature = "wasm")]
    console_error_panic_hook::set_once();
}

