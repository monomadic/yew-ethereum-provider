use yew::prelude::*;
use yew_ethereum_provider::{
    chain, AccountLabel, ConnectButton, EthereumContextProvider, SwitchNetworkButton,
};

#[function_component]
pub fn App() -> Html {
    // this is optional
    let disconnected = html! {
        <button>{"Disconnect"}</button>
    };

    html! {
        <div>
            <EthereumContextProvider>
                <ConnectButton {disconnected}>
                    <button>{ "Connect" }</button>
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
