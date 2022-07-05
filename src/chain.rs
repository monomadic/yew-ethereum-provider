use crate::BaseCurrency;
use serde::Serialize;

#[derive(Serialize, Default, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Chain {
    pub chain_id: String,
    pub chain_name: String,
    pub rpc_urls: [String; 1],
    pub native_currency: BaseCurrency,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_explorer_urls: Option<[String; 1]>,
}

impl Chain {
    pub fn ethereum() -> Self {
        Self {
            chain_id: "0x1".into(),
            chain_name: "Ethereum".into(),
            rpc_urls: [String::from("https://api.avax-test.network/ext/bc/C/rpc")],
            native_currency: BaseCurrency::eth(),
            block_explorer_urls: Some([String::from("https://api.avax-test.network/ext/bc/C/rpc")]),
        }
    }

    pub fn avalanche_testnet() -> Self {
        Self {
            chain_name: String::from("Avalanche Fuji Testnet"),
            chain_id: String::from("0xA869"),
            rpc_urls: [String::from("https://api.avax-test.network/ext/bc/C/rpc")],
            native_currency: BaseCurrency::avax(),
            block_explorer_urls: Some([String::from("https://api.avax-test.network/ext/bc/C/rpc")]),
        }
    }
}
