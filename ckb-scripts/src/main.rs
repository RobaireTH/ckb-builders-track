mod ckb_client; 
mod transaction;
mod address;

use ckb_sdk::{Address, HumanCapacity, NetworkType};
use std::str::FromStr;

fn main() {
    println!("Starting CKB script execution...");

    // Initialize CKB Client 
    let mut ckb_client = ckb_client::get_testnet_client();
    println!("CKB Client initialized for Testnet");

    // Generate both sender and receiver addresses 
    let network = NetworkType::Testnet;
    match address::generate_addresses(network) {
        Ok(((sender_address, sender_privkey), (receiver_address, _))) => {
            println!("Sender Address: {}", sender_address.to_string());
            println!("Sender Private Key: {}", sender_privkey);
            println!("Receiver Address: {}", receiver_address.to_string());

            println!("IMPORTANT: The sender address is newly generated and has 0 CKB.");
            println!("You must fund it using a faucet before this transaction can succeed.");

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
        }
        Err(e) => {
            eprintln!("Failed to generate addresses: {:?}", e);
        }
    }

    println!("CKB script execution finished.");
}