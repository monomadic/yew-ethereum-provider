use web3::{
    transports::eip_1193::{Eip1193, Provider},
    types::H160,
};
use yew::prelude::*;

#[derive(Clone)]
pub struct UseEthereumHandle {
    provider: Provider,
    inner: UseStateHandle<UseEthereumState>,
}

impl UseEthereumHandle {
    pub async fn connect(&self) {
        log::info!("connect()");
        let web3 = web3::Web3::new(Eip1193::new(self.provider.clone()));
        if let Ok(addresses) = web3.eth().request_accounts().await {
            self.inner.set(UseEthereumState {
                connected: true,
                addresses: Some(addresses),
            });
        };
    }

    pub fn disconnect(&self) {
        log::info!("disconnect()");
        self.inner.set(UseEthereumState {
            connected: false,
            addresses: None,
        });
    }

    pub fn connected(&self) -> bool {
        self.inner.connected
    }

    pub fn address(&self) -> Option<&H160> {
        self.inner.addresses.as_ref().and_then(|a| a.first())
    }

    pub fn display_address(&self) -> String {
        self.address().map(|a| a.to_string()).unwrap_or_default()
    }
}

#[derive(Default, Clone)]
pub struct UseEthereumState {
    connected: bool,
    addresses: Option<Vec<H160>>,
}

#[hook]
pub fn use_ethereum(default: Option<Provider>) -> UseEthereumHandle {
    let inner = use_state(move || UseEthereumState::default());

    UseEthereumHandle {
        provider: default.unwrap_or(Provider::default().unwrap().unwrap()),
        inner,
    }
}
