extern crate openssl;

use openssl::symm::{Cipher, Crypter, Mode};
use std::io;

fn des_encrypt(message: &str, key: &str) -> Result<Vec<u8>, openssl::error::Error> {
    if key.len() != 8 {
        return Err(openssl::error::Error::from("Key must be 8 bytes long."));
    }

    let cipher = Cipher::des_ecb();
    let mut encrypter = Crypter::new(cipher, Mode::Encrypt, key.as_bytes(), None)?;
    let mut encrypted_message = vec![0; message.len() + 8];

    let encrypted_len = encrypter.update(message.as_bytes(), &mut encrypted_message)?;

    encrypted_message.truncate(encrypted_len);
    
    Ok(encrypted_message)
}

fn des_decrypt(encrypted_message: &[u8], key: &str) -> Result<Vec<u8>, openssl::error::Error> {
    if key.len() != 8 {
        return Err(openssl::error::Error::from("Key must be 8 bytes long."));
    }

    let cipher = Cipher::des_ecb();
    let mut decrypter = Crypter::new(cipher, Mode::Decrypt, key.as_bytes(), None)?;
    let mut decrypted_message = vec![0; encrypted_message.len() + 8];

    let decrypted_len = decrypter.update(encrypted_message, &mut decrypted_message)?;

    decrypted_message.truncate(decrypted_len);
    
    Ok(decrypted_message)
}

fn main() {
    let mut key = String::new();
    let mut message = String::new();

    // Get the key from the user
    println!("Enter the 8-byte DES key: ");
    io::stdin().read_line(&mut key).expect("Failed to read line");

    // Remove leading and trailing whitespaces
    key = key.trim().to_string();
    if key.len() != 8 {
        println!("Key must be 8 bytes long.");
        return;
    }

    // Get the message from the user
    println!("Enter the message to encrypt: ");
    io::stdin().read_line(&mut message).expect("Failed to read line");

    // Remove leading and trailing whitespaces
    message = message.trim().to_string();

    // Perform encryption
    let encrypted = des_encrypt(&message, &key).expect("Encryption failed");
    println!("Encrypted message: {:?}", encrypted);

    // Perform decryption
    let decrypted = des_decrypt(&encrypted, &key).expect("Decryption failed");
    println!("Decrypted message: {:?}", String::from_utf8(decrypted).expect("Invalid UTF-8"));
}
