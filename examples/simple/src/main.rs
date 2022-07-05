use yew::prelude::*;
use yew_ethereum_provider::{
    AccountLabel, Chain, ConnectButton, EthereumContextProvider, SwitchNetwork, SwitchNetworkButton,
};

#[function_component]
pub fn App() -> Html {
    html! {
        <div>
            <EthereumContextProvider>
                <SwitchNetwork />
                <ConnectButton />
                <SwitchNetworkButton chain={Chain::ethereum()}/>
                <SwitchNetworkButton chain={Chain::avalanche_testnet()}/>
                <AccountLabel />
            </EthereumContextProvider>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}