
extern crate crypto;
use crypto::aes::{self, KeySize};
use crypto::blockmodes::PkcsPadding;
use crypto::buffer::{ReadBuffer, WriteBuffer, BufferResult};
use std::env;
use std::time::Instant;






fn aes_encrypt(key: &[u8], data: &[u8]) -> Result<Vec<u8>, &'static str> {
    if key.len() != 16 {
        return Err("Invalid key size, AES-128 requires a 16-byte key.");
    }

    let mut encryptor = aes::ecb_encryptor(KeySize::KeySize128, key, PkcsPadding);
    let mut final_result = Vec::new();

    let mut read_buffer = crypto::buffer::RefReadBuffer::new(data);
    let mut buffer = [0; 16];
    let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true).unwrap();

        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(final_result)
}

fn aes_decrypt(key: &[u8], data: &[u8]) -> Result<Vec<u8>, &'static str> {
    if key.len() != 16 {
        return Err("Invalid key size, AES-128 requires a 16-byte key.");
    }

    let mut decryptor = aes::ecb_decryptor(KeySize::KeySize128, key, PkcsPadding);
    let mut final_result = Vec::new();

    let mut read_buffer = crypto::buffer::RefReadBuffer::new(data);
    let mut buffer = [0; 16];
    let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true).unwrap();

        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(final_result)
}

fn main() {



    let start = Instant::now();
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <key>", args[0]);
        std::process::exit(1);
    }

    let key = args[1].as_bytes();
    let plaintext = b"Hello, AES!12345"; // Replace with your data

    let encrypted_data = aes_encrypt(key, plaintext).unwrap();
    let decrypted_data = aes_decrypt(key, &encrypted_data).unwrap();

    let original_str = String::from_utf8_lossy(plaintext).to_string();
    let encrypted_str = String::from_utf8_lossy(&encrypted_data).to_string();
    let decrypted_str = String::from_utf8_lossy(&decrypted_data).to_string();

    println!("Original: {}", original_str);
    println!("Encrypted: {}", encrypted_str);
    println!("Decrypted: {}", decrypted_str);

    let duration = start.elapsed();
    println!("Time taken: {:.6} seconds", duration.as_secs_f64());
}
