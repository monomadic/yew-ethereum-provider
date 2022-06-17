use std::sync::Arc;

use crate::components::EthereumProviderState;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component]
pub fn ConnectButton() -> Html {
    let provider = use_context::<Arc<EthereumProviderState>>().expect(
        "no ethereum provider found. you must wrap your components in an <EthereumProvider/>",
    );

    let on_connect_clicked = {
        let provider = provider.clone();
        Callback::from(move |_| {
            let provider = provider.clone();
            spawn_local(async move {
                provider.ethereum.connect().await;
            });
        })
    };

    let on_disconnect_clicked = {
        let provider = provider.clone();
        Callback::from(move |_| provider.ethereum.disconnect())
    };

    html! {
        <div>
            if provider.ethereum.connected() {
                <button onclick={on_disconnect_clicked}>
                    {"Disconnect "}
                    {provider.ethereum.display_address()}
                </button>
            } else {
                <button onclick={on_connect_clicked}>{"Connect"}</button>
            }
        </div>
    }
}
