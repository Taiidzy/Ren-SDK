//! Типы данных для Users API

use serde::{Deserialize, Serialize};

/// Ответ с данными пользователя
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UserResponse {
    pub id: i64,
    pub login: String,
    pub username: String,
    pub avatar: Option<String>,
}

/// Запрос на изменение имени пользователя
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateUsernameRequest {
    pub username: String,
}

/// Ответ с публичным ключом пользователя
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicKeyResponse {
    pub user_id: i64,
    pub public_key: String,
}

