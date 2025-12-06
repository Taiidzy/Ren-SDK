# Ren SDK - Android Example

Пример использования Ren SDK в Android приложении через Kotlin/JNI.

## Сборка

1. Соберите библиотеку для Android:
```bash
cd Ren-SDK

# Для Android (arm64)
cargo build --release --target aarch64-linux-android --features ffi

# Для Android (armv7)
cargo build --release --target armv7-linux-androideabi --features ffi

# Для Android (x86_64)
cargo build --release --target x86_64-linux-android --features ffi

# Для Android (x86)
cargo build --release --target i686-linux-android --features ffi
```

2. Создайте JNI обёртку (см. `jni_wrapper.cpp`)

3. Добавьте в `build.gradle`:
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

4. Скопируйте `.so` файлы в `app/src/main/jniLibs/<arch>/`

## Использование

```kotlin
import com.ren.sdk.RenSDK

// Инициализация клиента
val sdk = RenSDK.create("http://localhost:8001")

try {
    // Вход в систему
    sdk.login("user123", "password", rememberMe = false)
    Log.d("RenSDK", "Вход выполнен успешно")
    
    // Получение профиля
    val profile = sdk.getMe()
    Log.d("RenSDK", "Профиль: ${profile.username}")
    
    // Получение списка чатов
    val chats = sdk.getChats()
    Log.d("RenSDK", "Чатов: ${chats.size}")
    
    // Генерация ключей
    val keypair = RenSDK.generateKeypair()
    Log.d("RenSDK", "Public key: ${keypair.publicKey}")
    
    // Генерация соли
    val salt = RenSDK.generateSalt()
    Log.d("RenSDK", "Salt: $salt")
    
} catch (e: RenSDKError) {
    Log.e("RenSDK", "Ошибка: ${e.message}")
} finally {
    sdk.close()
}
```

## Структура проекта

```
app/
├── src/
│   └── main/
│       ├── java/
│       │   └── com/ren/sdk/
│       │       └── RenSDK.kt
│       └── jniLibs/
│           ├── arm64-v8a/
│           │   └── libren_sdk.so
│           ├── armeabi-v7a/
│           │   └── libren_sdk.so
│           ├── x86/
│           │   └── libren_sdk.so
│           └── x86_64/
│               └── libren_sdk.so
```

## JNI обёртка

Создайте файл `jni_wrapper.cpp` для JNI интерфейса между Kotlin и C FFI.

