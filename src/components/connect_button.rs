use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::hooks::UseEthereumHandle;

#[function_component]
pub fn ConnectButton() -> Html {
    let ethereum = use_context::<UseEthereumHandle>().expect(
        "no ethereum ethereum found. you must wrap your components in an <Ethereumethereum/>",
    );

    let on_connect_clicked = {
        let ethereum = ethereum.clone();
        Callback::from(move |_| {
            let ethereum = ethereum.clone();
            spawn_local(async move {
                ethereum.connect("0x36".to_string()).await;
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
                <button onclick={on_disconnect_clicked}>
                    {"Disconnect "}
                    {ethereum.display_address()}
                </button>
            } else {
                <button onclick={on_connect_clicked}>{"Connect"}</button>
            }
        </div>
    }
}
