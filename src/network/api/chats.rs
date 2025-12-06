//! Клиент API чатов для Ren SDK

use crate::client::RenClient;
use crate::error::SdkError;
use crate::network::api::types::chats::{Chat, CreateChatRequest};
use crate::network::api::types::messages::Message;
use reqwest::StatusCode;

/// Создать чат
pub async fn create_chat(client: &RenClient, req: CreateChatRequest) -> Result<Chat, SdkError> {
    let response = client
        .authorized_request(reqwest::Method::POST, "/chats")
        .await?
        .json(&req)
        .send()
        .await?;

    let status = response.status();
    match status {
        StatusCode::OK => {}
        StatusCode::BAD_REQUEST => {
            let error_msg = response.text().await.unwrap_or_else(|_| "Bad Request".to_string());
            return Err(SdkError::from_status(StatusCode::BAD_REQUEST, error_msg));
        }
        StatusCode::UNAUTHORIZED => {
            return Err(SdkError::from_status(
                StatusCode::UNAUTHORIZED,
                "Нет/невалидный токен".to_string(),
            ));
        }
        _ => {
            let error_msg = response.text().await.unwrap_or_else(|_| "Server error".to_string());
            return Err(SdkError::from_status(status, error_msg));
        }
    }

    let chat: Chat = response.json().await?;
    Ok(chat)
}

/// Получить список чатов
pub async fn get_chats(client: &RenClient) -> Result<Vec<Chat>, SdkError> {
    let response = client
        .authorized_request(reqwest::Method::GET, "/chats")
        .await?
        .send()
        .await?;

    let status = response.status();
    match status {
        StatusCode::OK => {}
        StatusCode::UNAUTHORIZED => {
            return Err(SdkError::from_status(
                StatusCode::UNAUTHORIZED,
                "Нет/невалидный токен".to_string(),
            ));
        }
        _ => {
            let error_msg = response.text().await.unwrap_or_else(|_| "Server error".to_string());
            return Err(SdkError::from_status(status, error_msg));
        }
    }

    let chats: Vec<Chat> = response.json().await?;
    Ok(chats)
}

/// Получить сообщения чата
pub async fn get_messages(
    client: &RenClient,
    chat_id: i64,
) -> Result<Vec<Message>, SdkError> {
    let response = client
        .authorized_request(reqwest::Method::GET, &format!("/chats/{}/messages", chat_id))
        .await?
        .send()
        .await?;

    let status = response.status();
    match status {
        StatusCode::OK => {}
        StatusCode::UNAUTHORIZED => {
            return Err(SdkError::from_status(
                StatusCode::UNAUTHORIZED,
                "Нет/невалидный токен".to_string(),
            ));
        }
        StatusCode::FORBIDDEN => {
            return Err(SdkError::from_status(
                StatusCode::FORBIDDEN,
                "Нет доступа (пользователь не участник чата)".to_string(),
            ));
        }
        _ => {
            let error_msg = response.text().await.unwrap_or_else(|_| "Server error".to_string());
            return Err(SdkError::from_status(status, error_msg));
        }
    }

    let messages: Vec<Message> = response.json().await?;
    Ok(messages)
}

/// Удалить чат или выйти из чата
pub async fn delete_chat(
    client: &RenClient,
    chat_id: i64,
    for_all: Option<bool>,
) -> Result<(), SdkError> {
    let mut url = format!("/chats/{}", chat_id);
    if let Some(for_all) = for_all {
        url = format!("{}?for_all={}", url, for_all);
    }

    let response = client
        .authorized_request(reqwest::Method::DELETE, &url)
        .await?
        .send()
        .await?;

    let status = response.status();
    match status {
        StatusCode::NO_CONTENT => Ok(()),
        StatusCode::UNAUTHORIZED => {
            Err(SdkError::from_status(
                StatusCode::UNAUTHORIZED,
                "Нет/невалидный токен".to_string(),
            ))
        }
        StatusCode::FORBIDDEN => {
            Err(SdkError::from_status(
                StatusCode::FORBIDDEN,
                "Нет прав (для group) / Не участник (для private)".to_string(),
            ))
        }
        StatusCode::NOT_FOUND => {
            Err(SdkError::from_status(
                StatusCode::NOT_FOUND,
                "Чат не найден".to_string(),
            ))
        }
        _ => {
            let error_msg = response.text().await.unwrap_or_else(|_| "Server error".to_string());
            Err(SdkError::from_status(status, error_msg))
        }
    }
}

