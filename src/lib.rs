#![deny(unsafe_code)]
#![cfg_attr(target_arch = "wasm32", allow(dead_code))]

// Crypto модуль
#[path = "crypto/crypto.rs"]
pub mod crypto;
#[path = "crypto/wrappers/wasm.rs"]
pub mod wasm_crypto;
#[path = "crypto/wrappers/wrapper.rs"]
pub mod wrapper;

// Client и Error
pub mod client;
pub mod error;

// E2EE модуль
pub mod e2ee;

// Network модули
#[path = "network/api/auth.rs"]
pub mod auth;
pub mod network;

// Реэкспорт основных типов
pub use crypto::{
    AeadKey, CryptoError, DecryptedFileWithMessage, EncryptedFile, EncryptedFileWithMessage,
    EncryptedMessage, KeyPair, decrypt_data, decrypt_file, decrypt_file_with_message,
    decrypt_message, derive_key_from_password, derive_key_from_string, encrypt_data,
    encrypt_file, encrypt_file_with_message, encrypt_message, export_private_key_b64,
    export_public_key_b64, generate_key_pair, generate_message_encryption_key, generate_nonce,
    generate_salt, import_private_key_b64, import_public_key_b64, unwrap_symmetric_key,
    wrap_symmetric_key,
};

pub use wrapper::{
    decrypt_message_with_secret, derive_master_key_b64_from_password,
    derive_master_key_from_password, encrypt_message_with_secret,
};

// Client
pub use client::RenClient;

// Error
pub use error::SdkError;

// Auth (старый API для обратной совместимости)
pub use auth::login;
pub use network::api::types::auth::AuthError;

// Network API
pub use network::api::auth::{login as api_login, register};
pub use network::api::users::{delete_me, get_avatar, get_me, get_public_key, update_avatar, update_username};
pub use network::api::chats::{create_chat, delete_chat, get_chats, get_messages};

// Network Types
pub use network::api::types::chats::{Chat, CreateChatRequest};
pub use network::api::types::messages::{Envelope, FileMetadata, Message};
pub use network::api::types::users::{PublicKeyResponse, UpdateUsernameRequest, UserResponse};
pub use network::api::types::auth::{LoginRequest, LoginResponse, RegisterRequest};

// WebSocket
#[cfg(feature = "websocket")]
pub use network::websocket::{WsClient, WsEvent};

// FFI модули
#[cfg(feature = "ffi")]
pub mod ffi;

#[cfg(feature = "wasm")]
pub mod wasm {
    pub use crate::ffi::wasm::*;
}
