use std::rc::Rc;
use web3::{
    transports::eip_1193::{Eip1193, Provider},
    futures::StreamExt,
};
use yew::{
    events::Event, html, Callback, Children, Component, Context, ContextProvider, Html, Properties, function_component, use_state
};
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

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AccountState {
    
    pub status: ConnectionStatus,
    pub web3: Web3Wrapper,
    pub accounts: Vec<String>,
}


impl AccountState {
    
    pub async fn connect(&self) -> Result<(), web3::Error> {
        self.web3.0.eth().request_accounts().await.map(|_| ())
    }

    pub async fn on<F>(&self, event_name: String, callback: F) 
    where
        F: Fn(String),
    {
        let provider = Provider::default().unwrap().unwrap();
        let transport: Eip1193 = Eip1193::new(provider);

        
        match event_name.clone() {
            connected => {
                let mut stream = transport.chain_changed_stream();

                while let Some(chainid) = stream.next().await {
                    callback(chainid.to_string());
                }
            },
            _ => {
            },
        }
        
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
            accounts: Vec::default(),
        })
    });

    html! {
        <ContextProvider<Rc<AccountState>> context={(*ctx).clone()}>
            {for props.children.iter()}
        </ContextProvider<Rc<AccountState>>>
    }
}
