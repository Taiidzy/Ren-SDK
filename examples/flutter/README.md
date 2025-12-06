# Ren SDK - Flutter Example

Пример использования Ren SDK в Flutter приложении через Dart FFI.

## Требования

- Flutter SDK (>=3.0.0)
- Rust toolchain
- Нативная библиотека Ren SDK

## Сборка нативной библиотеки

### Windows

```bash
cd Ren-SDK
cargo build --release --target x86_64-pc-windows-msvc --features ffi
# Скопируйте ren_sdk.dll в flutter проект
```

### Linux

```bash
cd Ren-SDK
cargo build --release --target x86_64-unknown-linux-gnu --features ffi
# Скопируйте libren_sdk.so в flutter проект
```

### macOS

```bash
cd Ren-SDK
cargo build --release --target x86_64-apple-darwin --features ffi
# Скопируйте libren_sdk.dylib в flutter проект
```

### Android

```bash
cd Ren-SDK
cargo ndk --target aarch64-linux-android --platform 21 build --release --features ffi
# Скопируйте libren_sdk.so в android/app/src/main/jniLibs/arm64-v8a/
```

### iOS

```bash
cd Ren-SDK
cargo build --release --target aarch64-apple-ios --features ffi
# Добавьте libren_sdk.a в Xcode проект
```

## Настройка проекта

1. Добавьте нативную библиотеку в проект:
   - **Windows/Linux/macOS**: в корень проекта или в `lib/`
   - **Android**: в `android/app/src/main/jniLibs/<arch>/`
   - **iOS**: добавьте в Xcode проект

2. Установите зависимости:
```bash
flutter pub get
```

3. Для Android добавьте в `android/app/build.gradle`:
```gradle
android {
    ...
    sourceSets {
        main {
            jniLibs.srcDirs = ['src/main/jniLibs']
        }
    }
}
```

## Использование

```dart
import 'ren_sdk_simple.dart' as ren_sdk;

// Создание клиента
final sdk = ren_sdk.RenSDK.create('http://localhost:8001');

try {
  // Вход в систему
  sdk.login('user123', 'password', rememberMe: false);
  
  // Получение профиля
  final profile = sdk.getMe();
  print('Профиль: ${profile['username']}');
  
  // Получение чатов
  final chats = sdk.getChats();
  print('Чатов: ${chats.length}');
  
  // Генерация ключей
  final keypair = ren_sdk.RenSDKCrypto.generateKeypair();
  print('Public key: ${keypair['public_key']}');
  
  // Генерация соли
  final salt = ren_sdk.RenSDKCrypto.generateSalt();
  print('Salt: $salt');
  
} catch (e) {
  print('Ошибка: $e');
} finally {
  sdk.dispose();
}
```

## Структура проекта

```
flutter_project/
├── lib/
│   ├── ren_sdk.dart      # Dart FFI обёртка
│   └── main.dart         # Пример использования
├── android/
│   └── app/
│       └── src/
│           └── main/
│               └── jniLibs/
│                   └── arm64-v8a/
│                       └── libren_sdk.so
└── ios/
    └── libren_sdk.a
```

## Запуск

```bash
flutter run
```

## Примечания

- Убедитесь, что нативная библиотека соответствует архитектуре целевой платформы
- Для Android соберите библиотеки для всех необходимых архитектур
- Для iOS используйте универсальную библиотеку (fat binary) или соберите отдельно для устройства и симулятора

