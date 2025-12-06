//! Типы данных для Chats API

use serde::{Deserialize, Serialize};

/// Модель чата
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Chat {
    pub id: i64,
    pub kind: String, // "private" | "group"
    pub title: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub is_archived: Option<bool>,
    pub peer_avatar: Option<String>,
    pub peer_username: String,
}

/// Запрос на создание чата
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateChatRequest {
    pub kind: String, // "private" | "group"
    pub title: Option<String>,
    pub user_ids: Vec<i64>,
}

