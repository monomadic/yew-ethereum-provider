use yew::prelude::*;

mod components;
use components::*;

mod hooks;

#[function_component]
pub fn App() -> Html {
    // html! {
    //     <EthereumProvider>
    //         <ConnectButtonComponent />
    //     </EthereumProvider>
    // }
    //

    html! {
        <div>
            <ConnectButton />
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
    // yew::start_app::<Main>();
}
