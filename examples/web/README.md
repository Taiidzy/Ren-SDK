# Ren SDK - Web Example

Пример использования Ren SDK в веб-приложении через WebAssembly.

## Сборка

1. Установите необходимые инструменты:
```bash
# Установите wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Установите Rust toolchain для wasm
rustup target add wasm32-unknown-unknown
```

2. Соберите WASM модуль:
```bash
cd Ren-SDK

# Важно: используйте --no-default-features чтобы исключить native зависимости
wasm-pack build --target web --out-dir examples/web/pkg --features wasm --no-default-features
```

**Примечание:** Если возникает ошибка с `getrandom`, это известная проблема совместимости между reqwest 0.12.24 и getrandom для WASM. 

**Временное решение:** Используйте только криптографические функции:
```bash
wasm-pack build --target web --out-dir examples/web/pkg --features wasm,crypto_x25519 --no-default-features
```

См. [WASM_BUILD.md](../../WASM_BUILD.md) для подробностей.

3. Запустите локальный сервер:
```bash
cd examples/web
python -m http.server 8080
# или
npx serve .
```

4. Откройте в браузере: http://localhost:8080

## Использование

1. Инициализируйте клиент с базовым URL вашего API
2. Войдите в систему с логином и паролем
3. Используйте методы клиента для работы с API

## API

### WasmClient

```javascript
import { WasmClient } from './pkg/ren_sdk.js';

const client = new WasmClient('http://localhost:8001');

// Вход
await client.login('login', 'password', false);

// Получить профиль
const profile = await client.get_me();

// Получить чаты
const chats = await client.get_chats();

// Создать чат
const chat = await client.create_chat('private', null, [1, 2]);

// Получить сообщения
const messages = await client.get_messages(chatId);
```

### Криптографические функции

```javascript
import { wasm_generate_keypair, wasm_generate_salt, wasm_encrypt_message_for_recipients } from './pkg/ren_sdk.js';

// Генерация ключей
const keypair = wasm_generate_keypair();

// Генерация соли
const salt = wasm_generate_salt();

// Шифрование сообщения
const recipientKeys = { 2: "public_key_user_2", 3: "public_key_user_3" };
const result = wasm_encrypt_message_for_recipients("Hello", JSON.stringify(recipientKeys));
```

