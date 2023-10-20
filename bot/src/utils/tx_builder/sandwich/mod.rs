use std::sync::Arc;

use crate::utils;
use ethers::prelude::{k256::ecdsa::SigningKey, *};
use tokio::sync::RwLock;

pub mod v2;
pub mod v3;

#[derive(Debug, Clone)]
pub struct SandwichMaker {
    pub v2: v2::SandwichLogicV2,
    pub v3: v3::SandwichLogicV3,
    pub sandwich_address: Address,
    pub searcher_wallet: Wallet<SigningKey>,
    pub nonce: Arc<RwLock<U256>>,
}

impl SandwichMaker {
    // Create a new `SandwichMaker` instance
    pub async fn new() -> Self {
        let sandwich_address = utils::dotenv::get_sandwich_contract_address();
        let searcher_wallet = utils::dotenv::get_searcher_wallet();

        let client = utils::create_websocket_client().await.unwrap();

        let nonce = if let Ok(n) = client
            .get_transaction_count(searcher_wallet.address(), None)
            .await
        {
            n
        } else {
            panic!("Failed to get searcher wallet nonce...");
        };

        let nonce = Arc::new(RwLock::new(nonce));

        Self {
            v2: v2::SandwichLogicV2::new(),
            v3: v3::SandwichLogicV3::new(),
            sandwich_address,
            searcher_wallet,
            nonce,
        }
    }

    pub async fn update_searcher_nonce(&self) {
        let mut nonce = self.nonce.write().await;
        let client = utils::create_websocket_client().await.unwrap();

        if let Ok(n) = client
            .get_transaction_count(self.searcher_wallet.address(), None)
            .await
        {
            *nonce = n;
        } else {
            panic!("Failed to update searcher wallet nonce...");
        };
    }
}

/// Encoded swap value used by other token
pub struct EncodedSwapValue {
    encoded_value: U256,
    mem_offset: U256,
    // real value after encoding
    byte_shift: U256,
}

impl EncodedSwapValue {
    fn new(encoded_value: U256, mem_offset: U256, byte_shift: U256) -> Self {
        Self {
            encoded_value,
            mem_offset,
            byte_shift,
        }
    }

    // returns the decoded value after applying byteshift (real value used during swaps)
    fn decode(&self) -> U256 {
        self.encoded_value * (U256::from(2).pow(U256::from(8) * self.byte_shift))
    }
}

/// Return the divisor used for encoding call value (weth amount)
pub fn get_weth_encode_divisor() -> U256 {
    U256::from_dec_str("4294967296").unwrap()
}
