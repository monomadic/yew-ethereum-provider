use yew::{Component, Context, Html, html, Callback};
use crate::EthereumContext;

#[derive(Default)]
pub struct TestComponent;

pub enum Msg {
    
}


impl Component for TestComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    // fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    //     Ok(true)
    // }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (ethereum, _) = ctx.link().context::<EthereumContext>(Callback::noop())
            .expect("context to be set");

        html! {
            <div>
                { ethereum.connection.connected.clone() }
            </div>
        }
    }
}