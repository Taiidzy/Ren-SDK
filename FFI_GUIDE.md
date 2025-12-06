# Руководство по FFI и кроссплатформенной сборке

Это руководство описывает, как собрать и использовать Ren SDK для разных платформ.

## Обзор

Ren SDK поддерживает три основных способа использования:

1. **Rust** - нативное использование из Rust кода
2. **WebAssembly** - для веб-приложений (JavaScript/TypeScript)
3. **C FFI** - для нативных платформ (iOS/Android через Swift/Kotlin)

## Сборка для Web (WebAssembly)

### Требования

```bash
# Установите wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Установите Rust toolchain для wasm
rustup target add wasm32-unknown-unknown
```

### Сборка

```bash
cd Ren-SDK
wasm-pack build --target web --out-dir examples/web/pkg --features wasm
```

Это создаст:
- `pkg/ren_sdk.js` - JavaScript обёртка
- `pkg/ren_sdk_bg.wasm` - WebAssembly модуль
- `pkg/ren_sdk.d.ts` - TypeScript определения

### Использование

```javascript
import init, { WasmClient } from './pkg/ren_sdk.js';

await init(); // Инициализация WASM

const client = new WasmClient('http://localhost:8001');
await client.login('login', 'password', false);
const profile = await client.get_me();
```

См. [examples/web/](examples/web/) для полного примера.

## Сборка для iOS

### Требования

- Xcode с Command Line Tools
- Rust toolchain

### Сборка

```bash
cd Ren-SDK

# Для реального устройства (arm64)
cargo build --release --target aarch64-apple-ios --features ffi

# Для симулятора (x86_64)
cargo build --release --target x86_64-apple-ios --features ffi

# Создайте универсальную библиотеку
lipo -create \
  target/aarch64-apple-ios/release/libren_sdk.a \
  target/x86_64-apple-ios/release/libren_sdk.a \
  -output libren_sdk_universal.a
```

### Генерация C заголовков

```bash
cargo install cbindgen
cbindgen --config cbindgen.toml --crate ren-sdk --output ren_sdk.h
```

### Интеграция в Xcode

1. Добавьте `libren_sdk_universal.a` в проект
2. Добавьте `ren_sdk.h` в проект
3. Добавьте `RenSDK.swift` из `examples/ios/`
4. В Build Settings добавьте путь к заголовкам

### Использование

```swift
import Foundation

let sdk = RenSDK(baseURL: "http://localhost:8001")
try sdk.login(login: "user123", password: "password")
let profile = try sdk.getMe()
```

См. [examples/ios/](examples/ios/) для полного примера.

## Сборка для Flutter

### Требования

- Flutter SDK (>=3.0.0)
- Rust toolchain
- Нативная библиотека для целевой платформы

### Сборка

Соберите нативную библиотеку для нужной платформы (см. соответствующие разделы), затем:

```bash
# Скопируйте библиотеку в Flutter проект
# Windows: ren_sdk.dll
# Linux: libren_sdk.so
# macOS: libren_sdk.dylib
# Android: libren_sdk.so в jniLibs
# iOS: libren_sdk.a в Xcode проект
```

### Использование

```dart
import 'ren_sdk.dart';

final sdk = RenSDK.create('http://localhost:8001');
sdk.login('user123', 'password');
final profile = sdk.getMe();
```

См. [examples/flutter/](examples/flutter/) для полного примера.

## Сборка для Linux/Windows (Native)

### Linux

```bash
cargo build --release --target x86_64-unknown-linux-gnu --features ffi
```

### Windows

```bash
cargo build --release --target x86_64-pc-windows-msvc --features ffi
```

### Использование

```c
#include "ren_sdk.h"

RenClientHandle* client = ren_sdk_client_new("http://localhost:8001");
ren_sdk_login(client, "user123", "password", 0);
char* profile = ren_sdk_get_me(client);
ren_sdk_client_free(client);
```

См. [examples/native/](examples/native/) для полных примеров.

## Сборка для Android

### Требования

- Android NDK
- Rust toolchain
- `cargo-ndk` (опционально, но рекомендуется)

```bash
cargo install cargo-ndk
```

### Сборка

```bash
cd Ren-SDK

# Для разных архитектур Android
cargo ndk --target aarch64-linux-android --platform 21 build --release --features ffi
cargo ndk --target armv7-linux-androideabi --platform 21 build --release --features ffi
cargo ndk --target x86_64-linux-android --platform 21 build --release --features ffi
cargo ndk --target i686-linux-android --platform 21 build --release --features ffi
```

### Интеграция в Android проект

1. Скопируйте `.so` файлы в `app/src/main/jniLibs/<arch>/`
2. Добавьте `RenSDK.kt` из `examples/android/` в проект
3. Создайте JNI обёртку (см. примеры)

### Использование

```kotlin
val sdk = RenSDK.create("http://localhost:8001")
sdk.login("user123", "password")
val profile = sdk.getMe()
```

См. [examples/android/](examples/android/) для полного примера.

## Автоматическая сборка

Используйте скрипт `build.sh` для автоматической сборки всех платформ:

```bash
chmod +x build.sh
./build.sh
```

## Структура FFI API

### C FFI функции

Все функции начинаются с префикса `ren_sdk_`:

- `ren_sdk_client_new(base_url)` - создаёт клиент
- `ren_sdk_client_free(handle)` - освобождает клиент
- `ren_sdk_login(handle, login, password, remember_me)` - вход
- `ren_sdk_get_me(handle)` - получает профиль (JSON)
- `ren_sdk_get_chats(handle)` - получает чаты (JSON)
- `ren_sdk_generate_keypair()` - генерирует ключи (JSON)
- `ren_sdk_generate_salt()` - генерирует соль
- `ren_sdk_free_string(ptr)` - освобождает строку

### WebAssembly API

- `WasmClient` - основной класс клиента
- `wasm_generate_keypair()` - генерирует ключи
- `wasm_generate_salt()` - генерирует соль
- `wasm_encrypt_message_for_recipients()` - шифрует сообщение
- `wasm_decrypt_message_with_envelope()` - расшифровывает сообщение

## Устранение неполадок

### WebAssembly

- Убедитесь, что используете `--target web` (не `bundler`)
- Проверьте, что все зависимости поддерживают wasm

### iOS

- Убедитесь, что архитектура соответствует целевой платформе
- Проверьте, что библиотека добавлена в "Link Binary With Libraries"

### Android

- Убедитесь, что NDK правильно настроен
- Проверьте, что все архитектуры собраны
- Убедитесь, что JNI функции правильно экспортированы

## Дополнительные ресурсы

- [wasm-pack документация](https://rustwasm.github.io/wasm-pack/)
- [cbindgen документация](https://github.com/eqrion/cbindgen)
- [Rust FFI руководство](https://doc.rust-lang.org/nomicon/ffi.html)

