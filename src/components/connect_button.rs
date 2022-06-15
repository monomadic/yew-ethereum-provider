use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::prelude::*;

use yew_hooks::use_async;

use crate::hooks::use_ethereum;

#[function_component]
pub fn ConnectButton() -> Html {
    let ethereum = use_ethereum(None);

    let on_connect_clicked = {
        let ethereum = ethereum.clone();
        Callback::from(move |_| {
            let ethereum = ethereum.clone();
            spawn_local(async move {
                ethereum.connect().await;
            });
        })
    };

    let on_disconnect_clicked = {
        let ethereum = ethereum.clone();
        Callback::from(move |_| ethereum.disconnect())
    };

    html! {
        <div>
        if ethereum.connected() {
            <button onclick={on_disconnect_clicked}>{"Disconnect"}</button>
        } else {
            <button onclick={on_connect_clicked}>{"Connect"}</button>
        }
        {&ethereum.connected()}
        </div>
    }
}
