use yew::prelude::*;
use yew_ethereum_provider::*;

#[function_component]
pub fn App() -> Html {
    html! {
        <div>
            <EthereumContextProvider>
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
