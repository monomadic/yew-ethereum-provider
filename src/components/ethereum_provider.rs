use std::sync::Arc;

use crate::hooks::UseEthereumHandle;
use yew::{function_component, html, use_state, Children, ContextProvider, Html, Properties};

use crate::hooks::use_ethereum;

#[derive(Clone)]
pub struct EthereumProviderState {
    pub ethereum: UseEthereumHandle,
}

impl PartialEq for EthereumProviderState {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(EthereumProvider)]
pub fn create(props: &Props) -> Html {
    let ethereum = use_ethereum(None);
    let ctx = use_state(|| Arc::new(EthereumProviderState { ethereum }));

    html! {
        <ContextProvider<Arc<EthereumProviderState>> context={(*ctx).clone()}>
            {for props.children.iter()}
        </ContextProvider<Arc<EthereumProviderState>>>
    }
}
