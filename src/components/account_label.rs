use crate::components::EthereumProviderState;
use std::sync::Arc;
use yew::prelude::*;

#[function_component]
pub fn AccountLabel() -> Html {
    let provider = use_context::<Arc<EthereumProviderState>>().expect(
        "no ethereum provider found. you must wrap your components in an <EthereumProvider/>",
    );

    html! {
        <div>
            {provider.ethereum.display_address()}
        </div>
    }
}
