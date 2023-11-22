use des_cipher::{des_encrypt, des_decrypt};
use std::io;
use std::time::Instant;

fn main() {
    println!("Do you want to (1) encrypt or (2) decrypt?");
    let mut choice: String = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice: u32 = choice.trim().parse::<u32>().expect("Invalid choice");

    println!("Please enter plaintext or ciphertext (as a number):");
    let mut text: String = String::new();
    io::stdin().read_line(&mut text).unwrap();
    let text: u64 = u64::from_str_radix(&text.trim(), 16).expect("Invalid input");

    println!("Please enter key (as a number):");
    let mut key: String = String::new();
    io::stdin().read_line(&mut key).unwrap();
    let key: u64 = u64::from_str_radix(&key.trim(), 16).expect("Invalid key");

    let start = Instant::now();

    match choice {
        1 => println!("Encrypted: {:016x}", des_encrypt(text, key)),
        2 => println!("Decrypted: {:016x}", des_decrypt(text, key)),
        _ => println!("Invalid choice"),
    }

    let duration = start.elapsed();
    println!("Time taken: {:.6} seconds", duration.as_secs_f64());
}
