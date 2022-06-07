use yew::{Component, Context, Html, html, ContextProvider, Properties, Children};
use web3::{
    transports::eip_1193::{Eip1193, Provider},
};

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Connection {
    pub connected: bool,
    pub is_loading: bool,
    pub error: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct EthereumContext {
    pub web3: web3::Web3<Eip1193>,
    pub connection: Connection,
}

pub enum Msg {
    
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[derive(Default)]
pub struct EthereumProvider;

impl Component for EthereumProvider {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    // fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    //     Ok(true)
    // }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let provider = Provider::default().unwrap().unwrap();
        let transport: Eip1193 = Eip1193::new(provider);
        let _web3 = web3::Web3::new(transport);

        let eth_ctx = EthereumContext {
            web3: _web3,
            connection: Connection {
                connected: false,
                is_loading: true,
                error: None,
            }
        };

        html! {
            <ContextProvider<EthereumContext> context={eth_ctx}>
                {for ctx.props().children.iter()}
            </ContextProvider<EthereumContext>>
        }
    }
}