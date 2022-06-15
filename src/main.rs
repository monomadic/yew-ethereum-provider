use yew::prelude::*;

mod components;
use components::{ConnectButtonComponent, EthereumProvider};

mod hooks;

#[function_component(Main)]
pub fn main() -> Html {
    wasm_logger::init(wasm_logger::Config::default());

    html! {
        <EthereumProvider>
            <ConnectButtonComponent />
        </EthereumProvider>
    }
}

fn main() {
    yew::start_app::<Main>();
}
