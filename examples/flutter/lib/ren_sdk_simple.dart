/// Упрощённая версия Ren SDK для Flutter
/// Использует упрощённый подход без Struct для RenResult

import 'dart:ffi';
import 'dart:io';
import 'dart:convert';
import 'package:ffi/ffi.dart';

/// Ошибка SDK
class RenSDKError implements Exception {
  final String message;
  final int code;

  RenSDKError(this.message, [this.code = 0]);

  @override
  String toString() => 'RenSDKError: $message (code: $code)';
}

/// Клиент Ren SDK для Flutter (упрощённая версия)
class RenSDK {
  final DynamicLibrary _dylib;
  final Pointer<Void> _handle;

  RenSDK._(this._dylib, this._handle);

  /// Создаёт новый клиент SDK
  factory RenSDK.create(String baseUrl) {
    final dylib = _loadLibrary();
    final createFunc = dylib
        .lookupFunction<
          Pointer<Void> Function(Pointer<Utf8>),
          Pointer<Void> Function(Pointer<Utf8>)
        >('ren_sdk_client_new');

    final baseUrlPtr = baseUrl.toNativeUtf8();
    final handle = createFunc(baseUrlPtr);
    malloc.free(baseUrlPtr);

    if (handle == nullptr) {
      throw RenSDKError('Failed to create SDK client');
    }

    return RenSDK._(dylib, handle);
  }

  /// Загружает нативную библиотеку
  static DynamicLibrary _loadLibrary() {
    if (Platform.isWindows) {
      return DynamicLibrary.open('ren_sdk.dll');
    } else if (Platform.isLinux) {
      return DynamicLibrary.open('libren_sdk.so');
    } else if (Platform.isMacOS) {
      return DynamicLibrary.open('libren_sdk.dylib');
    } else if (Platform.isAndroid) {
      return DynamicLibrary.open('libren_sdk.so');
    } else if (Platform.isIOS) {
      return DynamicLibrary.open('libren_sdk.a');
    } else {
      throw UnsupportedError('Platform not supported');
    }
  }

  /// Устанавливает токен авторизации
  void setToken(String token) {
    final setTokenFunc = _dylib
        .lookupFunction<
          Int32 Function(Pointer<Void>, Pointer<Utf8>),
          int Function(Pointer<Void>, Pointer<Utf8>)
        >('ren_sdk_client_set_token_simple');

    final tokenPtr = token.toNativeUtf8();
    final code = setTokenFunc(_handle, tokenPtr);
    malloc.free(tokenPtr);

    if (code != 0) {
      throw RenSDKError('Failed to set token', code);
    }
  }

  /// Получает токен
  String? getToken() {
    final getTokenFunc = _dylib
        .lookupFunction<
          Pointer<Utf8> Function(Pointer<Void>),
          Pointer<Utf8> Function(Pointer<Void>)
        >('ren_sdk_client_get_token');

    final tokenPtr = getTokenFunc(_handle);
    if (tokenPtr == nullptr) {
      return null;
    }

    final token = tokenPtr.toDartString();
    final freeFunc = _dylib
        .lookupFunction<
          Void Function(Pointer<Utf8>),
          void Function(Pointer<Utf8>)
        >('ren_sdk_free_string');
    freeFunc(tokenPtr);

    return token;
  }

  /// Выполняет вход в систему
  void login(String login, String password, {bool rememberMe = false}) {
    final loginFunc = _dylib
        .lookupFunction<
          Int32 Function(Pointer<Void>, Pointer<Utf8>, Pointer<Utf8>, Int32),
          int Function(Pointer<Void>, Pointer<Utf8>, Pointer<Utf8>, int)
        >('ren_sdk_login_simple');

    final loginPtr = login.toNativeUtf8();
    final passwordPtr = password.toNativeUtf8();
    final code = loginFunc(_handle, loginPtr, passwordPtr, rememberMe ? 1 : 0);
    malloc.free(loginPtr);
    malloc.free(passwordPtr);

    if (code != 0) {
      throw RenSDKError('Login failed', code);
    }
  }

  /// Получает профиль текущего пользователя
  Map<String, dynamic> getMe() {
    final getMeFunc = _dylib
        .lookupFunction<
          Pointer<Utf8> Function(Pointer<Void>),
          Pointer<Utf8> Function(Pointer<Void>)
        >('ren_sdk_get_me');

    final jsonPtr = getMeFunc(_handle);
    if (jsonPtr == nullptr) {
      throw RenSDKError('Failed to get profile');
    }

    final jsonStr = jsonPtr.toDartString();
    final freeFunc = _dylib
        .lookupFunction<
          Void Function(Pointer<Utf8>),
          void Function(Pointer<Utf8>)
        >('ren_sdk_free_string');
    freeFunc(jsonPtr);

    return jsonDecode(jsonStr) as Map<String, dynamic>;
  }

  /// Получает список чатов
  List<Map<String, dynamic>> getChats() {
    final getChatsFunc = _dylib
        .lookupFunction<
          Pointer<Utf8> Function(Pointer<Void>),
          Pointer<Utf8> Function(Pointer<Void>)
        >('ren_sdk_get_chats');

    final jsonPtr = getChatsFunc(_handle);
    if (jsonPtr == nullptr) {
      throw RenSDKError('Failed to get chats');
    }

    final jsonStr = jsonPtr.toDartString();
    final freeFunc = _dylib
        .lookupFunction<
          Void Function(Pointer<Utf8>),
          void Function(Pointer<Utf8>)
        >('ren_sdk_free_string');
    freeFunc(jsonPtr);

    return (jsonDecode(jsonStr) as List).cast<Map<String, dynamic>>();
  }

  /// Освобождает ресурсы
  void dispose() {
    final freeFunc = _dylib
        .lookupFunction<
          Void Function(Pointer<Void>),
          void Function(Pointer<Void>)
        >('ren_sdk_client_free');
    freeFunc(_handle);
  }
}

/// Статические функции для криптографии
class RenSDKCrypto {
  static DynamicLibrary? _dylib;

  static DynamicLibrary get _library {
    _dylib ??= RenSDK._loadLibrary();
    return _dylib!;
  }

  /// Генерирует пару ключей
  static Map<String, String> generateKeypair() {
    final func = _library
        .lookupFunction<Pointer<Utf8> Function(), Pointer<Utf8> Function()>(
          'ren_sdk_generate_keypair',
        );

    final jsonPtr = func();
    if (jsonPtr == nullptr) {
      throw RenSDKError('Failed to generate keypair');
    }

    final jsonStr = jsonPtr.toDartString();
    final freeFunc = _library
        .lookupFunction<
          Void Function(Pointer<Utf8>),
          void Function(Pointer<Utf8>)
        >('ren_sdk_free_string');
    freeFunc(jsonPtr);

    return Map<String, String>.from(jsonDecode(jsonStr));
  }

  /// Генерирует соль
  static String generateSalt() {
    final func = _library
        .lookupFunction<Pointer<Utf8> Function(), Pointer<Utf8> Function()>(
          'ren_sdk_generate_salt',
        );

    final saltPtr = func();
    if (saltPtr == nullptr) {
      throw RenSDKError('Failed to generate salt');
    }

    final salt = saltPtr.toDartString();
    final freeFunc = _library
        .lookupFunction<
          Void Function(Pointer<Utf8>),
          void Function(Pointer<Utf8>)
        >('ren_sdk_free_string');
    freeFunc(saltPtr);

    return salt;
  }
}
