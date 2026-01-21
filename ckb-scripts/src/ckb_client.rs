use ckb_sdk::rpc::CkbRpcClient;

pub const TESTNET_URL: &str = "https://testnet.ckb.dev";
pub const DEVNET_URL: &str = "http://127.0.0.1:8114";
pub const MAINNET_URL: &str = "https://mainnet.ckb.dev/rpc";

pub fn get_testnet_client() -> CkbRpcClient {
    CkbRpcClient::new(TESTNET_URL)
}