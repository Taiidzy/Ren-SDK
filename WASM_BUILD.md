# Сборка для WebAssembly

## Известные проблемы

### Проблема с getrandom

При сборке для WASM может возникнуть ошибка с `getrandom`, так как reqwest 0.12.24 тянет getrandom 0.3.4, который требует другую конфигурацию для WASM.

### Решение

**Вариант 1: Использовать правильные флаги (рекомендуется)**

```bash
cd Ren-SDK
wasm-pack build --target web --out-dir examples/web/pkg --features wasm --no-default-features
```

**Вариант 2: Обновить reqwest**

Обновите reqwest до версии, которая лучше поддерживает WASM. В будущих версиях это может быть исправлено.

**Вариант 3: Использовать только криптографию**

Для WASM можно использовать только криптографические функции (без сетевых):

```bash
wasm-pack build --target web --out-dir examples/web/pkg --features wasm,crypto_x25519 --no-default-features
```

Это соберёт только криптографические функции без сетевого API.

## Проверка

```bash
# Проверка компиляции
cargo check --target wasm32-unknown-unknown --features wasm --no-default-features

# Если проверка проходит, собирайте
wasm-pack build --target web --out-dir examples/web/pkg --features wasm --no-default-features
```

## Примечание

Если ошибка с getrandom сохраняется, это известная проблема совместимости между reqwest 0.12.24 и getrandom для WASM. В этом случае рекомендуется:

1. Использовать только криптографические функции (вариант 3)
2. Или обновить reqwest до версии с лучшей поддержкой WASM
3. Или использовать альтернативный HTTP клиент для WASM (например, через web-sys)
