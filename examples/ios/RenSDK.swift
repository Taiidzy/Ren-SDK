//
//  RenSDK.swift
//  Ren Messenger iOS
//
//  Swift обёртка для Ren SDK (C FFI)
//

import Foundation

/// Ошибка SDK
public enum RenSDKError: Error {
    case invalidHandle
    case invalidParameters
    case apiError(String)
    case unknown
}

/// Клиент Ren SDK
public class RenSDK {
    private var handle: OpaquePointer?
    private let baseURL: String
    
    /// Инициализирует клиент SDK
    public init(baseURL: String) {
        self.baseURL = baseURL
        self.handle = ren_sdk_client_new(baseURL)
    }
    
    deinit {
        if let handle = handle {
            ren_sdk_client_free(handle)
        }
    }
    
    /// Устанавливает токен авторизации
    public func setToken(_ token: String) throws {
        guard let handle = handle else {
            throw RenSDKError.invalidHandle
        }
        
        let result = token.withCString { tokenPtr in
            ren_sdk_client_set_token(handle, tokenPtr)
        }
        
        if result.code != 0 {
            let message = result.message != nil ? String(cString: result.message!) : "Unknown error"
            ren_sdk_free_string(result.message)
            throw RenSDKError.apiError(message)
        }
    }
    
    /// Получает токен
    public func getToken() -> String? {
        guard let handle = handle else {
            return nil
        }
        
        guard let tokenPtr = ren_sdk_client_get_token(handle) else {
            return nil
        }
        
        let token = String(cString: tokenPtr)
        ren_sdk_free_string(tokenPtr)
        return token
    }
    
    /// Выполняет вход в систему
    public func login(login: String, password: String, rememberMe: Bool = false) throws {
        guard let handle = handle else {
            throw RenSDKError.invalidHandle
        }
        
        let result = login.withCString { loginPtr in
            password.withCString { passwordPtr in
                ren_sdk_login(handle, loginPtr, passwordPtr, rememberMe ? 1 : 0)
            }
        }
        
        if result.code != 0 {
            let message = result.message != nil ? String(cString: result.message!) : "Unknown error"
            ren_sdk_free_string(result.message)
            throw RenSDKError.apiError(message)
        }
    }
    
    /// Получает профиль текущего пользователя
    public func getMe() throws -> [String: Any] {
        guard let handle = handle else {
            throw RenSDKError.invalidHandle
        }
        
        guard let jsonPtr = ren_sdk_get_me(handle) else {
            throw RenSDKError.unknown
        }
        
        let jsonString = String(cString: jsonPtr)
        ren_sdk_free_string(jsonPtr)
        
        guard let data = jsonString.data(using: .utf8),
              let json = try? JSONSerialization.jsonObject(with: data) as? [String: Any] else {
            throw RenSDKError.unknown
        }
        
        return json
    }
    
    /// Получает список чатов
    public func getChats() throws -> [[String: Any]] {
        guard let handle = handle else {
            throw RenSDKError.invalidHandle
        }
        
        guard let jsonPtr = ren_sdk_get_chats(handle) else {
            throw RenSDKError.unknown
        }
        
        let jsonString = String(cString: jsonPtr)
        ren_sdk_free_string(jsonPtr)
        
        guard let data = jsonString.data(using: .utf8),
              let json = try? JSONSerialization.jsonObject(with: data) as? [[String: Any]] else {
            throw RenSDKError.unknown
        }
        
        return json
    }
    
    /// Генерирует пару ключей
    public static func generateKeypair() throws -> [String: String] {
        guard let jsonPtr = ren_sdk_generate_keypair() else {
            throw RenSDKError.unknown
        }
        
        let jsonString = String(cString: jsonPtr)
        ren_sdk_free_string(jsonPtr)
        
        guard let data = jsonString.data(using: .utf8),
              let json = try? JSONSerialization.jsonObject(with: data) as? [String: String] else {
            throw RenSDKError.unknown
        }
        
        return json
    }
    
    /// Генерирует соль
    public static func generateSalt() -> String {
        guard let saltPtr = ren_sdk_generate_salt() else {
            return ""
        }
        
        let salt = String(cString: saltPtr)
        ren_sdk_free_string(saltPtr)
        return salt
    }
}

// MARK: - C FFI функции

@_silgen_name("ren_sdk_client_new")
func ren_sdk_client_new(_ baseURL: UnsafePointer<CChar>) -> OpaquePointer?

@_silgen_name("ren_sdk_client_free")
func ren_sdk_client_free(_ handle: OpaquePointer)

@_silgen_name("ren_sdk_client_set_token")
func ren_sdk_client_set_token(_ handle: OpaquePointer, _ token: UnsafePointer<CChar>) -> RenResult

@_silgen_name("ren_sdk_client_get_token")
func ren_sdk_client_get_token(_ handle: OpaquePointer) -> UnsafeMutablePointer<CChar>?

@_silgen_name("ren_sdk_login")
func ren_sdk_login(_ handle: OpaquePointer, _ login: UnsafePointer<CChar>, _ password: UnsafePointer<CChar>, _ rememberMe: Int32) -> RenResult

@_silgen_name("ren_sdk_get_me")
func ren_sdk_get_me(_ handle: OpaquePointer) -> UnsafeMutablePointer<CChar>?

@_silgen_name("ren_sdk_get_chats")
func ren_sdk_get_chats(_ handle: OpaquePointer) -> UnsafeMutablePointer<CChar>?

@_silgen_name("ren_sdk_generate_keypair")
func ren_sdk_generate_keypair() -> UnsafeMutablePointer<CChar>?

@_silgen_name("ren_sdk_generate_salt")
func ren_sdk_generate_salt() -> UnsafeMutablePointer<CChar>?

@_silgen_name("ren_sdk_free_string")
func ren_sdk_free_string(_ str: UnsafeMutablePointer<CChar>)

struct RenResult {
    let code: Int32
    let message: UnsafeMutablePointer<CChar>?
}

