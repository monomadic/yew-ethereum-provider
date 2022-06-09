use yew::prelude::*;
use crate::EthereumContext;

#[derive(Default)]
pub struct TestComponent;

pub enum Msg {
    ClickedConnect,
    Connected(bool),
}


impl Component for TestComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let (ethereum, _) = ctx.link().context::<EthereumContext>(Callback::noop())
            .expect("context to be set");
        match msg {
            Msg::ClickedConnect => { 
                ctx.link().send_future(async move{
                    ethereum.web3.0.eth().request_accounts().await.unwrap();
                    Msg::Connected(true)
                });    
                true
            }
            Msg::Connected(_is_onnected) => { true }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <button onclick={ctx.link().callback(|_| Msg::ClickedConnect)}>{"Connect"}</button>
            </div>
        }
    }
}