use web3::transports::eip_1193::Provider;
use yew::prelude::*;

#[derive(Clone)]
pub struct UseEthereumHandle {
    provider: Provider,
    inner: UseStateHandle<UseEthereumState>,
}

impl UseEthereumHandle {
    pub fn connect(&self) {
        log::info!("connect()");
        self.inner.set(UseEthereumState { connected: true });
    }

    pub fn disconnect(&self) {
        log::info!("disconnect()");
        self.inner.set(UseEthereumState { connected: false });
    }

    pub fn connected(&self) -> bool {
        self.inner.connected
    }
}

#[derive(Default, Clone)]
pub struct UseEthereumState {
    connected: bool,
}

#[hook]
pub fn use_ethereum(default: Option<Provider>) -> UseEthereumHandle {
    let inner = use_state(move || UseEthereumState::default());

    UseEthereumHandle {
        provider: default.unwrap_or(Provider::default().unwrap().unwrap()),
        inner,
    }
}
