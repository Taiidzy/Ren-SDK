use ren_sdk::crypto::*;
use ren_sdk::{
    decrypt_message_with_secret, derive_master_key_b64_from_password,
    derive_master_key_from_password, encrypt_message_with_secret,
};
use base64::{engine::general_purpose, Engine as _};

#[test]
fn test_salt_and_nonce_generation() {
    let salt_b64 = generate_salt();
    let salt = general_purpose::STANDARD.decode(salt_b64).unwrap();
    assert_eq!(salt.len(), 16);

    let nonce_b64 = generate_nonce();
    let nonce = general_purpose::STANDARD.decode(nonce_b64).unwrap();
    assert_eq!(nonce.len(), 12);
}

#[test]
fn test_decrypt_message_with_wrong_key_fails() {
    let key1 = generate_message_encryption_key();
    let key2 = generate_message_encryption_key();
    let enc = encrypt_message("oops", &key1).unwrap();
    let bad = decrypt_message(&enc.ciphertext, &enc.nonce, &key2);
    assert!(bad.is_err());
}

#[test]
fn test_decrypt_message_with_tampered_ciphertext_fails() {
    let key = generate_message_encryption_key();
    let mut enc = encrypt_message("auth", &key).unwrap();
    // flip a byte in ciphertext
    let mut ct = general_purpose::STANDARD.decode(&enc.ciphertext).unwrap();
    ct[0] ^= 0xFF;
    enc.ciphertext = general_purpose::STANDARD.encode(ct);
    let res = decrypt_message(&enc.ciphertext, &enc.nonce, &key);
    assert!(res.is_err());
}

#[test]
fn test_decrypt_data_with_invalid_nonce_length_fails() {
    let key = derive_key_from_string("secret").unwrap();
    let ct = encrypt_data("hello", &key).unwrap();
    let mut raw = general_purpose::STANDARD.decode(&ct).unwrap();
    // remove a nonce byte to create invalid 11-byte nonce
    raw.remove(0);
    let bad_b64 = general_purpose::STANDARD.encode(raw);
    let res = decrypt_data(&bad_b64, &key);
    assert!(res.is_err());
}

#[test]
fn test_decrypt_file_with_invalid_base64_fails() {
    let key = generate_message_encryption_key();
    let res = decrypt_file("not-base64!!!", "also-not-base64", &key);
    assert!(res.is_err());
}

#[test]
fn test_derive_key_from_string_and_encrypt_decrypt_data() {
    let key = derive_key_from_string("secret").unwrap();
    let ct = encrypt_data("hello", &key).unwrap();
    let pt = decrypt_data(&ct, &key).unwrap();
    assert_eq!(pt, "hello");
}

#[test]
fn test_encrypt_decrypt_message() {
    let key = generate_message_encryption_key();
    let enc = encrypt_message("ping", &key).unwrap();
    let dec = decrypt_message(&enc.ciphertext, &enc.nonce, &key).unwrap();
    assert_eq!(dec, "ping");
}

#[test]
fn test_encrypt_decrypt_file() {
    let key = generate_message_encryption_key();
    let data = b"file-bytes".to_vec();
    let ef = encrypt_file(&data, "f.txt", "text/plain", &key).unwrap();
    let dec = decrypt_file(&ef.ciphertext, &ef.nonce, &key).unwrap();
    assert_eq!(dec, data);
}

#[test]
fn test_encrypt_decrypt_file_with_message() {
    let key = generate_message_encryption_key();
    let data = b"file-data".to_vec();
    let efm = encrypt_file_with_message(&data, "hi", &key, "f.bin", "application/octet-stream").unwrap();
    let out = decrypt_file_with_message(&efm.enc_file, &efm.ciphertext, &efm.nonce, &key, &efm.filename, &efm.mimetype).unwrap();
    assert_eq!(out.file, data);
    assert_eq!(out.message, "hi");
}

#[test]
fn test_wrap_unwrap_symmetric_key() {
    let receiver = generate_key_pair(false);
    let receiver_pk = &receiver.public_key;
    let receiver_sk = &receiver.private_key;

    let msg_key = generate_message_encryption_key();

    let (wrapped, eph_pub, nonce) = wrap_symmetric_key(&msg_key, receiver_pk).unwrap();
    let unwrapped = unwrap_symmetric_key(&wrapped, &eph_pub, &nonce, receiver_sk).unwrap();

    let enc = encrypt_message("secret-msg", &msg_key).unwrap();
    let dec = decrypt_message(&enc.ciphertext, &enc.nonce, &unwrapped).unwrap();
    assert_eq!(dec, "secret-msg");
}

#[test]
fn test_wrapper_encrypt_decrypt_with_secret() {
    let enc = encrypt_message_with_secret("my-secret", "hello").unwrap();
    let dec = decrypt_message_with_secret("my-secret", &enc.ciphertext, &enc.nonce).unwrap();
    assert_eq!(dec, "hello");
}

#[test]
fn test_wrapper_derive_master_key_from_password() {
    let salt_b64 = generate_salt();
    let aead = derive_master_key_from_password("P@ssw0rd", &salt_b64).unwrap();
    let b64 = derive_master_key_b64_from_password("P@ssw0rd", &salt_b64).unwrap();
    let raw = general_purpose::STANDARD.decode(b64).unwrap();
    assert_eq!(aead.to_bytes().len(), 32);
    assert_eq!(raw.len(), 32);
}
