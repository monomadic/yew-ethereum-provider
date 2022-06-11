use std::rc::Rc;

use wasm_bindgen_futures::spawn_local;
use web3::futures::StreamExt;
use web3::transports::eip_1193::{Eip1193, Provider};
use yew::{
    events::Event, html, Callback, Children, Component, Context, ContextProvider, Html, Properties,
};
use yew::{function_component, use_state};

#[derive(Clone, Debug)]
pub struct Web3Wrapper(pub web3::Web3<Eip1193>);
impl PartialEq for Web3Wrapper {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum ConnectionStatus {
    #[default]
    Disconnected,
    Connected,
}

pub enum Msg {
    AccountsChanged(Vec<String>),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EthereumProviderOld {
    pub connection_status: ConnectionStatus,
    pub accounts: Vec<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AccountState {
    pub status: ConnectionStatus,
    pub web3: Web3Wrapper,
}

impl AccountState {
    pub async fn connect(&self) -> Result<(), web3::Error> {
        self.web3.0.eth().request_accounts().await.map(|_| ())
    }
}

#[function_component(EthereumProvider)]
pub fn create(props: &Props) -> Html {
    let provider = Provider::default().unwrap().unwrap();
    let transport: Eip1193 = Eip1193::new(provider);
    let web3 = Web3Wrapper(web3::Web3::new(transport));

    let ctx = use_state(|| {
        Rc::new(AccountState {
            status: ConnectionStatus::default(),
            web3,
        })
    });

    html! {
        <ContextProvider<Rc<AccountState>> context={(*ctx).clone()}>
            {for props.children.iter()}
        </ContextProvider<Rc<AccountState>>>
    }
}
