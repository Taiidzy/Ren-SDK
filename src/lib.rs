#![deny(unsafe_code)]

#[path = "crypto/crypto.rs"]
pub mod crypto;
#[path = "crypto/wrappers/wasm.rs"]
pub mod wasm;
#[path = "crypto/wrappers/wrapper.rs"]
pub mod wrapper;

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

