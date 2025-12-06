# Ren SDK - Native Examples (Linux/Windows)

Примеры использования Ren SDK на нативных платформах через C FFI.

## Linux

### Сборка

```bash
cd Ren-SDK

# Соберите библиотеку
cargo build --release --target x86_64-unknown-linux-gnu --features ffi

# Сгенерируйте C заголовки
cbindgen --config cbindgen.toml --crate ren-sdk --output examples/native/linux/ren_sdk.h

# Скомпилируйте пример
cd examples/native/linux
gcc -o example main.c -L../../../../target/x86_64-unknown-linux-gnu/release -lren_sdk -lpthread -ldl -lm

# Запустите (убедитесь, что библиотека в LD_LIBRARY_PATH)
export LD_LIBRARY_PATH=../../../../target/x86_64-unknown-linux-gnu/release:$LD_LIBRARY_PATH
./example
```

### Использование

```c
#include "ren_sdk.h"

// Создание клиента
RenClientHandle* client = ren_sdk_client_new("http://localhost:8001");

// Вход
RenResult result = ren_sdk_login(client, "user123", "password", 0);
if (result.code == 0) {
    printf("Вход выполнен\n");
}

// Получение профиля
char* profile = ren_sdk_get_me(client);
printf("Профиль: %s\n", profile);
ren_sdk_free_string(profile);

// Освобождение
ren_sdk_client_free(client);
```

## Windows

### Сборка

```bash
cd Ren-SDK

# Соберите библиотеку
cargo build --release --target x86_64-pc-windows-msvc --features ffi

# Сгенерируйте C заголовки
cbindgen --config cbindgen.toml --crate ren-sdk --output examples/native/windows/ren_sdk.h

# Скомпилируйте пример (используя MSVC или MinGW)
cd examples/native/windows
# MSVC:
cl main.c /link ren_sdk.lib /LIBPATH:..\..\..\..\target\x86_64-pc-windows-msvc\release
# MinGW:
gcc -o example.exe main.c -L../../../../target/x86_64-pc-windows-msvc/release -lren_sdk
```

### Использование

Аналогично Linux примеру.

## Структура

```
examples/native/
├── linux/
│   ├── main.c          # Пример для Linux
│   └── ren_sdk.h       # C заголовки (генерируются)
└── windows/
    ├── main.c          # Пример для Windows
    └── ren_sdk.h       # C заголовки (генерируются)
```

## Примечания

- Убедитесь, что нативная библиотека доступна в PATH (Windows) или LD_LIBRARY_PATH (Linux)
- Для статической линковки используйте `staticlib` вместо `cdylib` в Cargo.toml
- На Windows может потребоваться установка Visual C++ Redistributable

