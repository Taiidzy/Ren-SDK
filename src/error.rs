//! Типы ошибок SDK

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SdkError {
    /// Пользователь не аутентифицирован
    #[error("Not authenticated")]
    NotAuthenticated,
    
    /// HTTP ошибка
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    
    /// Ошибка сериализации/десериализации
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    /// Ошибка криптографии
    #[error("Crypto error: {0}")]
    Crypto(#[from] crate::CryptoError),
    
    /// Ошибка API (400, 401, 403, 404, 409, 500)
    #[error("API error: {0}")]
    Api(String),
    
    /// Ошибка WebSocket
    #[error("WebSocket error: {0}")]
    WebSocket(String),
    
    /// Ошибка HTTP
    #[error("HTTP error: {0}")]
    HttpError(#[from] http::Error),
    
    /// Другая ошибка
    #[error("{0}")]
    Other(String),
}

impl SdkError {
    pub fn from_status(status: reqwest::StatusCode, message: String) -> Self {
        match status {
            reqwest::StatusCode::UNAUTHORIZED => SdkError::Api(format!("Unauthorized: {}", message)),
            reqwest::StatusCode::FORBIDDEN => SdkError::Api(format!("Forbidden: {}", message)),
            reqwest::StatusCode::NOT_FOUND => SdkError::Api(format!("Not Found: {}", message)),
            reqwest::StatusCode::BAD_REQUEST => SdkError::Api(format!("Bad Request: {}", message)),
            reqwest::StatusCode::CONFLICT => SdkError::Api(format!("Conflict: {}", message)),
            _ => SdkError::Api(format!("Server error ({}): {}", status, message)),
        }
    }
}

