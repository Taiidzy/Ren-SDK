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

## Основные возможности (планируемые)

-  Сквозное шифрование (E2EE)  
-  Индивидуальные и групповые чаты  
-  Голосовые и видеозвонки  
-  Групповые звонки  
-  Передача файлов и мультимедиа  
-  Работа с контактами и статусами  
-  Синхронизация между устройствами  
-  Поддержка нескольких платформ  
-  Гибкая архитектура модулей  

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

```rust
use ren_sdk::{
  encrypt_message_with_secret, decrypt_message_with_secret,
  derive_master_key_from_password, generate_salt,
};

// 1) Шифрование/дешифрование сообщения из секретной строки
let enc = encrypt_message_with_secret("my-secret", "hello")?;
let dec = decrypt_message_with_secret("my-secret", &enc.ciphertext, &enc.nonce)?;
assert_eq!(dec, "hello");

// 2) Получение мастер‑ключа (для расшифровки приватного ключа, полученного с сервера)
let salt_b64 = generate_salt();
let master = derive_master_key_from_password("P@ssw0rd", &salt_b64)?;
```

---

## Roadmap

- [ ] Реализация базового ядра (core)  
- [ ] Добавление шифрования (crypto)  
- [ ] Модуль сетевого взаимодействия  
- [ ] Локальное хранение и синхронизация  
- [ ] Звонки и групповые чаты  
- [ ] Привязки для iOS/Android/Web  

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

### License

Released under the **MIT License**.  
See [LICENSE](LICENSE) for details.

### Author

**Taiidzy**  
Creator and maintainer of **Ren-SDK**  
[Taiidzy](https://github.com/Taiidzy)
