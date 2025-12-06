//! Клиент API аутентификации для Ren SDK

use crate::client::RenClient;
use crate::error::SdkError;
use crate::network::api::types::auth::{AuthError, LoginRequest, LoginResponse, RegisterRequest};
use crate::network::api::types::users::UserResponse;
use reqwest::multipart;
use reqwest::StatusCode;

/// Регистрация нового пользователя
pub async fn register(
    client: &RenClient,
    req: RegisterRequest,
    avatar_data: Option<Vec<u8>>,
    avatar_filename: Option<String>,
) -> Result<UserResponse, SdkError> {
    let mut form = multipart::Form::new()
        .text("login", req.login)
        .text("username", req.username)
        .text("password", req.password)
        .text("pkebymk", req.pkebymk)
        .text("pkebyrk", req.pkebyrk)
        .text("salt", req.salt)
        .text("pk", req.pk);

    if let (Some(data), Some(filename)) = (avatar_data, avatar_filename) {
        let part = multipart::Part::bytes(data)
            .file_name(filename)
            .mime_str("image/jpeg")?;
        form = form.part("avatar", part);
    }

    let response = client
        .http_client()
        .post(format!("{}/auth/register", client.base_url()))
        .multipart(form)
        .send()
        .await?;

    let status = response.status();
    match status {
        StatusCode::BAD_REQUEST => {
            let error_msg = response.text().await.unwrap_or_else(|_| "Bad Request".to_string());
            return Err(SdkError::from_status(StatusCode::BAD_REQUEST, error_msg));
        }
        StatusCode::CONFLICT => {
            let error_msg = response.text().await.unwrap_or_else(|_| "Conflict".to_string());
            return Err(SdkError::from_status(StatusCode::CONFLICT, error_msg));
        }
        StatusCode::OK => {}
        _ => {
            let error_msg = response.text().await.unwrap_or_else(|_| "Server error".to_string());
            return Err(SdkError::from_status(status, error_msg));
        }
    }

    let user: UserResponse = response.json().await?;
    Ok(user)
}

/// Аутентификация по логину/паролю
pub async fn login(client: &RenClient, req: LoginRequest) -> Result<LoginResponse, SdkError> {
    let response = client
        .http_client()
        .post(format!("{}/auth/login", client.base_url()))
        .json(&req)
        .send()
        .await?;

    let status = response.status();
    match status {
        StatusCode::UNAUTHORIZED => {
            return Err(SdkError::from_status(
                StatusCode::UNAUTHORIZED,
                "Неверный логин или пароль".to_string(),
            ));
        }
        StatusCode::OK => {}
        _ => {
            let error_msg = response.text().await.unwrap_or_else(|_| "Server error".to_string());
            return Err(SdkError::from_status(status, error_msg));
        }
    }

    let body: LoginResponse = response.json().await?;
    
    // Сохраняем токен и ID пользователя в клиенте
    #[cfg(feature = "native")]
    {
        client.set_token(body.token.clone()).await;
        client.set_user_id(body.user.id).await;
    }
    #[cfg(not(feature = "native"))]
    {
        client.set_token(body.token.clone());
        client.set_user_id(body.user.id);
    }
    
    Ok(body)
}

/// Старая функция login для обратной совместимости (использует старый API)
pub async fn login_legacy(req: LoginRequest) -> Result<LoginResponse, AuthError> {
    use reqwest::StatusCode;
    
    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:8001/auth-service/auth/login")
        .json(&req)
        .send()
        .await?;

    match response.status() {
        StatusCode::UNAUTHORIZED => return Err(AuthError::Unauthorized),
        StatusCode::NOT_FOUND => return Err(AuthError::NotFound),
        _ => {}
    }

    let body: LoginResponse = response.json().await?;
    Ok(body)
}
