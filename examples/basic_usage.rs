//! Пример использования Ren SDK
//!
//! Этот пример демонстрирует базовое использование SDK:
//! - Создание клиента
//! - Регистрация и вход
//! - Работа с чатами
//! - Отправка сообщений через WebSocket

use ren_sdk::{
    RenClient, SdkError,
    network::api::auth::{login, register},
    network::api::users::get_me,
    network::api::chats::{create_chat, get_chats, get_messages},
    network::api::types::auth::{LoginRequest, RegisterRequest},
    network::api::types::chats::CreateChatRequest,
    crypto::{generate_key_pair, derive_key_from_password, decrypt_message, import_private_key_b64},
    e2ee::encrypt_message_for_recipients,
};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), SdkError> {
    // Создаём клиент SDK
    let client = RenClient::new("http://localhost:8001");
    
    // Пример регистрации
    let keypair = generate_key_pair(false);
    let salt = ren_sdk::generate_salt();
    let master_key = derive_key_from_password("my_password", &salt)?;
    
    // В реальном приложении нужно зашифровать приватный ключ мастер-ключом
    // Здесь упрощённый пример
    let register_req = RegisterRequest {
        login: "user123".to_string(),
        username: "User123".to_string(),
        password: "my_password".to_string(),
        pkebymk: "encrypted_private_key_by_master_key".to_string(),
        pkebyrk: "encrypted_private_key_by_recovery_key".to_string(),
        salt: salt.clone(),
        pk: keypair.public_key.clone(),
    };
    
    // Регистрация (закомментировано, так как требует реального сервера)
    // let user = register(&client, register_req, None, None).await?;
    // println!("Зарегистрирован пользователь: {:?}", user);
    
    // Пример входа
    let login_req = LoginRequest {
        login: "user123".to_string(),
        password: "my_password".to_string(),
        remember_me: Some(false),
    };
    
    // Вход (закомментировано, так как требует реального сервера)
    // let login_resp = login(&client, login_req).await?;
    // println!("Вход выполнен: {:?}", login_resp);
    
    // Получение профиля
    // let me = get_me(&client).await?;
    // println!("Мой профиль: {:?}", me);
    
    // Создание чата
    // let create_chat_req = CreateChatRequest {
    //     kind: "private".to_string(),
    //     title: None,
    //     user_ids: vec![1, 2],
    // };
    // let chat = create_chat(&client, create_chat_req).await?;
    // println!("Создан чат: {:?}", chat);
    
    // Получение списка чатов
    // let chats = get_chats(&client).await?;
    // println!("Мои чаты: {:?}", chats);
    
    // Получение сообщений чата
    // let messages = get_messages(&client, 1).await?;
    // println!("Сообщения: {:?}", messages);
    
    // Пример шифрования сообщения для нескольких получателей
    let mut recipient_keys = HashMap::new();
    recipient_keys.insert(2, "public_key_user_2".to_string());
    recipient_keys.insert(3, "public_key_user_3".to_string());
    
    // let (encrypted_msg, envelopes) = encrypt_message_for_recipients(
    //     "Привет, это зашифрованное сообщение!",
    //     &recipient_keys,
    // )?;
    // println!("Зашифрованное сообщение: {}", encrypted_msg);
    // println!("Конверты: {:?}", envelopes);
    
    println!("Пример использования SDK готов!");
    Ok(())
}

