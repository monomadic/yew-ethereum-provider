use std::rc::Rc;
use web3::{
    futures::StreamExt,
    transports::eip_1193::{Eip1193, Provider},
};
use yew::{
    events::Event, function_component, html, use_state, Callback, Children, Component, Context,
    ContextProvider, Html, Properties,
};

#[derive(Clone, Debug)]
pub struct TransportWrapper(Eip1193);
impl PartialEq for TransportWrapper {
    fn eq(&self, _other: &Self) -> bool {
        false // don't trigger diff update
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum ConnectionStatus {
    #[default]
    Disconnected,
    Connected,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AccountState {
    pub transport: TransportWrapper,
}

impl AccountState {
    pub async fn connect(&self) -> Result<Vec<web3::types::H160>, web3::Error> {
        log::info!("connect() called");
        let web3 = web3::Web3::new(&self.transport.0);
        web3.eth().request_accounts().await
    }

    pub async fn on_chain_changed<F>(&self, callback: F)
    where
        F: Fn(String),
    {
        log::info!("registered on_chain_changed");

        while let Some(chainid) = self.transport.0.chain_changed_stream().next().await {
            callback(chainid.to_string());
        }
    }

    pub async fn on_account_changed<F>(&self, callback: F)
    where
        F: Fn(Vec<web3::types::H160>),
    {
        while let Some(chainid) = self.transport.0.accounts_changed_stream().next().await {
            callback(chainid);
        }
    }

    pub async fn on_connect<F>(&self, callback: F)
    where
        F: Fn(Option<String>),
    {
        while let Some(connect) = self.transport.0.connect_stream().next().await {
            callback(connect);
        }
    }

    pub async fn on_disconnect<F>(&self, callback: F)
    where
        F: Fn(String),
    {
        while let Some(err) = self.transport.0.disconnect_stream().next().await {
            callback(err.to_string());
        }
    }
}

#[function_component(EthereumProvider)]
pub fn create(props: &Props) -> Html {
    let provider = Provider::default().unwrap().unwrap();
    let transport = TransportWrapper(Eip1193::new(provider));
    let ctx = use_state(|| Rc::new(AccountState { transport }));

    html! {
        <ContextProvider<Rc<AccountState>> context={(*ctx).clone()}>
            {for props.children.iter()}
        </ContextProvider<Rc<AccountState>>>
    }
}
