use std::ptr::null;

use web3::{
    futures::{StreamExt, TryFutureExt},
    transports::eip_1193::{Eip1193, Provider},
    types::H160,
};
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::{JsValue, prelude::*, JsCast};
use serde::Serialize;
use js_sys::{JsString, Function};

#[derive(Clone, Debug)]
pub struct UseEthereumHandle {
    provider: Provider,
    connected: UseStateHandle<bool>,
    accounts: UseStateHandle<Option<Vec<H160>>>,
    chain_id: UseStateHandle<Option<String>>,
}

impl PartialEq for UseEthereumHandle {
    fn eq(&self, other: &Self) -> bool {
        self.connected == other.connected
            && self.accounts == other.accounts
            && self.chain_id == other.chain_id
    }
}

#[derive(Serialize)]
pub struct TransactionArgs {
    pub method: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub params: Vec<TransactionParam>,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum TransactionParam {
    Params(TransactionCallParams),
    SwitchEthereumChainParameter(ChainId),
    AddEthereumChainParameter(AddChainParams),
    // WatchAssetParameter (WatchAssetParams),
    Tag(String),
}

#[derive(Serialize, Default)]
pub struct ChainId {
    pub chainId: String,
    
}

#[derive(Serialize, Default)]
pub struct NativeCurrency {
    name: String,
    symbol: String, // 2-6 characters long
    decimals: u32,

}

#[derive(Serialize, Default)]
pub struct AddChainParams {

    pub chainId: String,

    pub chainName: String,

    pub rpcUrls: [String;1],

    pub nativeCurrency : NativeCurrency,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockExplorerUrls: Option<[String; 1]>,
}

#[derive(Serialize, Default)]
pub struct TransactionCallParams {
    // MUST be the currently selected address (or the error 'MetaMask
    // RPC Error: Invalid parameters: must provide an Ethereum address.' will occur)
    pub from: String,
    
    // required except during contract creation
    pub to: String,
    
    /// (Optional) if present contract interaction or creation is assumed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,

    /// (Optional) Hex-encoded value of the network's native currency to send
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_number: Option<String>,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(catch, js_namespace=["window", "ethereum"], js_name=request)]
    pub async fn ethereum_request(args: &JsValue) -> Result<JsValue, JsString>;

    #[wasm_bindgen(js_namespace=["window", "ethereum"], js_name=on)]
    pub fn on(event: &JsString, handler: &Function);
}

impl UseEthereumHandle {
    pub async fn connect(&self, chain_id: String) {
        log::info!("connect()");
        let web3 = web3::Web3::new(Eip1193::new(self.provider.clone()));
        
        Self::add_chain(chain_id.clone()).await;
        Self::switch_chain(chain_id.clone()).await;
        
        if let Ok(addresses) = web3.eth().request_accounts().await {
            log::info!("request_accounts() {:?}", addresses);

            self.connected.set(true);
            self.accounts.set(Some(addresses));

            {
                let this = self.clone();
                spawn_local(async move {
                    let this = this.clone();
                    this.on_chain_changed(|chain_id| {
                        // log::info!("event: chainChanged: {}", chain_id);
                        log::info!("event: chainChanged {:?}", chain_id);
                        this.chain_id.set(Some(chain_id));
                    })
                    .await;
                });
            }

            {
                let this = self.clone();
                spawn_local(async move {
                    let this = this.clone();
                    log::info!("event: accountsChanged before");
                    this.on_accounts_changed(|addresses| {
                        log::info!("event: accountsChanged");
                        if addresses.is_empty() {
                            this.connected.set(false);
                        }
                        this.accounts.set(Some(addresses));
                    })
                    .await;
                });
            }

            {
                let this = self.clone();
                spawn_local(async move {
                    this.on_connect(|connect| {
                        log::info!("event: connect: {:?}", connect);
                        this.connected.set(true);
                    })
                    .await;
                });
            }

            {
                let this = self.clone();
                spawn_local(async move {
                    this.on_disconnect(|chain_id| {
                        log::info!("event: disconnect: {}", chain_id);
                        this.connected.set(false);
                    })
                    .await;
                });
            }
        };
    }

    pub fn disconnect(&self) {
        log::info!("disconnect()");
        self.connected.set(false);
    }

    pub fn connected(&self) -> bool {
        *self.connected
    }

    pub fn address(&self) -> Option<&H160> {
        self.accounts.as_ref().and_then(|a| a.first())
    }

    pub fn display_address(&self) -> String {
        self.address().map(|a| a.to_string()).unwrap_or_default()
    }

    pub async fn on_accounts_changed<F>(&self, callback: F)
    where
        F: Fn(Vec<web3::types::H160>),
    {
        let transport = Eip1193::new(self.provider.clone());
        let mut stream = transport.accounts_changed_stream();
        while let Some(accounts) = stream.next().await {
            log::info!("accounts changed");
            callback(accounts.clone());
        }
    }

    pub async fn on_chain_changed<F>(&self, callback: F)
    where
        F: Fn(String),
    {
        let transport = Eip1193::new(self.provider.clone());
        let mut stream = transport.chain_changed_stream();
        while let Some(chainid) = stream.next().await {
            callback(chainid.to_string());
        }
    }

    pub async fn on_connect<F>(&self, callback: F)
    where
        F: Fn(Option<String>),
    {
        let transport = Eip1193::new(self.provider.clone());
        let mut stream = transport.connect_stream();
        while let Some(connect) = stream.next().await {
            callback(connect);
        }
    }

    pub async fn on_disconnect<F>(&self, callback: F)
    where
        F: Fn(String),
    {
        let transport = Eip1193::new(self.provider.clone());
        let mut stream = transport.disconnect_stream();
        while let Some(err) = stream.next().await {
            callback(err.to_string());
        }
    }

    /**
    * EIP-3326: Switch a wallet to another chain
    * https://eips.ethereum.org/EIPS/eip-3326
    * https://docs.metamask.io/guide/rpc-api.html#other-rpc-methods
    *
    * @param {number} chainId network chain identifier
    */
    pub async fn switch_chain(chain_id: String) -> Result<JsValue, JsString> {
        log::info!("switch_chain");

        ethereum_request(&JsValue::from_serde(&TransactionArgs {
            method: "wallet_switchEthereumChain".into(),
            params: vec![
                TransactionParam::SwitchEthereumChainParameter( ChainId {
                    chainId: chain_id.into(),
                }),
            ],
        }).unwrap()).await
    }


    /**
    * EIP-3085: Add a wallet to another chain
    * https://eips.ethereum.org/EIPS/eip-3085
    * https://docs.metamask.io/guide/rpc-api.html#wallet-addethereumchain
    */
    pub async fn add_chain(chain_id: String) -> Result<JsValue, JsString> {
        log::info!("add_chain");
        ethereum_request(&JsValue::from_serde(&TransactionArgs {
            method: "wallet_addEthereumChain".into(),
            params: vec![
                TransactionParam::AddEthereumChainParameter( AddChainParams {
                    chainId: chain_id.clone(),
                    chainName: "Smart Chain".to_string(),
                    // nativeCurrency: config.baseCurrency,
                    rpcUrls: ["https://bsc-dataseed.binance.org/".to_string()],
                    nativeCurrency: NativeCurrency {
                        name: "Smart Chain".to_string(),
                        symbol: "BNB".to_string(), // 2-6 characters long
                        decimals: 18,
                    },
                    blockExplorerUrls: Some(["https://bscscan.com/".to_string()]),
                }),
            ],  
        }).unwrap()).await
    }
}

#[hook]
pub fn use_ethereum(default: Option<Provider>) -> UseEthereumHandle {
    let connected = use_state(move || false);
    let accounts = use_state(move || None as Option<Vec<H160>>);
    let chain_id = use_state(move || None as Option<String>);

    UseEthereumHandle {
        provider: default.unwrap_or_else(|| Provider::default().unwrap().unwrap()),
        connected,
        accounts,
        chain_id,
    }
}
