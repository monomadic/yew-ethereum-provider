use yew::prelude::*;
use yew_ethereum_provider::{AccountLabel, ConnectButton, EthereumContextProvider, SwitchNetwork};

#[function_component]
pub fn App() -> Html {
    html! {
        <div>
            <EthereumContextProvider>
                <SwitchNetwork />
                <ConnectButton />
                <AccountLabel />
            </EthereumContextProvider>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
