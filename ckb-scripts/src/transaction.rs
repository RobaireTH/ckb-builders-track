use ckb_sdk::{
    constants::SIGHASH_TYPE_HASH,
    rpc::CkbRpcClient,
    traits::{
        DefaultCellCollector, DefaultCellDepResolver, DefaultHeaderDepResolver,
        DefaultTransactionDependencyProvider, SecpCkbRawKeySigner, CellCollector as CkbCellCollector, 
    },
    tx_builder::{transfer::CapacityTransferBuilder, CapacityBalancer, TxBuilder},
    unlock::{ScriptUnlocker, SecpSighashUnlocker},
    Address, HumanCapacity, ScriptId, NetworkType, 
};
use ckb_types::{
    bytes::Bytes,
    core::BlockView,
    h256,
    packed::{CellOutput, Script, WitnessArgs},
    prelude::*,
};
use std::collections::HashMap;
use std::str::FromStr;
use std::error::Error; 

// Define a struct to hold transaction parameters for clarity
pub struct TxParams {
    pub sender_address: Address,
    pub sender_secret_key_hex: String, // uses 'hex' to accept the sender address 
    pub receiver_address: Address,
    pub capacity_to_transfer: HumanCapacity,
    pub network_type: NetworkType,
}

// Build and send a transaction
pub fn build_and_send_transaction(
    ckb_client: &mut CkbRpcClient,
    params: TxParams,
) -> Result<String, Box<dyn Error>> {

    // Unlock script
    let sender_secret_key_bytes = hex::decode(&params.sender_secret_key_hex)?; // the hex is decoded here
    let secp_secret_key = secp256k1::SecretKey::from_slice(&sender_secret_key_bytes)?;
    let signer = SecpCkbRawKeySigner::new_with_secret_keys(vec![secp_secret_key]); // Signs the transaction
    let sighash_unlocker = SecpSighashUnlocker::from(Box::new(signer) as Box<_>);
    let sighash_script_id = ScriptId::new_type(SIGHASH_TYPE_HASH.clone());
    let mut unlockers = HashMap::default();
    unlockers.insert(
        sighash_script_id,
        Box::new(sighash_unlocker) as Box<dyn ScriptUnlocker>,
    );

    // Placeholder witness may be left empty for SIGHASH
    let placeholder_witness = WitnessArgs::new_builder()
        .lock(Some(Bytes::from(vec![0u8; 65])).pack())
        .build();
    let balancer = CapacityBalancer::new_simple(
        params.sender_address.payload().into(),
        placeholder_witness,
        1000, // The default Tx fee in Shannons, could be adjusted.
    );

    // Dependency Resolvers and Collectors 
    let rpc_url = ckb_client.get_url().to_string(); // Get URL from client
    let genesis_block = ckb_client.get_block_by_number(0.into())
        .map_err(|e| format!("Failed to get genesis block: {}", e))?
        .ok_or("Genesis block not found")?;
    let cell_dep_resolver = DefaultCellDepResolver::from_genesis(&BlockView::from(genesis_block))?;

    let header_dep_resolver = DefaultHeaderDepResolver::new(&rpc_url);
    let mut cell_collector = DefaultCellCollector::new(&rpc_url); // Use the correct RPC URL
    let tx_dep_provider = DefaultTransactionDependencyProvider::new(&rpc_url, 10);

    // Build the Transaction Output 
    let output = CellOutput::new_builder()
        .lock(Script::from(&params.receiver_address))
        .capacity(params.capacity_to_transfer.0.pack())
        .build();

    // Build the Transaction
    let builder = CapacityTransferBuilder::new(vec![(output, Bytes::default())]); // Assumes an empty cell state

    // Construct the transacion using the transaction builder
    let (tx, _signed_tx_info) = builder.build_unlocked(
        &mut cell_collector,
        &cell_dep_resolver,
        &header_dep_resolver,
        &tx_dep_provider,
        &balancer,
        &unlockers,
    )?;

    // Send the Transaction 
    let tx_hash = ckb_client.send_transaction(tx.clone())?; //clones the transaction for future use
    Ok(tx_hash.to_string())
}