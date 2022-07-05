use crate::hooks::{use_ethereum, UseEthereumHandle};
use yew::{function_component, html, Children, ContextProvider, Html, Properties};

#[derive(Clone, PartialEq)]
pub struct EthereumProviderState {
    pub ethereum: UseEthereumHandle,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn EthereumContextProvider(props: &Props) -> Html {
    let ethereum = use_ethereum(None);

    html! {
        <ContextProvider<UseEthereumHandle> context={ethereum}>
            {for props.children.iter()}
        </ContextProvider<UseEthereumHandle>>
    }
}
