//! Типы данных для Messages API

use serde::{Deserialize, Serialize};

/// Конверт для E2EE (envelope)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Envelope {
    pub key: String, // зашифрованный ключ, base64
    #[serde(rename = "ephem_pub_key")]
    pub ephem_pub_key: String, // эфемерный публичный ключ, base64
    pub iv: String, // вектор инициализации, base64
}

/// Метаданные файла
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct FileMetadata {
    pub file_id: Option<i64>,
    pub filename: String,
    pub mimetype: String,
    pub size: i64,
    pub enc_file: Option<String>, // зашифрованный файл, base64
    pub nonce: Option<String>, // nonce для файла, base64
    #[serde(rename = "file_creation_date")]
    pub file_creation_date: Option<String>,
    pub nonces: Option<Vec<String>>, // для chunked файлов
    pub chunk_size: Option<i64>,
    pub chunk_count: Option<i64>,
}

/// Модель сообщения
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub id: i64,
    pub chat_id: i64,
    pub sender_id: i64,
    pub message: String, // зашифрованное сообщение, E2EE
    pub message_type: String, // "text" | "file" | "image" и т.д.
    pub created_at: String, // ISO8601
    pub edited_at: Option<String>, // ISO8601
    pub is_read: bool,
    #[serde(rename = "has_files")]
    pub has_files: Option<bool>,
    pub metadata: Option<Vec<FileMetadata>>,
    pub envelopes: Option<std::collections::HashMap<String, Envelope>>, // { userId: Envelope }
    pub status: Option<String>, // "pending" | "sent" (для клиента)
}

