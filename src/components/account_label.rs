use crate::hooks::UseEthereumHandle;
use yew::prelude::*;

#[function_component]
pub fn AccountLabel() -> Html {
    let ethereum = use_context::<UseEthereumHandle>().expect(
        "no ethereum provider found. you must wrap your components in an <EthereumProvider/>",
    );

    html! {
        <div>
            if ethereum.connected() {
                {ethereum.display_address()}
            } else {
                {"disconnected"}
            }
        </div>
    }
}
