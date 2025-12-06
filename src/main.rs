//! Простой CLI вокруг Ren SDK.
//!
//! Поддерживаемые команды:
//! - gen-keypair
//! - enc-msg <secret> <message>
//! - dec-msg <secret> <cipher_b64> <nonce_b64>
//! - login <username> <password>
use ren_sdk::crypto::*;
use std::env;
use ren_sdk::{RenClient, SdkError};
use ren_sdk::network::api::auth::login;
use ren_sdk::LoginRequest;

fn print_usage() {
    eprintln!("Usage:\n  ren-sdk gen-keypair\n  ren-sdk enc-msg <secret> <message>\n  ren-sdk dec-msg <secret> <cipher_b64> <nonce_b64>\n  ren-sdk login <username> <password>");
}

#[tokio::main]
async fn main() {
    // Собираем аргументы CLI: args[0] — имя бинарника, args[1] — подкоманда.
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        return;
    }
    match args[1].as_str() {
        "gen-keypair" => {
            let kp = generate_key_pair(false);
            println!("public_key_b64: {}", kp.public_key);
            println!("private_key_b64: {}", kp.private_key);
        }
        "enc-msg" => {
            // Шифруем сообщение с ключом, производным от секретной строки.
            if args.len() < 4 { print_usage(); return; }
            let secret = &args[2];
            let msg = &args[3];
            let key = derive_key_from_string(secret).expect("key");
            let enc = encrypt_message(msg, &key).expect("enc");
            println!("ciphertext_b64: {}", enc.ciphertext);
            println!("nonce_b64: {}", enc.nonce);
        }
        "dec-msg" => {
            // Расшифровываем сообщение по шифртексту и nonce (оба в base64).
            if args.len() < 5 { print_usage(); return; }
            let secret = &args[2];
            let ct = &args[3];
            let nonce = &args[4];
            let key = derive_key_from_string(secret).expect("key");
            let msg = decrypt_message(ct, nonce, &key).expect("dec");
            println!("{}", msg);
        }
        "login" => {
            // Вызываем API аутентификации с переданными учётными данными и печатаем понятный результат.
            if args.len() < 4 { print_usage(); return; }
            let username = &args[2];
            let password = &args[3];
            let client = RenClient::new("http://localhost:8001");
            let req = LoginRequest { 
                login: username.clone(), 
                password: password.clone(),
                remember_me: None,
            };
            match login(&client, req).await {
                Ok(resp) => println!("Ok({:?})", resp),
                Err(SdkError::Api(msg)) if msg.contains("Unauthorized") => {
                    eprintln!("Error: 401 Unauthorized (invalid credentials)");
                }
                Err(SdkError::Api(msg)) if msg.contains("Not Found") => {
                    eprintln!("Error: 404 Not Found (user not found)");
                }
                Err(e) => eprintln!("Error: {:?}", e),
            }
        }
        _ => print_usage(),
    }
}

