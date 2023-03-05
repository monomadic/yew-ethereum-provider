use yew::prelude::*;
use yew_ethereum_provider::{
    chain, AccountLabel, ConnectButton, EthereumContextProvider, SwitchNetworkButton,
};

#[function_component]
pub fn App() -> Html {
    html! {
        <div>
            <EthereumContextProvider>
                <ConnectButton>
                    <button>{"Connect"}</button>
                </ConnectButton>
                <SwitchNetworkButton chain={chain::ethereum()}/>
                <SwitchNetworkButton chain={chain::avalanche_testnet()}/>
                <AccountLabel />
            </EthereumContextProvider>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
