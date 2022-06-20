use web_sys::HtmlInputElement;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::hooks::UseEthereumHandle;

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
                ethereum.add_chain(select.to_string()).await;
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
