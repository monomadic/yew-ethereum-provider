use crate::EthereumProvider;
use yew::prelude::*;
use web_sys::HtmlInputElement;

#[derive(Default)]
pub struct ConnectButtonComponent {
    select_switch_network: NodeRef,
}

pub enum Msg {
    ClickedConnect,
    Connected,
    ChangedChain,
}

impl Component for ConnectButtonComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            select_switch_network: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let (ethereum, _) = ctx
            .link()
            .context::<EthereumProvider>(Callback::noop())
            .expect("context to be set");
        let select = self.select_switch_network.cast::<HtmlInputElement>();
        match msg {
            Msg::ClickedConnect => {
                ctx.link().send_future(async move {
                    ethereum.connect().await;
                    // ethereum.web3.0.eth().request_accounts().await.unwrap();
                    Msg::Connected
                });
                false
            }

            Msg::ChangedChain => {
                ctx.link().send_future(async move {
                    ethereum.switch_chain(select.unwrap().value()).await;
                    // ethereum.web3.0.eth().request_accounts().await.unwrap();
                    Msg::Connected
                });
                false
            }

            Msg::Connected => true, // update view
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (ethereum, _) = ctx
            .link()
            .context::<EthereumProvider>(Callback::noop())
            .expect("context to be set");

        html! {
            <div>
                <button onclick={ctx.link().callback(|_| Msg::ClickedConnect)}>{"Connect"}</button>
                <br />
                <select 
                    ref={self.select_switch_network.clone()}
                    onchange={ctx.link().callback(|_| Msg::ChangedChain)}
                >
                    <option selected=true value="0x1">{ "Ethereum" }</option>
                    <option value="0x38">{ "BSC" }</option>
                </select>
                // <button onclick={ctx.link().callback(|_| Msg::ChangedChain)}>{"Change Network"}</button>
                <br />
                { format!("{:?}", ethereum.connection_status) }
            </div>
        }
    }
}
