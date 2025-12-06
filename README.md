# Ren-SDK

**Language:** Русский / English

---

## 🇷🇺 Описание

**Ren-SDK** — это SDK, написанное на **Rust**, предназначенное для создания кроссплатформенных мессенджеров с полной логикой работы, включая **сквозное шифрование (E2EE)**, чаты, звонки и управление пользователями.  
SDK используется как ядро для мессенджера **Ren**, но может быть интегрировано в любые другие проекты.

### Основная идея

Цель Ren-SDK — предоставить универсальный программный интерфейс, который можно использовать для создания UI под:
- iOS  
- Android  
- Windows  
- Linux  
- macOS  
- Web  

SDK отвечает за сетевое взаимодействие, шифрование, хранение данных, синхронизацию и управление сессиями, полностью изолируя разработчиков интерфейсов от сложной логики.

---

## Основные возможности

✅ **Реализовано:**
-  Сквозное шифрование (E2EE) с X25519, ChaCha20-Poly1305
-  Индивидуальные и групповые чаты
-  WebSocket для real-time событий (сообщения, typing, presence)
-  Передача файлов и мультимедиа (с шифрованием)
-  Управление пользователями (регистрация, вход, профиль)
-  Полная совместимость с API согласно спецификации

🚧 **Планируется:**
-  Голосовые и видеозвонки
-  Групповые звонки
-  Синхронизация между устройствами
-  Привязки для iOS/Android/Web (FFI)  

---

## Архитектура (актуально)

```
ren-sdk/
 ├── src/
 │   ├── crypto/
 │   │   ├── crypto.rs          # Основная криптология
 │   │   ├── types/             # Типы и ошибки
 │   │   │   └── mod.rs
 │   │   └── wrappers/
 │   │       └── wrapper.rs     # Высокоуровневые обёртки
 │   ├── lib.rs                 # Публичный API
 │   └── main.rs                # Небольшой CLI (ren-cli)
 ├── tests/                     # Интеграционные тесты
 └── docs/crypto.md             # Документация по крипто-модулю
```

- Примитивы: X25519 (ECDH), HKDF-SHA256, ChaCha20-Poly1305 (AEAD), PBKDF2-HMAC-SHA256, SHA-256.
- Форматы ввода/вывода: Base64.
- Пароль+соль используются ТОЛЬКО для получения мастер‑ключа (расшифровка приватного ключа при авторизации). Не используйте пароль/мастер‑ключ для шифрования пользовательских данных.

---

## Быстрый старт

### Базовое использование

```rust
use ren_sdk::{RenClient, SdkError};
use ren_sdk::network::api::auth::{login, register};
use ren_sdk::network::api::users::get_me;
use ren_sdk::network::api::chats::{create_chat, get_chats};
use ren_sdk::LoginRequest;

#[tokio::main]
async fn main() -> Result<(), SdkError> {
    // Создаём клиент SDK
    let client = RenClient::new("http://localhost:8001");
    
    // Вход в систему
    let login_req = LoginRequest {
        login: "user123".to_string(),
        password: "password".to_string(),
        remember_me: Some(false),
    };
    let login_resp = login(&client, login_req).await?;
    println!("Вход выполнен: {:?}", login_resp);
    
    // Получение профиля
    let me = get_me(&client).await?;
    println!("Мой профиль: {:?}", me);
    
    // Получение списка чатов
    let chats = get_chats(&client).await?;
    println!("Мои чаты: {:?}", chats);
    
    Ok(())
}
```

### Шифрование сообщений (E2EE)

```rust
use ren_sdk::e2ee::encrypt_message_for_recipients;
use std::collections::HashMap;

// Шифрование сообщения для нескольких получателей
let mut recipient_keys = HashMap::new();
recipient_keys.insert(2, "public_key_user_2".to_string());
recipient_keys.insert(3, "public_key_user_3".to_string());

let (encrypted_msg, envelopes) = encrypt_message_for_recipients(
    "Привет, это зашифрованное сообщение!",
    &recipient_keys,
)?;
```

### WebSocket (real-time события)

```rust
use ren_sdk::network::websocket::{WsClient, WsEvent};
use std::sync::Arc;

let client = Arc::new(RenClient::new("http://localhost:8001"));
let mut ws = WsClient::new(client);

// Подключение
ws.connect().await?;

// Инициализация с контактами
ws.init(vec![2, 3, 5]).await?;

// Присоединение к чату
ws.join_chat(123).await?;

// Отправка сообщения
ws.send_message(
    123,
    "encrypted_message".to_string(),
    "text".to_string(),
    envelopes,
    None,
).await?;

// Получение событий
while let Some(event) = ws.next_event().await {
    match event {
        WsEvent::MessageNew { chat_id, message } => {
            println!("Новое сообщение в чате {}: {:?}", chat_id, message);
        }
        WsEvent::Presence { user_id, status } => {
            println!("Пользователь {} теперь {}", user_id, status);
        }
        _ => {}
    }
}
```

---

## Roadmap

- [x] Реализация базового ядра (core)  
- [x] Добавление шифрования (crypto)  
- [x] Модуль сетевого взаимодействия (HTTP API)  
- [x] WebSocket для real-time событий
- [x] E2EE шифрование сообщений
- [x] Привязки для iOS/Android/Web (FFI)
- [ ] Локальное хранение и синхронизация  
- [ ] Звонки и групповые чаты  

---

## Сборка и тесты

- Сборка библиотеки: `cargo build`
- Тесты: `cargo test`
- CLI: `cargo run --bin ren-cli -- <cmd>`

Команды CLI (для отладки):
- `ren-cli gen-keypair`
- `ren-cli enc-msg <secret> <message>`
- `ren-cli dec-msg <secret> <cipher_b64> <nonce_b64>`

---

## Лицензия

Проект распространяется под лицензией **MIT**.  
См. файл [LICENSE](LICENSE) для подробностей.

---

## Автор

**Taiidzy**  
Автор и разработчик проекта **Ren-SDK**  
[Taiidzy](https://github.com/Taiidzy)

---

## 🇬🇧 English version

### Overview

**Ren-SDK** is a **Rust-based SDK** designed for building cross-platform messengers with full core logic, including **end-to-end encryption (E2EE)**, chats, calls, and user management.  
It is the core of the **Ren Messenger**, but can be integrated into any third-party applications.

### Supported platforms

- iOS  
- Android  
- Windows  
- Linux  
- macOS  
- Web  

### Planned features

- End-to-end encryption  
- Private and group chats  
- Voice and video calls  
- Group calls  
- File transfer and attachments  
- Contact and status management  
- Device synchronization  
- Multi-platform support  
- Modular architecture  

### Project structure (tentative)

```
ren-sdk/
 ├── core/          # Core logic
 ├── crypto/        # Encryption module
 ├── network/       # Networking and transport
 ├── storage/       # Local data management
 ├── calls/         # Voice/video calls
 ├── group/         # Group chats and calls
 └── bindings/      # Platform bindings (FFI)
```

### Roadmap

- [ ] Core implementation  
- [ ] Encryption module  
- [ ] Networking layer  
- [ ] Storage and synchronization  
- [ ] Calls and group features  
- [ ] Bindings for mobile and web  

## FFI и кроссплатформенная поддержка

SDK поддерживает использование из разных языков и платформ:

### Web (WebAssembly)

```bash
# Сборка для веба
wasm-pack build --target web --out-dir pkg --features wasm
```

```javascript
import { WasmClient } from './pkg/ren_sdk.js';

const client = new WasmClient('http://localhost:8001');
await client.login('login', 'password', false);
const profile = await client.get_me();
```

См. [examples/web/README.md](examples/web/README.md) для подробностей.

### iOS (Swift)

```bash
# Сборка для iOS
cargo build --release --target aarch64-apple-ios --features ffi
```

```swift
let sdk = RenSDK(baseURL: "http://localhost:8001")
try sdk.login(login: "user123", password: "password")
let profile = try sdk.getMe()
```

См. [examples/ios/README.md](examples/ios/README.md) для подробностей.

### Android (Kotlin/JNI)

```bash
# Сборка для Android
cargo build --release --target aarch64-linux-android --features ffi
```

```kotlin
val sdk = RenSDK.create("http://localhost:8001")
sdk.login("user123", "password")
val profile = sdk.getMe()
```

См. [examples/android/README.md](examples/android/README.md) для подробностей.

### Flutter (Dart FFI)

```bash
# Соберите нативную библиотеку для целевой платформы
cargo build --release --target <target> --features ffi
```

```dart
import 'ren_sdk.dart';

final sdk = RenSDK.create("http://localhost:8001");
sdk.login("user123", "password");
final profile = sdk.getMe();
```

См. [examples/flutter/README.md](examples/flutter/README.md) для подробностей.

### Linux/Windows (Native C)

```bash
# Linux
cargo build --release --target x86_64-unknown-linux-gnu --features ffi

# Windows
cargo build --release --target x86_64-pc-windows-msvc --features ffi
```

```c
#include "ren_sdk.h"

RenClientHandle* client = ren_sdk_client_new("http://localhost:8001");
ren_sdk_login(client, "user123", "password", 0);
```

См. [examples/native/README.md](examples/native/README.md) для подробностей.

### Генерация C заголовков

```bash
cargo install cbindgen
cbindgen --config cbindgen.toml --crate ren-sdk --output ren_sdk.h
```

## License

Released under the **MIT License**.  
See [LICENSE](LICENSE) for details.

### Author

**Taiidzy**  
Creator and maintainer of **Ren-SDK**  
[Taiidzy](https://github.com/Taiidzy)
