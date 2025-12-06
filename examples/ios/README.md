# Ren SDK - iOS Example

Пример использования Ren SDK в iOS приложении через Swift.

## Сборка

1. Соберите статическую библиотеку для iOS:
```bash
cd Ren-SDK

# Для iOS (arm64)
cargo build --release --target aarch64-apple-ios --features ffi

# Для iOS Simulator (x86_64)
cargo build --release --target x86_64-apple-ios --features ffi

# Для универсальной библиотеки используйте lipo:
lipo -create \
  target/aarch64-apple-ios/release/libren_sdk.a \
  target/x86_64-apple-ios/release/libren_sdk.a \
  -output libren_sdk_universal.a
```

2. Добавьте библиотеку в Xcode проект:
   - Перетащите `libren_sdk_universal.a` в проект
   - Добавьте `RenSDK.swift` в проект
   - В Build Settings добавьте путь к заголовкам

3. Сгенерируйте C заголовки (опционально):
```bash
cargo install cbindgen
cbindgen --config cbindgen.toml --crate ren-sdk --output ren_sdk.h
```

## Использование

```swift
import Foundation

// Инициализация клиента
let sdk = RenSDK(baseURL: "http://localhost:8001")

// Вход в систему
do {
    try sdk.login(login: "user123", password: "password", rememberMe: false)
    print("Вход выполнен успешно")
} catch {
    print("Ошибка входа: \(error)")
}

// Получение профиля
do {
    let profile = try sdk.getMe()
    print("Профиль: \(profile)")
} catch {
    print("Ошибка получения профиля: \(error)")
}

// Получение списка чатов
do {
    let chats = try sdk.getChats()
    print("Чаты: \(chats)")
} catch {
    print("Ошибка получения чатов: \(error)")
}

// Генерация ключей
do {
    let keypair = try RenSDK.generateKeypair()
    print("Public key: \(keypair["public_key"] ?? "")")
    print("Private key: \(keypair["private_key"] ?? "")")
} catch {
    print("Ошибка генерации ключей: \(error)")
}

// Генерация соли
let salt = RenSDK.generateSalt()
print("Salt: \(salt)")
```

## Структура проекта

```
YourApp/
├── RenSDK.swift          # Swift обёртка
├── libren_sdk_universal.a  # Статическая библиотека
└── ren_sdk.h            # C заголовки (опционально)
```

