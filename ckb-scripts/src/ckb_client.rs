use ckb_sdk::rpc::CkbRpcClient;

let testnet_url = "https://testnet.ckb.dev"; // Testnet
let devnet_url = "http://127.0.0.1:8114"; // Devnet
let mainnet_url = "https://mainnet.ckb.dev/rpc"; // Mainnet

// Connect to Testnet
let mut ckb_client = CkbRpcClient::new(testnet_url);