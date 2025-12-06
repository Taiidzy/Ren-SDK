//! Клиент API пользователей для Ren SDK

use crate::client::RenClient;
use crate::error::SdkError;
use crate::network::api::types::users::{PublicKeyResponse, UpdateUsernameRequest, UserResponse};
use reqwest::multipart;
use reqwest::StatusCode;

/// Получить профиль текущего пользователя
#[cfg(feature = "native")]
pub async fn get_me(client: &RenClient) -> Result<UserResponse, SdkError> {
    let response = client
        .authorized_request(reqwest::Method::GET, "/users/me")
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
        StatusCode::NOT_FOUND => {
            return Err(SdkError::from_status(
                StatusCode::NOT_FOUND,
                "Пользователь не найден".to_string(),
            ));
        }
        _ => {
            let error_msg = response.text().await.unwrap_or_else(|_| "Server error".to_string());
            return Err(SdkError::from_status(status, error_msg));
        }
    }

    let user: UserResponse = response.json().await?;
    Ok(user)
}

/// Изменить имя пользователя
pub async fn update_username(
    client: &RenClient,
    req: UpdateUsernameRequest,
) -> Result<UserResponse, SdkError> {
    let response = client
        .authorized_request(reqwest::Method::PATCH, "/users/username")
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
        StatusCode::CONFLICT => {
            return Err(SdkError::from_status(
                StatusCode::CONFLICT,
                "Имя пользователя уже занято".to_string(),
            ));
        }
        _ => {
            let error_msg = response.text().await.unwrap_or_else(|_| "Server error".to_string());
            return Err(SdkError::from_status(status, error_msg));
        }
    }

    let user: UserResponse = response.json().await?;
    Ok(user)
}

/// Обновить аватар пользователя
pub async fn update_avatar(
    client: &RenClient,
    avatar_data: Option<Vec<u8>>,
    avatar_filename: Option<String>,
    remove: bool,
) -> Result<UserResponse, SdkError> {
    let mut form = multipart::Form::new();

    if remove {
        form = form.text("remove", "true");
    } else if let (Some(data), Some(filename)) = (avatar_data, avatar_filename) {
        let part = multipart::Part::bytes(data)
            .file_name(filename)
            .mime_str("image/jpeg")?;
        form = form.part("avatar", part);
    }

    let response = client
        .authorized_request(reqwest::Method::PATCH, "/users/avatar")
        .await?
        .multipart(form)
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

    let user: UserResponse = response.json().await?;
    Ok(user)
}

/// Получить файл аватара
pub async fn get_avatar(client: &RenClient, path: &str) -> Result<Vec<u8>, SdkError> {
    let response = client
        .http_client()
        .get(format!("{}/avatars/{}", client.base_url(), path))
        .send()
        .await?;

    let status = response.status();
    match status {
        StatusCode::OK => {}
        StatusCode::BAD_REQUEST => {
            return Err(SdkError::from_status(
                StatusCode::BAD_REQUEST,
                "Некорректный путь".to_string(),
            ));
        }
        StatusCode::NOT_FOUND => {
            return Err(SdkError::from_status(
                StatusCode::NOT_FOUND,
                "Файл не найден".to_string(),
            ));
        }
        _ => {
            let error_msg = response.text().await.unwrap_or_else(|_| "Server error".to_string());
            return Err(SdkError::from_status(status, error_msg));
        }
    }

    let bytes = response.bytes().await?.to_vec();
    Ok(bytes)
}

/// Удалить аккаунт текущего пользователя
pub async fn delete_me(client: &RenClient) -> Result<(), SdkError> {
    let response = client
        .authorized_request(reqwest::Method::DELETE, "/users/me")
        .await?
        .send()
        .await?;

    let status = response.status();
    match status {
        StatusCode::NO_CONTENT => {
            #[cfg(feature = "native")]
            client.clear().await;
            #[cfg(not(feature = "native"))]
            client.clear();
            Ok(())
        }
        StatusCode::UNAUTHORIZED => {
            return Err(SdkError::from_status(
                StatusCode::UNAUTHORIZED,
                "Нет/невалидный токен".to_string(),
            ));
        }
        _ => {
            let error_msg = response.text().await.unwrap_or_else(|_| "Server error".to_string());
            Err(SdkError::from_status(status, error_msg))
        }
    }
}

/// Получить публичный ключ пользователя
pub async fn get_public_key(client: &RenClient, user_id: i64) -> Result<PublicKeyResponse, SdkError> {
    let response = client
        .http_client()
        .get(format!("{}/users/{}/public-key", client.base_url(), user_id))
        .send()
        .await?;

    let status = response.status();
    match status {
        StatusCode::OK => {}
        StatusCode::BAD_REQUEST => {
            return Err(SdkError::from_status(
                StatusCode::BAD_REQUEST,
                "Некорректный ID пользователя".to_string(),
            ));
        }
        StatusCode::NOT_FOUND => {
            return Err(SdkError::from_status(
                StatusCode::NOT_FOUND,
                "Пользователь не найден или публичный ключ отсутствует".to_string(),
            ));
        }
        _ => {
            let error_msg = response.text().await.unwrap_or_else(|_| "Server error".to_string());
            return Err(SdkError::from_status(status, error_msg));
        }
    }

    let key: PublicKeyResponse = response.json().await?;
    Ok(key)
}

