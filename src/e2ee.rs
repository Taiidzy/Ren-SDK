//! Модуль для E2EE шифрования сообщений
//!
//! Предоставляет высокоуровневые функции для:
//! - Шифрования сообщений для нескольких получателей
//! - Создания конвертов (envelopes) для каждого участника чата
//! - Расшифровки сообщений

use crate::crypto::{generate_message_encryption_key, wrap_symmetric_key};
use crate::error::SdkError;
use crate::network::api::types::messages::Envelope;
use std::collections::HashMap;

/// Шифрует сообщение для нескольких получателей
///
/// Создаёт симметричный ключ, шифрует сообщение этим ключом,
/// и создаёт конверты (envelopes) для каждого получателя.
pub fn encrypt_message_for_recipients(
    message: &str,
    recipient_public_keys: &HashMap<i64, String>, // user_id -> public_key_b64
) -> Result<(String, HashMap<String, Envelope>), SdkError> {
    // Генерируем симметричный ключ для сообщения
    let message_key = generate_message_encryption_key();
    
    // Шифруем сообщение
    let encrypted = crate::crypto::encrypt_message(message, &message_key)?;
    
    // Создаём конверты для каждого получателя
    let mut envelopes = HashMap::new();
    for (user_id, public_key_b64) in recipient_public_keys {
        let (wrapped_key, ephem_pub_key, iv) = wrap_symmetric_key(&message_key, public_key_b64)?;
        envelopes.insert(
            user_id.to_string(),
            Envelope {
                key: wrapped_key,
                ephem_pub_key,
                iv,
            },
        );
    }
    
    Ok((encrypted.ciphertext, envelopes))
}

/// Расшифровывает сообщение используя конверт и приватный ключ
pub fn decrypt_message_with_envelope(
    encrypted_message: &str,
    envelope: &Envelope,
    private_key_b64: &str,
) -> Result<String, SdkError> {
    // Разворачиваем симметричный ключ из конверта
    let message_key = crate::crypto::unwrap_symmetric_key(
        &envelope.key,
        &envelope.ephem_pub_key,
        &envelope.iv,
        private_key_b64,
    )?;
    
    // Расшифровываем сообщение
    let decrypted = crate::crypto::decrypt_message(
        encrypted_message,
        &envelope.iv, // Используем iv как nonce
        &message_key,
    )?;
    
    Ok(decrypted)
}

/// Шифрует файл и создаёт метаданные
pub fn encrypt_file_for_recipients(
    file_data: &[u8],
    filename: &str,
    mimetype: &str,
    recipient_public_keys: &HashMap<i64, String>,
) -> Result<(String, String, HashMap<String, Envelope>), SdkError> {
    // Генерируем симметричный ключ для файла
    let file_key = generate_message_encryption_key();
    
    // Шифруем файл
    let encrypted_file = crate::crypto::encrypt_file(file_data, filename, mimetype, &file_key)?;
    
    // Создаём конверты для каждого получателя
    let mut envelopes = HashMap::new();
    for (user_id, public_key_b64) in recipient_public_keys {
        let (wrapped_key, ephem_pub_key, iv) = wrap_symmetric_key(&file_key, public_key_b64)?;
        envelopes.insert(
            user_id.to_string(),
            Envelope {
                key: wrapped_key,
                ephem_pub_key,
                iv,
            },
        );
    }
    
    Ok((encrypted_file.ciphertext, encrypted_file.nonce, envelopes))
}

