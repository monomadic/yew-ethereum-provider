use std::rc::Rc;
use web3::transports::eip_1193::{Eip1193, Provider};
use yew::{
    events::Event, html, Callback, Children, Component, Context, ContextProvider, Html, Properties
};
use yew::{function_component, use_state};
use yew::{html, Children, ContextProvider, Properties};

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


// type Callback = fn();
impl AccountState {
    
    pub async fn connect(&self) -> Result<(), web3::Error> {
        wasm_logger::init(wasm_logger::Config::default());
        log::info!("connect");
        self.web3.0.eth().request_accounts().await.map(|_| ())
    }

    pub async fn on<F>(&self, event_name: String, callback: F) 
    where
        F: Fn(String),
    {
        wasm_logger::init(wasm_logger::Config::default());
        log::info!("chain changed");
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
                log::info!("no match");
            },
        }
        
    }
}

#[function_component(EthereumProvider)]
pub fn create(props: &Props) -> Html {
    wasm_logger::init(wasm_logger::Config::default());
    
    let provider = Provider::default().unwrap().unwrap();
    let transport: Eip1193 = Eip1193::new(provider);
    // let ts = transport.clone();
    let web3 = Web3Wrapper(web3::Web3::new(transport));

    // let mut stream = ts.chain_changed_stream();
    
    let ctx = use_state(|| {
        Rc::new(AccountState {
            status: ConnectionStatus::default(),
            web3,
            accounts: Vec::default(),
        })
    });

    // spawn_local(async move {
    //     while let Some(chainid) = stream.next().await {
    //         log::info!("chain changed {:?}", &chainid);
    //         // callback(chainid.to_string());
    //     }
    // });

    

    html! {
        <ContextProvider<Rc<AccountState>> context={(*ctx).clone()}>
            {for props.children.iter()}
        </ContextProvider<Rc<AccountState>>>
    }
}
