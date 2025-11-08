use ren_sdk::crypto::*;
use std::env;

fn print_usage() {
    eprintln!("Usage:\n  ren-sdk gen-keypair\n  ren-sdk enc-msg <secret> <message>\n  ren-sdk dec-msg <secret> <cipher_b64> <nonce_b64>");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        return;
    }
    match args[1].as_str() {
        "gen-keypair" => {
            let kp = generate_key_pair(false);
            println!("public_key_b64: {}", kp.public_key);
            println!("private_key_b64: {}", kp.private_key);
        }
        "enc-msg" => {
            if args.len() < 4 { print_usage(); return; }
            let secret = &args[2];
            let msg = &args[3];
            let key = derive_key_from_string(secret).expect("key");
            let enc = encrypt_message(msg, &key).expect("enc");
            println!("ciphertext_b64: {}", enc.ciphertext);
            println!("nonce_b64: {}", enc.nonce);
        }
        "dec-msg" => {
            if args.len() < 5 { print_usage(); return; }
            let secret = &args[2];
            let ct = &args[3];
            let nonce = &args[4];
            let key = derive_key_from_string(secret).expect("key");
            let msg = decrypt_message(ct, nonce, &key).expect("dec");
            println!("{}", msg);
        }
        _ => print_usage(),
    }
}

