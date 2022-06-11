use crate::EthereumProvider;
use yew::prelude::*;

#[derive(Default)]
pub struct ConnectButtonComponent;

pub enum Msg {
    ClickedConnect,
    Connected,
    ChangedChain,
}

impl Component for ConnectButtonComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let (ethereum, _) = ctx
            .link()
            .context::<EthereumProvider>(Callback::noop())
            .expect("context to be set");

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
                    ethereum.switch_chain("0x1".to_string()).await;
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
                <button onclick={ctx.link().callback(|_| Msg::ChangedChain)}>{"Change Network"}</button>
                { format!("{:?}", ethereum.connection_status) }
            </div>
        }
    }
}
