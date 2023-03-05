use yew::prelude::*;

use crate::UseEthereumHandle;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    pub connected_html: Option<Html>,
}

#[function_component]
pub fn ConnectButton(props: &Props) -> Html {
    let ethereum = use_context::<UseEthereumHandle>().expect(
        "no ethereum ethereum found. you must wrap your components in an <Ethereumethereum/>",
    );

    let connect = {
        let ethereum = ethereum.clone();
        Callback::from(move |_| {
            let ethereum = ethereum.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let _ = ethereum.connect().await;
            });
        })
    };

    let disconnect = {
        let ethereum = ethereum.clone();
        Callback::from(move |_| ethereum.disconnect())
    };

    let short_address = ethereum.display_short_address();

    let connected_html = props.connected_html.clone().unwrap_or_else(|| {
       html! {
            <div class={classes!("hover:shadow", "shadow", "btn", "connected")}>
                <img src="./images/providers/metamask.svg" height="24" width="24" alt="metamask" class="inline-flex mr-2" />
                {short_address}
            </div>
       }
   });

    html! {
        <div>
            if ethereum.connected() {
                <div onclick={disconnect}>
                    {connected_html}
                </div>
            } else {
                <div onclick={connect}>
                    <div class={classes!("btn", "btn-primary", "disconnected")}>
                        {"Connect Wallet"}
                    </div>
                </div>
            }
        </div>
    }
}
