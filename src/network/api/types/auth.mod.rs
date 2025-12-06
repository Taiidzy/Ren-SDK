//! Типы данных для API аутентификации.
//!
//! `LoginRequest` — JSON-запрос на сервер.
//! `LoginResponse` — ожидаемый JSON-ответ; некоторые поля помечены `serde`-переименованием,
//! чтобы соответствовать ключам сервера в `camelCase`.
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Успешный ответ сервиса аутентификации (согласно спецификации).
pub struct LoginResponse {
    pub message: String,
    pub user: crate::network::api::types::users::UserResponse,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Запрос на регистрацию
pub struct RegisterRequest {
    pub login: String,
    pub username: String,
    pub password: String,
    pub pkebymk: String, // публичный ключ, зашифрованный мастер-ключом
    pub pkebyrk: String, // публичный ключ, зашифрованный ключом восстановления
    pub salt: String, // соль для криптографии
    pub pk: String, // публичный ключ
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Запрос на логин
pub struct LoginRequest {
    pub login: String,
    pub password: String,
    pub remember_me: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct VerifyRequest {
    pub token: String,
}

#[derive(thiserror::Error, Debug)]
/// Ошибки домена аутентификации, которые возвращаются пользователю.
pub enum AuthError {
    /// Сервер вернул 401 Unauthorized — неверные учётные данные.
    #[error("unauthorized")]
    Unauthorized,
    /// Сервер вернул 404 Not Found — пользователь не найден.
    #[error("user not found")]
    NotFound,
    /// Сетевые/HTTP ошибки верхнего уровня (включая ошибки разбора JSON).
    #[error(transparent)]
    Http(#[from] reqwest::Error),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct VerifyResponse {
    pub user_id: i128,
}
