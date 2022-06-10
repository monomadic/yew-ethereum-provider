use yew::prelude::*;

mod components;
use components::{ConnectButtonComponent, EthereumProvider};

#[function_component(Main)]
pub fn main() -> Html {
    html! {
        <EthereumProvider>
            <ConnectButtonComponent />
        </EthereumProvider>
    }
}

fn main() {
    yew::start_app::<Main>();
}
