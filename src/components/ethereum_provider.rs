use web3::transports::eip_1193::{Eip1193, Provider};
use serde::Serialize;
use yew::{html, Children, Component, Context, ContextProvider, Html, Properties};
use js_sys::{JsString, Function};
use wasm_bindgen::{JsValue, prelude::*};

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

#[derive(Serialize)]
pub struct TransactionArgs {
    pub method: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub params: Vec<TransactionParam>,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum TransactionParam {
    Params(TransactionCallParams),
}

#[derive(Serialize, Default)]
pub struct TransactionCallParams {
    pub chain_id: String,
}

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(catch, js_namespace=["window", "ethereum"], js_name=request)]
    pub async fn ethereum_request(args: &JsValue) -> Result<JsValue, JsString>;

    #[wasm_bindgen(js_namespace=["window", "ethereum"], js_name=on)]
    pub fn on(event: &JsString, handler: &Function);
}

impl EthereumProvider {
    pub async fn connect(&self) {
        // TODO: remove unwrap, return Result
        self.web3.0.eth().request_accounts().await.unwrap();
    }

    pub async fn switch_chain (&self, chain_id: String) {
        ethereum_request(&JsValue::from_serde(&TransactionArgs {
            method: "wallet_switchEthereumChain".into(),
            params: vec![
                TransactionParam::Params(TransactionCallParams {
                    chain_id: chain_id.clone(),
                })
            ],
        }).unwrap()).await;
    }

}

impl Component for EthereumProvider {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        let provider = Provider::default().unwrap().unwrap();
        let transport: Eip1193 = Eip1193::new(provider);
        let _web3 = Web3Wrapper(web3::Web3::new(transport));

        Self {
            web3: _web3,
            connection_status: ConnectionStatus::default(),
            accounts: Vec::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AccountsChanged(_accounts) => {
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

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {}

    fn destroy(&mut self, _ctx: &Context<Self>) {}
}
