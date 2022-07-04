use web_sys::HtmlInputElement;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::hooks::{UseEthereumHandle, AddChainParams, NativeCurrency};

#[function_component]
pub fn SwitchNetwork() -> Html {
    let ethereum = use_context::<UseEthereumHandle>().expect(
        "no ethereum ethereum found. you must wrap your components in an <Ethereumethereum/>",
    );

    let on_switch_chain = {
        let ethereum = ethereum.clone();
        Callback::from(move |e: Event| {
            let select = e.target_unchecked_into::<HtmlInputElement>().value();

            let ethereum = ethereum.clone();
            
            spawn_local(async move {
                let chain_info = AddChainParams {
                    chainId: "0x38".to_string(),
                    chainName: "Smart Chain".to_string(),
                    rpcUrls: ["https://bsc-dataseed.binance.org/".to_string()],
                    nativeCurrency: NativeCurrency {
                        name: "Smart Chain".to_string(),
                        symbol: "BNB".to_string(), // 2-6 characters long
                        decimals: 18,
                    },
                    blockExplorerUrls: Some(["https://bscscan.com/".to_string()]),
                };
                ethereum.add_chain(chain_info).await;
                ethereum.switch_chain(select.to_string()).await;
            });
        })
    };

    html! {
        <div>
            <select onchange={on_switch_chain}>
                <option value="0x1">{ "Ethereum Network" }</option>
                <option value="0x38">{ "BSC Network" }</option>
            </select>
        </div>
    }
}
