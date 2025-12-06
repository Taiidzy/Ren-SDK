//! C FFI интерфейс для нативных платформ (iOS/Android)

#![allow(unsafe_code)]

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::ptr;
use std::sync::Arc;

use crate::client::RenClient;
use crate::network::api::types::auth::LoginRequest;

/// Opaque handle для RenClient
#[repr(C)]
pub struct RenClientHandle {
    client: Arc<RenClient>,
}

/// Результат операции (0 = успех, иначе код ошибки)
#[repr(C)]
pub struct RenResult {
    pub code: c_int,
    pub message: *mut c_char, // освобождается вызывающей стороной через ren_sdk_free_string
}

impl RenResult {
    fn success() -> Self {
        Self {
            code: 0,
            message: ptr::null_mut(),
        }
    }

    fn error(code: c_int, msg: String) -> Self {
        let c_msg = CString::new(msg).unwrap_or_else(|_| CString::new("Unknown error").unwrap());
        Self {
            code,
            message: c_msg.into_raw(),
        }
    }
}

/// Создаёт новый клиент SDK
///
/// # Safety
/// base_url должен быть валидной C строкой
#[no_mangle]
pub unsafe extern "C" fn ren_sdk_client_new(base_url: *const c_char) -> *mut RenClientHandle {
    if base_url.is_null() {
        return ptr::null_mut();
    }

    let url = match CStr::from_ptr(base_url).to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let client = RenClient::new(url);
    Box::into_raw(Box::new(RenClientHandle {
        client: Arc::new(client),
    }))
}

/// Освобождает клиент SDK
///
/// # Safety
/// handle должен быть валидным указателем, полученным из ren_sdk_client_new
#[no_mangle]
pub unsafe extern "C" fn ren_sdk_client_free(handle: *mut RenClientHandle) {
    if !handle.is_null() {
        drop(Box::from_raw(handle));
    }
}

/// Освобождает строку, выделенную SDK
///
/// # Safety
/// str_ptr должен быть валидным указателем, полученным из SDK
#[no_mangle]
pub unsafe extern "C" fn ren_sdk_free_string(str_ptr: *mut c_char) {
    if !str_ptr.is_null() {
        drop(CString::from_raw(str_ptr));
    }
}

/// Устанавливает токен авторизации
///
/// # Safety
/// handle должен быть валидным указателем
/// token должен быть валидной C строкой
#[no_mangle]
pub unsafe extern "C" fn ren_sdk_client_set_token(
    handle: *mut RenClientHandle,
    token: *const c_char,
) -> RenResult {
    if handle.is_null() || token.is_null() {
        return RenResult::error(1, "Invalid parameters".to_string());
    }

    let token_str = match CStr::from_ptr(token).to_str() {
        Ok(s) => s,
        Err(_) => return RenResult::error(2, "Invalid token string".to_string()),
    };

    // Для синхронного FFI нужно использовать блокирующий вызов
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        (*handle).client.set_token(token_str.to_string()).await;
    });

    RenResult::success()
}

/// Получает токен (возвращает строку, которую нужно освободить через ren_sdk_free_string)
///
/// # Safety
/// handle должен быть валидным указателем
#[no_mangle]
pub unsafe extern "C" fn ren_sdk_client_get_token(
    handle: *mut RenClientHandle,
) -> *mut c_char {
    if handle.is_null() {
        return ptr::null_mut();
    }

    let rt = tokio::runtime::Runtime::new().unwrap();
    let token = rt.block_on(async {
        (*handle).client.get_token().await
    });

    match token {
        Some(t) => {
            match CString::new(t) {
                Ok(c_str) => c_str.into_raw(),
                Err(_) => ptr::null_mut(),
            }
        }
        None => ptr::null_mut(),
    }
}

/// Выполняет вход в систему
///
/// # Safety
/// handle должен быть валидным указателем
/// login и password должны быть валидными C строками
#[no_mangle]
pub unsafe extern "C" fn ren_sdk_login(
    handle: *mut RenClientHandle,
    login: *const c_char,
    password: *const c_char,
    remember_me: c_int,
) -> RenResult {
    if handle.is_null() || login.is_null() || password.is_null() {
        return RenResult::error(1, "Invalid parameters".to_string());
    }

    let login_str = match CStr::from_ptr(login).to_str() {
        Ok(s) => s,
        Err(_) => return RenResult::error(2, "Invalid login string".to_string()),
    };

    let password_str = match CStr::from_ptr(password).to_str() {
        Ok(s) => s,
        Err(_) => return RenResult::error(3, "Invalid password string".to_string()),
    };

    let req = LoginRequest {
        login: login_str.to_string(),
        password: password_str.to_string(),
        remember_me: if remember_me != 0 { Some(true) } else { Some(false) },
    };

    let rt = tokio::runtime::Runtime::new().unwrap();
    match rt.block_on(async {
        crate::network::api::auth::login(&(*handle).client, req).await
    }) {
        Ok(_) => RenResult::success(),
        Err(e) => RenResult::error(100, format!("Login failed: {}", e)),
    }
}

/// Получает профиль текущего пользователя (JSON строка)
///
/// # Safety
/// handle должен быть валидным указателем
#[no_mangle]
pub unsafe extern "C" fn ren_sdk_get_me(
    handle: *mut RenClientHandle,
) -> *mut c_char {
    if handle.is_null() {
        return ptr::null_mut();
    }

    let rt = tokio::runtime::Runtime::new().unwrap();
    match rt.block_on(async {
        crate::network::api::users::get_me(&(*handle).client).await
    }) {
        Ok(user) => {
            match serde_json::to_string(&user) {
                Ok(json) => {
                    match CString::new(json) {
                        Ok(c_str) => c_str.into_raw(),
                        Err(_) => ptr::null_mut(),
                    }
                }
                Err(_) => ptr::null_mut(),
            }
        }
        Err(_) => ptr::null_mut(),
    }
}

/// Получает список чатов (JSON строка)
///
/// # Safety
/// handle должен быть валидным указателем
#[no_mangle]
pub unsafe extern "C" fn ren_sdk_get_chats(
    handle: *mut RenClientHandle,
) -> *mut c_char {
    if handle.is_null() {
        return ptr::null_mut();
    }

    let rt = tokio::runtime::Runtime::new().unwrap();
    match rt.block_on(async {
        crate::network::api::chats::get_chats(&(*handle).client).await
    }) {
        Ok(chats) => {
            match serde_json::to_string(&chats) {
                Ok(json) => {
                    match CString::new(json) {
                        Ok(c_str) => c_str.into_raw(),
                        Err(_) => ptr::null_mut(),
                    }
                }
                Err(_) => ptr::null_mut(),
            }
        }
        Err(_) => ptr::null_mut(),
    }
}

/// Генерирует пару ключей (JSON строка с public_key и private_key)
///
/// # Safety
/// Функция безопасна, не требует параметров
#[no_mangle]
pub unsafe extern "C" fn ren_sdk_generate_keypair() -> *mut c_char {
    let keypair = crate::generate_key_pair(false);
    match serde_json::to_string(&keypair) {
        Ok(json) => {
            match CString::new(json) {
                Ok(c_str) => c_str.into_raw(),
                Err(_) => ptr::null_mut(),
            }
        }
        Err(_) => ptr::null_mut(),
    }
}

/// Генерирует соль для криптографии
///
/// # Safety
/// Функция безопасна
#[no_mangle]
pub unsafe extern "C" fn ren_sdk_generate_salt() -> *mut c_char {
    let salt = crate::generate_salt();
    match CString::new(salt) {
        Ok(c_str) => c_str.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

