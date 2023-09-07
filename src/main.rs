use anyhow::Result;
use secp256k1::{PublicKey, SecretKey, rand};
use web3::signing::keccak256;
use web3::types::Address;
use std::io;
use std::fs;
use std::sync::{mpsc, Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use hex;
use std::io::Write;
use tokio::time::Instant;

const N_THREADS: usize = 8; // Number of threads
const INITIAL_CHARACTERS: usize = 8; // Number of repeated characters at the beginning of the address after 0x...
const NUMBER_OF_REPEATED_CHARACTERS: usize = 8; // Number of repeating characters in the address anywhere e.g. 0x...0000...

fn generate_keypair() -> (SecretKey, PublicKey) {
    let secp = secp256k1::Secp256k1::new();
    let mut rng = rand::thread_rng();
    secp.generate_keypair(&mut rng)
}

fn public_key_address(public_key: &PublicKey) -> Address {
    let public_key = public_key.serialize_uncompressed();
    debug_assert_eq!(public_key[0], 0x04);
    let hash = keccak256(&public_key[1..]);
    Address::from_slice(&hash[12..])
}

fn append_to_file(data: &String, name_file: String) -> Result<(), std::io::Error> {
    if !fs::metadata("Logs").is_ok() {
        fs::create_dir("Logs")?;
    }
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(format!("Logs/{}.txt", name_file))?;
    writeln!(file, "{}", data)?;
    Ok(())
}

fn has_repeated_chars(s: String) -> bool {
    if s.chars().take(INITIAL_CHARACTERS).all(|c| c == s.chars().nth(2).unwrap()) {
        return true;
    }

    let nm = NUMBER_OF_REPEATED_CHARACTERS - 1;
    let chars: Vec<char> = s.chars().collect();
    for i in 0..chars.len() - NUMBER_OF_REPEATED_CHARACTERS {
        if chars[i..i + nm].iter().all(|&c| c == chars[i]) {
            return true;
        }
    }
    false
}

fn main() -> Result<()> {
    let desired_prefix = input("Enter the desired sequence after '0x': ");
    let num_wallets: usize = input("Enter the number of wallets to generate: ")
        .parse()
        .expect("Failed to parse number");

    let start = Instant::now();
    println!("Start...");

    let (tx, rx) = mpsc::channel();
    let running = Arc::new(AtomicBool::new(true));

    for _ in 0..N_THREADS {
        let tx = tx.clone();
        let running_clone = running.clone();
        thread::spawn(move || {
            while running_clone.load(Ordering::Relaxed) {
                let keypair = generate_keypair();
                if tx.send(keypair).is_err() {
                    break;
                }
            }
        });
    }

    let mut generated_wallets = 0;

    while generated_wallets < num_wallets {
        let (secret_key, pub_key) = rx.recv().unwrap();
        let address = public_key_address(&pub_key);
        let address_str = hex::encode(address);
        let private_key = hex::encode(secret_key.as_ref());
        let data = format!("0x{}:{}", &address_str, &private_key);

        if address_str.starts_with(&desired_prefix) {
            let elapsed = start.elapsed().as_secs();
            println!("0x{} : {} sec.", address_str, elapsed);
            append_to_file(&data, "address_private".to_string()).expect("Error write in file address_private.txt");
            generated_wallets += 1;

        } else if has_repeated_chars(address_str.clone()) {
            append_to_file(&data, "repeated_address_private".to_string()).expect("Error write in file repeated_address_private.txt");
        }
    }

    running.store(false, Ordering::Relaxed); // Signal worker threads to stop

    println!("Saved {} wallets to wallets.txt", generated_wallets);
    input("Press 'Enter' to close.");
    Ok(())
}

fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line");
    buffer.trim().to_string()
}
