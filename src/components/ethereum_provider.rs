use wasm_bindgen_futures::spawn_local;
use web3::futures::StreamExt;
use web3::transports::eip_1193::{Eip1193, Provider};
use yew::{html, Callback, Children, Component, Context, ContextProvider, Html, Properties};

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
pub struct EthereumProvider {
    pub web3: Web3Wrapper,
    pub connection_status: ConnectionStatus,
    pub accounts: Vec<String>,
}

impl EthereumProvider {
    pub async fn connect(&self) {
        // TODO: remove unwrap, return Result
        self.web3.0.eth().request_accounts().await.unwrap();
    }
}

impl Component for EthereumProvider {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let provider = Provider::default().unwrap().unwrap();
        let transport: Eip1193 = Eip1193::new(provider);
        let _web3 = Web3Wrapper(web3::Web3::new(transport));

        // spawn_local(async move {
        //     let provider = Provider::default().unwrap().unwrap();
        //     let transport: Eip1193 = Eip1193::new(provider);
        //     let mut stream = transport.clone().accounts_changed_stream();
        //     while let Some(accounts) = stream.next().await {
        //         ctx.link().send_message(Msg::AccountsChanged(Vec::new()));
        //     }
        // });

        Self {
            web3: _web3,
            connection_status: ConnectionStatus::default(),
            accounts: Vec::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AccountsChanged(accounts) => {
                self.connection_status = ConnectionStatus::Connected;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <ContextProvider<EthereumProvider> context={self.clone()}>
                {for ctx.props().children.iter()}
            </ContextProvider<EthereumProvider>>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

    fn destroy(&mut self, ctx: &Context<Self>) {}
}
