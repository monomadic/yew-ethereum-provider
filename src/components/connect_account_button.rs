use crate::EthereumProvider;
use yew::prelude::*;

#[derive(Default)]
pub struct ConnectButtonComponent;

pub enum Msg {
    ClickedConnect,
    Connected,
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
                { format!("{:?}", ethereum.connection_status) }
            </div>
        }
    }
}
