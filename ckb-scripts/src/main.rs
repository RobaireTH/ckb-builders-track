mod ckb_client; 
mod transaction;
mod address;

use ckb_sdk::rpc::CkbRpcClient;
use ckb_sdk::Address; 

fn main() {
    println!("Starting CKB script execution...");

    // Initialize CKB Client 
    let testnet_url = "https://testnet.ckb.dev"; 
    let mut ckb_client = CkbRpcClient::new(testnet_url);
    println!("CKB Client initialized for {}", testnet_url);

    // Generate both sender and receiver addresses 
    match address::generate_addresses() {
        Ok((sender_address, receiver_address)) => {
            println!("Sender Address: {}", sender_address.to_string());
            println!("Receiver Address: {}", receiver_address.to_string());

            // Build, Send transaction
            match transaction::build_and_send_transaction(
                &mut ckb_client,
                sender_address.clone(),
                receiver_address.clone(),
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