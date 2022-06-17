use web3::{
    futures::StreamExt,
    transports::eip_1193::{Eip1193, Provider},
    types::H160,
};
use yew::prelude::*;

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

impl UseEthereumHandle {
    pub async fn connect(&self) {
        log::info!("connect()");
        let web3 = web3::Web3::new(Eip1193::new(self.provider.clone()));
        if let Ok(addresses) = web3.eth().request_accounts().await {
            log::info!("request_accounts() {:?}", addresses);

            self.connected.set(true);
            self.accounts.set(Some(addresses));

            self.on_accounts_changed(move |addresses| {
                log::info!("event: accountsChanged");
                if addresses.is_empty() {
                    self.connected.set(false);
                }
                self.accounts.set(Some(addresses));
            })
            .await;

            self.on_chain_changed(move |chain_id| {
                log::info!("event: chainChanged: {}", chain_id);
                self.chain_id.set(Some(chain_id));
            })
            .await;

            self.on_connect(move |connect| {
                log::info!("event: connect: {:?}", connect);
                self.connected.set(true);
            })
            .await;

            self.on_disconnect(move |chain_id| {
                log::info!("event: disconnect: {}", chain_id);
                self.connected.set(false);
            })
            .await;
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
        while let Some(accounts) = transport.accounts_changed_stream().next().await {
            callback(accounts);
        }
    }

    pub async fn on_chain_changed<F>(&self, callback: F)
    where
        F: Fn(String),
    {
        let transport = Eip1193::new(self.provider.clone());
        while let Some(chainid) = transport.chain_changed_stream().next().await {
            callback(chainid.to_string());
        }
    }

    pub async fn on_connect<F>(&self, callback: F)
    where
        F: Fn(Option<String>),
    {
        let transport = Eip1193::new(self.provider.clone());
        while let Some(connect) = transport.connect_stream().next().await {
            callback(connect);
        }
    }

    pub async fn on_disconnect<F>(&self, callback: F)
    where
        F: Fn(String),
    {
        let transport = Eip1193::new(self.provider.clone());
        while let Some(err) = transport.disconnect_stream().next().await {
            callback(err.to_string());
        }
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
