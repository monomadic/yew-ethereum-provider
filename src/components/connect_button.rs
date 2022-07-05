use crate::hooks::UseEthereumHandle;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub connected_class: Option<String>,
    pub disconnected_class: Option<String>,
}

#[function_component]
pub fn ConnectButton(props: &Props) -> Html {
    let ethereum = use_context::<UseEthereumHandle>().expect(
        "no ethereum ethereum found. you must wrap your components in an <Ethereumethereum/>",
    );

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

    let ethereum = ethereum.clone();
    html! {
        <div>
            if ethereum.connected() {
                <button onclick={on_disconnect_clicked} class={props.disconnected_class.clone()}>
                    {"Disconnect "}
                    {ethereum.display_address()}
                </button>
            } else {
                <button onclick={on_connect_clicked} class={props.connected_class.clone()}>{"Connect"}</button>
            }
        </div>
    }
}
