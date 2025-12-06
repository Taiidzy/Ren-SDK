//! Основной клиент SDK для работы с API мессенджера.
//!
//! RenClient управляет состоянием (токен, базовый URL, HTTP клиент) и предоставляет
//! методы для работы с API: аутентификация, пользователи, чаты, WebSocket.

use reqwest::{Client, ClientBuilder};
use std::sync::Arc;
#[cfg(feature = "native")]
use tokio::sync::RwLock;
#[cfg(not(feature = "native"))]
use std::sync::RwLock;

/// Основной клиент SDK
#[derive(Clone)]
pub struct RenClient {
    /// HTTP клиент для API запросов
    http_client: Client,
    /// Базовый URL API сервера
    base_url: String,
    /// JWT токен авторизации
    #[cfg(feature = "native")]
    token: Arc<tokio::sync::RwLock<Option<String>>>,
    #[cfg(not(feature = "native"))]
    token: Arc<std::sync::RwLock<Option<String>>>,
    /// ID текущего пользователя
    #[cfg(feature = "native")]
    user_id: Arc<tokio::sync::RwLock<Option<i64>>>,
    #[cfg(not(feature = "native"))]
    user_id: Arc<std::sync::RwLock<Option<i64>>>,
    /// Приватный ключ пользователя (для E2EE)
    #[cfg(feature = "native")]
    private_key: Arc<tokio::sync::RwLock<Option<String>>>,
    #[cfg(not(feature = "native"))]
    private_key: Arc<std::sync::RwLock<Option<String>>>,
}

impl RenClient {
    /// Создаёт новый клиент SDK с указанным базовым URL
    pub fn new(base_url: impl Into<String>) -> Self {
        let client = ClientBuilder::new()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            http_client: client,
            base_url: base_url.into(),
            #[cfg(feature = "native")]
            token: Arc::new(tokio::sync::RwLock::new(None)),
            #[cfg(not(feature = "native"))]
            token: Arc::new(std::sync::RwLock::new(None)),
            #[cfg(feature = "native")]
            user_id: Arc::new(tokio::sync::RwLock::new(None)),
            #[cfg(not(feature = "native"))]
            user_id: Arc::new(std::sync::RwLock::new(None)),
            #[cfg(feature = "native")]
            private_key: Arc::new(tokio::sync::RwLock::new(None)),
            #[cfg(not(feature = "native"))]
            private_key: Arc::new(std::sync::RwLock::new(None)),
        }
    }

    /// Устанавливает токен авторизации
    #[cfg(feature = "native")]
    pub async fn set_token(&self, token: String) {
        *self.token.write().await = Some(token);
    }
    #[cfg(not(feature = "native"))]
    pub fn set_token(&self, token: String) {
        *self.token.write().unwrap() = Some(token);
    }

    /// Получает текущий токен
    #[cfg(feature = "native")]
    pub async fn get_token(&self) -> Option<String> {
        self.token.read().await.clone()
    }
    #[cfg(not(feature = "native"))]
    pub fn get_token(&self) -> Option<String> {
        self.token.read().unwrap().clone()
    }

    /// Устанавливает ID пользователя
    #[cfg(feature = "native")]
    pub async fn set_user_id(&self, user_id: i64) {
        *self.user_id.write().await = Some(user_id);
    }
    #[cfg(not(feature = "native"))]
    pub fn set_user_id(&self, user_id: i64) {
        *self.user_id.write().unwrap() = Some(user_id);
    }

    /// Получает ID пользователя
    #[cfg(feature = "native")]
    pub async fn get_user_id(&self) -> Option<i64> {
        *self.user_id.read().await
    }
    #[cfg(not(feature = "native"))]
    pub fn get_user_id(&self) -> Option<i64> {
        *self.user_id.read().unwrap()
    }

    /// Устанавливает приватный ключ
    #[cfg(feature = "native")]
    pub async fn set_private_key(&self, private_key: String) {
        *self.private_key.write().await = Some(private_key);
    }
    #[cfg(not(feature = "native"))]
    pub fn set_private_key(&self, private_key: String) {
        *self.private_key.write().unwrap() = Some(private_key);
    }

    /// Получает приватный ключ
    #[cfg(feature = "native")]
    pub async fn get_private_key(&self) -> Option<String> {
        self.private_key.read().await.clone()
    }
    #[cfg(not(feature = "native"))]
    pub fn get_private_key(&self) -> Option<String> {
        self.private_key.read().unwrap().clone()
    }

    /// Очищает состояние (выход из аккаунта)
    #[cfg(feature = "native")]
    pub async fn clear(&self) {
        *self.token.write().await = None;
        *self.user_id.write().await = None;
        *self.private_key.write().await = None;
    }
    #[cfg(not(feature = "native"))]
    pub fn clear(&self) {
        *self.token.write().unwrap() = None;
        *self.user_id.write().unwrap() = None;
        *self.private_key.write().unwrap() = None;
    }

    /// Получает HTTP клиент
    pub fn http_client(&self) -> &Client {
        &self.http_client
    }

    /// Получает базовый URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Создаёт HTTP запрос с авторизацией
    #[cfg(feature = "native")]
    pub async fn authorized_request(&self, method: reqwest::Method, path: &str) -> Result<reqwest::RequestBuilder, crate::SdkError> {
        let token = self.get_token().await
            .ok_or(crate::SdkError::NotAuthenticated)?;
        
        Ok(self.http_client
            .request(method, format!("{}{}", self.base_url, path))
            .header("Authorization", format!("Bearer {}", token)))
    }
    #[cfg(not(feature = "native"))]
    pub fn authorized_request(&self, method: reqwest::Method, path: &str) -> Result<reqwest::RequestBuilder, crate::SdkError> {
        let token = self.get_token()
            .ok_or(crate::SdkError::NotAuthenticated)?;
        
        Ok(self.http_client
            .request(method, format!("{}{}", self.base_url, path))
            .header("Authorization", format!("Bearer {}", token)))
    }
}

