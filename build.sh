#!/bin/bash
# Скрипт для сборки SDK для разных платформ

set -e

echo "Building Ren SDK for multiple platforms..."

# WebAssembly
echo "Building for WebAssembly..."
wasm-pack build --target web --out-dir examples/web/pkg --features wasm

# iOS
echo "Building for iOS..."
cargo build --release --target aarch64-apple-ios --features ffi
cargo build --release --target x86_64-apple-ios --features ffi

# Создаём универсальную библиотеку для iOS
lipo -create \
  target/aarch64-apple-ios/release/libren_sdk.a \
  target/x86_64-apple-ios/release/libren_sdk.a \
  -output examples/ios/libren_sdk_universal.a

# Linux
echo "Building for Linux..."
cargo build --release --target x86_64-unknown-linux-gnu --features ffi

# Windows
echo "Building for Windows..."
cargo build --release --target x86_64-pc-windows-msvc --features ffi

# Android
echo "Building for Android..."
# Установите Android NDK и настройте cargo-ndk
# cargo build --release --target aarch64-linux-android --features ffi
# cargo build --release --target armv7-linux-androideabi --features ffi
# cargo build --release --target x86_64-linux-android --features ffi
# cargo build --release --target i686-linux-android --features ffi

echo "Build complete!"

