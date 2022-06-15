use web3::transports::eip_1193::Provider;
use yew::prelude::*;

pub struct UseEthereumHandle {
    inner: UseStateHandle<UseEthereumState>,
}

pub struct UseEthereumState {
    provider: Provider,
}

#[hook]
pub fn use_ethereum(default: Option<Provider>) -> UseEthereumHandle {
    let inner = use_state(move || UseEthereumState {
        provider: default.unwrap_or(Provider::default().unwrap().unwrap()),
    });

    UseEthereumHandle { inner }
}
