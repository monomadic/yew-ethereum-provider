use wasm_bindgen::JsValue;
use web3::{
    futures::StreamExt,
    transports::eip_1193::{Chain, ERC20Asset, Eip1193, Provider},
    types::H160,
};
use yew::{platform::spawn_local, prelude::*};

#[derive(Clone, Debug)]
pub struct UseEthereumHandle {
    pub provider: Provider,
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

impl UseEthereumHandle {
    pub async fn connect(&self) -> Result<(), String> {
        log::info!("connect()");
        let web3 = web3::Web3::new(Eip1193::new(self.provider.clone()));

        if let Ok(addresses) = web3.eth().request_accounts().await {
            log::info!("request_accounts() {:?}", addresses);

            self.connected.set(true);
            self.accounts.set(Some(addresses));

            let chain_id = web3.eth().chain_id().await.ok().map(|c| c.to_string());
            self.chain_id.set(chain_id);

            {
                let this = self.clone();
                spawn_local(async move {
                    let this = this.clone();
                    this.on_chain_changed(|chain_id| {
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
        Ok(())
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

    /// returns the chain_id as a decimal. returns None on invalid chain values
    pub fn chain_id(&self) -> Option<i64> {
        self.chain_id
            .as_ref()
            .map(|chain_id| i64::from_str_radix(chain_id.trim_start_matches("0x"), 16).ok())
            .unwrap_or(None)
    }

    pub fn display_short_address(&self) -> String {
        self.address().map(|a| a.to_string()).unwrap_or_default()
    }

    pub fn display_address(&self) -> String {
        self.address()
            .map(|add| format!("{:?}", add))
            .expect("could not display address")
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

    /// switch chain or prompt user to add chain
    ///
    /// # Arguments
    /// * `chain` - a `Chain` instance representing the target chain
    ///
    pub async fn switch_chain_with_fallback(&self, chain: &Chain) -> Result<(), JsValue> {
        self.add_chain(chain).await?;
        self.switch_chain(&chain.chain_id).await?;
        Ok(())
    }

    /**
     * EIP-3326: Switch a wallet to another chain
     * https://eips.ethereum.org/EIPS/eip-3326
     * https://docs.metamask.io/guide/rpc-api.html#other-rpc-methods
     *
     * @param {number} chainId network chain identifier
     */
    pub async fn switch_chain(&self, chain_id: &str) -> Result<JsValue, JsValue> {
        log::info!("switch_chain");
        let transport = Eip1193::new(self.provider.clone());
        transport
            .switch_chain(chain_id)
            .await
            .map(|_| JsValue::from(chain_id))
            .map_err(|_| JsValue::from("error deserializing request params"))
    }

    /// EIP-3085: Add a wallet to another chain
    /// - https://eips.ethereum.org/EIPS/eip-3085
    /// - https://docs.metamask.io/guide/rpc-api.html#wallet-addethereumchain
    pub async fn add_chain(&self, chain: &Chain) -> Result<(), JsValue> {
        log::info!("add_chain");

        let transport = Eip1193::new(self.provider.clone());
        transport
            .add_chain(chain)
            .await
            .map(|_| ())
            .map_err(|_| JsValue::from("error deserializing request params"))
    }

    pub async fn watch_asset(&self, asset: &ERC20Asset) -> Result<(), JsValue> {
        log::info!("add_chain");

        let transport = Eip1193::new(self.provider.clone());
        transport
            .watch_asset(asset)
            .await
            .map(|_| ())
            .map_err(|_| JsValue::from("error deserializing request params"))
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
