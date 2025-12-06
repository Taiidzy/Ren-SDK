//! FFI модуль для Ren SDK
//!
//! Предоставляет C-совместимый интерфейс для использования SDK из других языков:
//! - C/C++ (для iOS/Android через JNI)
//! - Swift (через C interop)
//! - Kotlin (через JNI)
//!
//! Для Web используйте модуль wasm вместо этого.

#![allow(unsafe_code)] // FFI требует unsafe

#[cfg(feature = "wasm")]
pub mod wasm;

#[cfg(feature = "ffi")]
mod c_ffi;

#[cfg(feature = "ffi")]
pub use c_ffi::*;
