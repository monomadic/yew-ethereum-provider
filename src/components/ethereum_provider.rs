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
        wasm_logger::init(wasm_logger::Config::default());
        
        
        if event_name == "accountsChanged" {
            let provider = Provider::default().unwrap().unwrap();
            let transport: Eip1193 = Eip1193::new(provider);
            let mut stream = transport.accounts_changed_stream();

            // let mut stream = self.transport.0.accounts_changed_stream();

            while let Some(accounts) = stream.next().await {
                callback(accounts[0].to_string());
            }
        } else if event_name == "chainChanged" {
            let provider = Provider::default().unwrap().unwrap();
            let transport: Eip1193 = Eip1193::new(provider);
            let mut stream = transport.chain_changed_stream();

            // let mut stream = self.transport.0.chain_changed_stream();

            while let Some(chainid) = stream.next().await {
                callback(chainid.to_string());
            }
        } else if event_name == "connect" {
            let provider = Provider::default().unwrap().unwrap();
            let transport: Eip1193 = Eip1193::new(provider);
            let mut stream = transport.connect_stream();

            // let mut stream = self.transport.0.connect_stream();

            while let Some(connect) = stream.next().await {
                log::info!("connect provider: ");
                callback(connect.unwrap().to_string());
            }
        } else if event_name == "disconnect" {
            let provider = Provider::default().unwrap().unwrap();
            let transport: Eip1193 = Eip1193::new(provider);
            let mut stream = transport.disconnect_stream();

            // let mut stream = self.transport.0.disconnect_stream();

            while let Some(error) = stream.next().await {
                log::info!("disconnect provider: {:?}", error);
                callback(error.to_string());
            }
        }
        
    }
    
    pub async fn on_chain_changed<F>(&self, callback: F) 
    where
        F: Fn(String),
    {
        self.on("chainChanged".to_string(), |chain_id|{
            callback(chain_id.to_string());
        }).await;
    }

    pub async fn on_account_changed<F>(&self, callback: F) 
    where
        F: Fn(String),
    {
        self.on("accountsChanged".to_string(), |address|{
            callback(address.to_string());
        }).await;
    }

    pub async fn on_connect<F>(&self, callback: F) 
    where
        F: Fn(String),
    {
        self.on("connect".to_string(), |connect|{
            log::info!("connect");
            callback(connect.to_string());
        }).await;
    }

    pub async fn on_disconnect<F>(&self, callback: F) 
    where
        F: Fn(String),
    {
        self.on("disconnect".to_string(), |error|{
            log::info!("disconnect");
            callback(error.to_string());
        }).await;
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
