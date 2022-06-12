use std::rc::Rc;

use crate::EthereumProvider;
use wasm_bindgen_futures::spawn_local;
use web3::helpers::CallFuture;
use yew::prelude::*;

use super::ethereum_provider::AccountState;

// #[derive(Default)]
// pub struct ConnectButtonComponent;

pub enum Msg {
    ClickedConnect,
    Connected,
    ChangedChain,
}

#[function_component(ConnectButtonComponent)]
pub fn create() -> Html {
    wasm_logger::init(wasm_logger::Config::default());

    let greeting = use_state(|| "No one has greeted me yet!".to_string());

    let account = use_context::<Rc<AccountState>>().expect("No context found.");

    let provider = account.clone();

    {
        let greeting = greeting.clone();
        spawn_local(async move {

            provider.on("connected".to_string(), move |chain_id| {
                greeting.set(chain_id.to_string());
            }).await;    
        });
    }

    // let aa = greeting.to_string();
    let connect_wallet = Callback::from(move |_| {
        
        let acc = account.clone();

        spawn_local(async move {
            acc.connect().await;
        });
    });

    

    html! {
        <div>
            <button onclick={connect_wallet}>{"Connect"}</button>
            <div>
            <span>{"Chain Id: "}</span>{ (*greeting).to_string() }
            </div>
        </div>
    }
}

// impl Component for ConnectButtonComponent {
//     type Message = Msg;
//     type Properties = ();
//
//     fn create(_ctx: &Context<Self>) -> Self {
//         Self::default()
//     }
//
//     fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
//         // let (ethereum, _) = ctx
//         //     .link()
//         //     .context::<EthereumProvider>(Callback::noop())
//         //     .expect("context to be set");
//
//         let account = use_context::<AccountState>().expect("No context found.");
//
//         match msg {
//             Msg::ClickedConnect => {
//                 ctx.link().send_future(async move {
//                     // ethereum.connect().await;
//                     // account.connect().await;
//                     Msg::Connected
//                 });
//                 false
//             }
//             Msg::Connected => true, // update view
//         }
//     }
//
//     fn view(&self, ctx: &Context<Self>) -> Html {
//         // let (ethereum, _) = ctx
//         //     .link()
//         //     .context::<EthereumProvider>(Callback::noop())
//         //     .expect("context to be set");
//
//         let account = use_context::<AccountState>().expect("No context found.");
//
//         html! {
//             <div>
//                 <button onclick={ctx.link().callback(|_| Msg::ClickedConnect)}>{"Connect"}</button>
//                 // { format!("{:?}", ethereum) }
//             </div>
//         }
//     }
// }
