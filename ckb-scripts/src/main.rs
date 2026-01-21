mod ckb_client; 
mod transaction;
mod address;

use ckb_sdk::{Address, HumanCapacity, NetworkType};
use std::str::FromStr;
use std::env;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    println!("Starting CKB script execution...");

    // Initialize CKB Client 
    let mut ckb_client = ckb_client::get_testnet_client();
    println!("CKB Client initialized for Testnet");

    // Generate both sender and receiver addresses 
    let network = NetworkType::Testnet;

    let (sender_address, sender_privkey) = match env::var("CKB_SENDER_KEY") {
        Ok(key) if !key.is_empty() => {
            match address::privkey_to_address(network, &key) {
                Ok(addr) => {
                     println!("Loaded Sender Address from CKB_SENDER_KEY env var.");
                     (addr, key)
                },
                Err(e) => {
                    eprintln!("Error deriving address from CKB_SENDER_KEY: {:?}", e);
                    return;
                }
            }
        },
        _ => {
            println!("CKB_SENDER_KEY not found. Generating new sender address.");
            match address::generate_ckb_address(network) {
                Ok(pair) => pair,
                Err(e) => {
                    eprintln!("Failed to generate sender address: {:?}", e);
                    return;
                }
            }
        }
    };

    // Generate a new receiver address
    let (receiver_address, _receiver_key) = match address::generate_ckb_address(network) {
        Ok(pair) => pair,
        Err(e) => {
            eprintln!("Failed to generate receiver address: {:?}", e);
            return;
        }
    };

    println!("Sender Address: {}", sender_address.to_string());
    // Only print private key if we generated it, or if user wants to see it? 
    // The user script printed it before, so we will print it again but maybe mask it if loaded? 
    // The user explicit asked to "keep the function... needed to generate both", implying dev/test context.
    println!("Sender Private Key: {}", sender_privkey);
    println!("Receiver Address: {}", receiver_address.to_string());

    if env::var("CKB_SENDER_KEY").is_err() {
        println!("IMPORTANT: The sender address is newly generated and has 0 CKB.");
        println!("You must fund it using https://faucet.nervos.org/ before this transaction can succeed.");
    }

    // Build, Send transaction
    let tx_params = transaction::TxParams {
        sender_address: sender_address.clone(),
        sender_secret_key_hex: sender_privkey,
        receiver_address: receiver_address.clone(),
        capacity_to_transfer: HumanCapacity::from_str("100.0").unwrap(),
        network_type: network,
    };

    match transaction::build_and_send_transaction(
        &mut ckb_client,
        tx_params,
    ) {
        Ok(tx_hash) => {
            println!("Transaction sent successfully! Tx Hash: {}", tx_hash);
        }
        Err(e) => {
            eprintln!("Failed to send transaction: {:?}", e);
        }
    }

    println!("CKB script execution finished.");
}