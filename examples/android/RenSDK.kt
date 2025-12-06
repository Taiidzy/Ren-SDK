package com.ren.sdk

import android.util.Log
import org.json.JSONArray
import org.json.JSONObject

/**
 * Ошибка SDK
 */
sealed class RenSDKError(message: String) : Exception(message) {
    object InvalidHandle : RenSDKError("Invalid handle")
    object InvalidParameters : RenSDKError("Invalid parameters")
    class ApiError(msg: String) : RenSDKError("API error: $msg")
    object Unknown : RenSDKError("Unknown error")
}

/**
 * Клиент Ren SDK для Android
 */
class RenSDK private constructor(private val handle: Long) {
    
    companion object {
        private const val TAG = "RenSDK"
        
        // Загружаем нативную библиотеку
        init {
            System.loadLibrary("ren_sdk")
        }
        
        /**
         * Создаёт новый клиент SDK
         */
        fun create(baseURL: String): RenSDK {
            val handle = nativeClientNew(baseURL)
            if (handle == 0L) {
                throw RenSDKError.InvalidHandle
            }
            return RenSDK(handle)
        }
        
        /**
         * Генерирует пару ключей
         */
        fun generateKeypair(): KeyPair {
            val jsonStr = nativeGenerateKeypair()
            val json = JSONObject(jsonStr)
            return KeyPair(
                publicKey = json.getString("public_key"),
                privateKey = json.getString("private_key")
            )
        }
        
        /**
         * Генерирует соль
         */
        fun generateSalt(): String {
            return nativeGenerateSalt()
        }
    }
    
    /**
     * Устанавливает токен авторизации
     */
    fun setToken(token: String) {
        val result = nativeSetToken(handle, token)
        if (result.code != 0) {
            throw RenSDKError.ApiError(result.message ?: "Unknown error")
        }
    }
    
    /**
     * Получает токен
     */
    fun getToken(): String? {
        val token = nativeGetToken(handle)
        return token
    }
    
    /**
     * Выполняет вход в систему
     */
    fun login(login: String, password: String, rememberMe: Boolean = false) {
        val result = nativeLogin(handle, login, password, rememberMe)
        if (result.code != 0) {
            throw RenSDKError.ApiError(result.message ?: "Unknown error")
        }
    }
    
    /**
     * Получает профиль текущего пользователя
     */
    fun getMe(): User {
        val jsonStr = nativeGetMe(handle) ?: throw RenSDKError.Unknown
        val json = JSONObject(jsonStr)
        return User(
            id = json.getLong("id"),
            login = json.getString("login"),
            username = json.getString("username"),
            avatar = json.optString("avatar", null)
        )
    }
    
    /**
     * Получает список чатов
     */
    fun getChats(): List<Chat> {
        val jsonStr = nativeGetChats(handle) ?: throw RenSDKError.Unknown
        val jsonArray = JSONArray(jsonStr)
        val chats = mutableListOf<Chat>()
        for (i in 0 until jsonArray.length()) {
            val chatJson = jsonArray.getJSONObject(i)
            chats.add(Chat(
                id = chatJson.getLong("id"),
                kind = chatJson.getString("kind"),
                title = chatJson.optString("title", null),
                peerUsername = chatJson.getString("peer_username"),
                peerAvatar = chatJson.optString("peer_avatar", null)
            ))
        }
        return chats
    }
    
    /**
     * Освобождает ресурсы
     */
    fun close() {
        nativeClientFree(handle)
    }
    
    // Нативные методы
    private external fun nativeClientNew(baseURL: String): Long
    private external fun nativeClientFree(handle: Long)
    private external fun nativeSetToken(handle: Long, token: String): NativeResult
    private external fun nativeGetToken(handle: Long): String?
    private external fun nativeLogin(handle: Long, login: String, password: String, rememberMe: Boolean): NativeResult
    private external fun nativeGetMe(handle: Long): String?
    private external fun nativeGetChats(handle: Long): String?
    
    private external fun nativeGenerateKeypair(): String
    private external fun nativeGenerateSalt(): String
    
    /**
     * Результат нативной операции
     */
    data class NativeResult(
        val code: Int,
        val message: String?
    )
    
    /**
     * Пара ключей
     */
    data class KeyPair(
        val publicKey: String,
        val privateKey: String
    )
    
    /**
     * Пользователь
     */
    data class User(
        val id: Long,
        val login: String,
        val username: String,
        val avatar: String?
    )
    
    /**
     * Чат
     */
    data class Chat(
        val id: Long,
        val kind: String,
        val title: String?,
        val peerUsername: String,
        val peerAvatar: String?
    )
}

