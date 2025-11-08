# Crypto module (Ren-SDK)
## Обёртки (wrapper)

- `encrypt_message_with_secret(secret: &str, message: &str) -> EncryptedMessage`
- `decrypt_message_with_secret(secret: &str, ciphertext_b64: &str, nonce_b64: &str) -> String`
- `derive_master_key_from_password(password: &str, salt_b64: &str) -> AeadKey`
- `derive_master_key_b64_from_password(password: &str, salt_b64: &str) -> String`

### Примеры (wrapper)

```rust
use ren_sdk::{encrypt_message_with_secret, decrypt_message_with_secret};

let enc = encrypt_message_with_secret("my-secret", "hello")?;
let dec = decrypt_message_with_secret("my-secret", &enc.ciphertext, &enc.nonce)?;
assert_eq!(dec, "hello");
```

Деривация мастер-ключа из пароля и соли:

```rust
use ren_sdk::{derive_master_key_from_password, derive_master_key_b64_from_password};

let salt_b64 = ren_sdk::generate_salt();
let aead = derive_master_key_from_password("P@ssw0rd", &salt_b64)?; // готовый ключ
let key_b64 = derive_master_key_b64_from_password("P@ssw0rd", &salt_b64)?; // экспорт 32 байт Base64
```


- **Примитивы (Rust):** X25519 (ECDH), HKDF-SHA256, ChaCha20-Poly1305 (AEAD), PBKDF2-HMAC-SHA256, SHA-256
- **Совместимость семантики:** Сохранены форматы ввода/вывода из TS-версии (Base64), структуры результатов и разделение nonce/iv.
- **Назначение:** Симметричное шифрование сообщений и файлов, обертывание симметричных ключей через ECDH, деривация ключей из строки и получение мастер‑ключа из пароля+соли для расшифровки приватного ключа при авторизации.

## Маппинг TS → Rust

- TS `AES-GCM` → Rust `ChaCha20-Poly1305` (AEAD), тот же размер nonce = 12 байт, строки Base64.
- TS `ECDH P-256` → Rust `X25519` (сырые ключи 32 байта, Base64).
- TS SPKI/PKCS#8 → Rust хранит ключи в RAW (Base64-32байта). Экспорт/импорт: `export_*_b64`/`import_*_b64`.
- Salt/Nonce → 16/12 байт соответственно, возвращаются как Base64.

## Публичное API (основные функции)

- **Ключи и деривации**
  - `generate_key_pair(extractable: bool) -> KeyPair`
  - `export_public_key_b64(&X25519PublicKey) -> String`
  - `export_private_key_b64(&StaticSecret) -> String`
  - `import_public_key_b64(b64: &str) -> X25519PublicKey`
  - `import_private_key_b64(b64: &str) -> StaticSecret`
  - `derive_key_from_password(password: &str, salt_b64: &str) -> AeadKey` (PBKDF2, 100k)
  - `derive_key_from_string(secret: &str) -> AeadKey` (SHA-256(secret)[0..32])
  - `generate_message_encryption_key() -> AeadKey`
  - `generate_salt() -> String` (Base64-16)
  - `generate_nonce() -> String` (Base64-12)

- **Симметричное шифрование**
  - `encrypt_data(plain: &str, key: &AeadKey) -> String` → Base64(iv||cipher)
  - `decrypt_data(b64: &str, key: &AeadKey) -> String`
  - `encrypt_message(plain: &str, key: &AeadKey) -> EncryptedMessage { ciphertext, nonce }`
  - `decrypt_message(cipher_b64: &str, nonce_b64: &str, key: &AeadKey) -> String`
  - `encrypt_file(bytes: &[u8], filename, mimetype, key) -> EncryptedFile`
  - `decrypt_file(cipher_b64, nonce_b64, key) -> Vec<u8>`
  - `encrypt_file_with_message(bytes, message, key, filename, mimetype) -> EncryptedFileWithMessage`
  - `decrypt_file_with_message(enc_file_b64, ciphertext_b64, nonce_b64, key, filename, mimetype) -> DecryptedFileWithMessage`

- **Обертка ключа (ECDH + AEAD)**
  - `wrap_symmetric_key(key_to_wrap: &AeadKey, receiver_public_key_b64: &str) -> (wrappedKey_b64, ephemeralPublicKey_b64, nonce_b64)`
  - `unwrap_symmetric_key(wrappedKey_b64, ephemeralPublicKey_b64, nonce_b64, receiver_private_key_b64) -> AeadKey`


## Примеры

### 1) Деривация из строки и шифрование сообщения

```rust
use ren_sdk::crypto::*;

let key = derive_key_from_string("my-access-key").unwrap();
let enc = encrypt_message("hello", &key).unwrap();
let plain = decrypt_message(&enc.ciphertext, &enc.nonce, &key).unwrap();
assert_eq!(plain, "hello");
```

Примечание по паролю и соли: пароль используется ТОЛЬКО для получения мастер‑ключа (`derive_master_key_from_password`/`derive_master_key_b64_from_password`).
Этот мастер‑ключ предназначен для расшифровки приватного ключа, который возвращает сервер во время авторизации пользователя. Он не предназначен для шифрования/дешифрования сообщений или файлов.

### 3) Обмен ключом: wrap/unwrap симметричного ключа

```rust
use ren_sdk::crypto::*;

// получатель генерирует свою пару X25519
let receiver = generate_key_pair(false);
let msg_key = generate_message_encryption_key();
// отправитель оборачивает ключ для получателя
let (wrapped, eph_pub, nonce) = wrap_symmetric_key(&msg_key, &receiver.public_key).unwrap();
// получатель разворачивает
let unwrapped = unwrap_symmetric_key(&wrapped, &eph_pub, &nonce, &receiver.private_key).unwrap();
// проверка
let em = encrypt_message("secret", &msg_key).unwrap();
let dec = decrypt_message(&em.ciphertext, &em.nonce, &unwrapped).unwrap();
assert_eq!(dec, "secret");
```

### 4) Файлы и сообщение

```rust
use ren_sdk::crypto::*;

let key = generate_message_encryption_key();
let data = b"file-bytes".to_vec();
let efm = encrypt_file_with_message(&data, "caption", &key, "note.txt", "text/plain").unwrap();
let out = decrypt_file_with_message(&efm.enc_file, &efm.ciphertext, &efm.nonce, &key, &efm.filename, &efm.mimetype).unwrap();
assert_eq!(out.file, data);
assert_eq!(out.message, "caption");
```

## CLI

В репозитории есть простой CLI (для отладки):

- `ren-sdk gen-keypair`
- `ren-sdk enc-msg <secret> <message>`
- `ren-sdk dec-msg <secret> <cipher_b64> <nonce_b64>`

Пример:

```
ren-sdk enc-msg mysecret "hi"
# -> ciphertext_b64: ...
# -> nonce_b64: ...
ren-sdk dec-msg mysecret <cipher_b64> <nonce_b64>
```

## Поддерживаемые платформы

- Android, iOS, Windows, macOS, Linux.
- Для Web целевой порт обеспечивается за счет совместимых форматов (Base64),
  но внутри Rust используются X25519 + ChaCha20-Poly1305. Для использования в WASM
  предлагется предоставить JS/TS обертки, совместимые по формату.

## Соображения безопасности

- Nonce (12 байт) должен быть уникален для каждого шифрования под одним ключом.
  В библиотеке nonce генерируется случайно, и повторное использование одной и той же пары (key, nonce) недопустимо.
- `wrap_symmetric_key` использует X25519 ECDH и HKDF для получения ключа обертки, затем AEAD.
- Чувствительные ключи следует хранить безопасно на платформе (Keystore/Keychain и т. д.).
- PBKDF2 использует 100k итераций — при необходимости можно увеличить параметр.

## Отличия от TS-версии

- В TS: P-256 + AES-GCM; в Rust: X25519 + ChaCha20-Poly1305. Внешние форматы (Base64, структуры) сохранены.
- В TS ключи публичный/приватный экспортируются как SPKI/PKCS#8; в Rust — сырые 32 байта (Base64).

