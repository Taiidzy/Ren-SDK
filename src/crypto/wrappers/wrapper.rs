use crate::crypto::{
    decrypt_message, derive_key_from_password, derive_key_from_string, encrypt_message,
    EncryptedMessage, CryptoError,
};
use base64::{engine::general_purpose, Engine as _};
use crate::AeadKey;

/// Шифрует строковое сообщение, используя секретную строку как источник ключа.
///
/// Внутри: SHA-256 от `secret` -> AEAD ключ -> `encrypt_message`.
/// Возвращает `EncryptedMessage` с Base64-полями `ciphertext` и `nonce`.
pub fn encrypt_message_with_secret(secret: &str, message: &str) -> Result<EncryptedMessage, CryptoError> {
    let key = derive_key_from_string(secret)?;
    encrypt_message(message, &key)
}

/// Дешифрует строковое сообщение, зашифрованное через `encrypt_message_with_secret`.
///
/// Принимает: `secret` (строка), `ciphertext_b64` и `nonce_b64` (Base64).
/// Возвращает исходную строку при успешной аутентифицированной расшифровке.
pub fn decrypt_message_with_secret(secret: &str, ciphertext_b64: &str, nonce_b64: &str) -> Result<String, CryptoError> {
    let key = derive_key_from_string(secret)?;
    decrypt_message(ciphertext_b64, nonce_b64, &key)
}

/// Деривирует мастер-ключ (AEAD, 32 байта) из пароля и соли (Base64-16 байт).
/// Возвращает AeadKey, который можно использовать для шифрования/дешифрования.
pub fn derive_master_key_from_password(password: &str, salt_b64: &str) -> Result<AeadKey, CryptoError> {
    derive_key_from_password(password, salt_b64)
}

/// Деривирует мастер-ключ из пароля и соли и экспортирует его как Base64 (сырые 32 байта).
pub fn derive_master_key_b64_from_password(password: &str, salt_b64: &str) -> Result<String, CryptoError> {
    let key = derive_key_from_password(password, salt_b64)?;
    Ok(general_purpose::STANDARD.encode(key.to_bytes()))
}