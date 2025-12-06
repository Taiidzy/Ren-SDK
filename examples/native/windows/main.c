// Пример использования Ren SDK на Windows через C FFI

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "ren_sdk.h"

int main() {
    printf("Ren SDK - Windows Example\n");
    printf("=========================\n\n");

    // Создание клиента
    RenClientHandle* client = ren_sdk_client_new("http://localhost:8001");
    if (!client) {
        fprintf(stderr, "Ошибка создания клиента\n");
        return 1;
    }
    printf("✓ Клиент создан\n");

    // Вход в систему
    RenResult result = ren_sdk_login(client, "user123", "password", 0);
    if (result.code != 0) {
        fprintf(stderr, "Ошибка входа: %s\n", result.message ? result.message : "Unknown");
        if (result.message) {
            ren_sdk_free_string(result.message);
        }
        ren_sdk_client_free(client);
        return 1;
    }
    printf("✓ Вход выполнен\n");
    if (result.message) {
        ren_sdk_free_string(result.message);
    }

    // Получение профиля
    char* profile_json = ren_sdk_get_me(client);
    if (profile_json) {
        printf("✓ Профиль получен:\n%s\n\n", profile_json);
        ren_sdk_free_string(profile_json);
    } else {
        fprintf(stderr, "Ошибка получения профиля\n");
    }

    // Получение списка чатов
    char* chats_json = ren_sdk_get_chats(client);
    if (chats_json) {
        printf("✓ Чаты получены:\n%s\n\n", chats_json);
        ren_sdk_free_string(chats_json);
    } else {
        fprintf(stderr, "Ошибка получения чатов\n");
    }

    // Генерация ключей
    char* keypair_json = ren_sdk_generate_keypair();
    if (keypair_json) {
        printf("✓ Пара ключей сгенерирована:\n%s\n\n", keypair_json);
        ren_sdk_free_string(keypair_json);
    }

    // Генерация соли
    char* salt = ren_sdk_generate_salt();
    if (salt) {
        printf("✓ Соль сгенерирована: %s\n", salt);
        ren_sdk_free_string(salt);
    }

    // Освобождение ресурсов
    ren_sdk_client_free(client);
    printf("\n✓ Клиент освобождён\n");

    return 0;
}

