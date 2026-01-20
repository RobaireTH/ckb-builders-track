use ckb_sdk::types::{Address, AddressPayload, NetworkType};
use ckb_sdk::secp256k1; 
use ckb_crypto::secp::SECP256K1; 
use rand::Rng;
use std::error::Error;

// Generate a single ckb address.
pub fn generate_ckb_address(network: NetworkType) -> Result<Address, Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let privkey_bytes: [u8; 32] = rng.gen();
    let secp_secret_key = secp256k1::SecretKey::from_slice(&privkey_bytes)?;
    let pubkey = secp256k1::PublicKey::from_secret_key(&SECP256K1, &secp_secret_key)?;
    let payload = AddressPayload::from_pubkey(&pubkey);
    let address = Address::new(network, payload, true);
    Ok(address)
}

// Generate both sender and receiver addresses.
pub fn generate_addresses(network: NetworkType) -> Result<(Address, Address), Box<dyn Error>> {
    let sender_address = generate_ckb_address(network)?;
    // The sender address normally wouldn't have to be gerenated using 'rand'. 
    let receiver_address = generate_ckb_address(network)?;
    Ok((sender_address, receiver_address))
}

