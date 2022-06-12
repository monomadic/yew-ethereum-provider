use super::ethereum_provider::AccountState;
use std::rc::Rc;
use yew::prelude::*;
use yew_hooks::use_async;
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
    // let connect_wallet = Callback::from(move |_| {
        
    //     let acc = account.clone();

    //     spawn_local(async move {
    //         acc.connect().await;
    //     });
    // });
    let connect = use_async(async move { account.connect().await });
    let onclick = Callback::from(move |_| connect.run());

    

    html! {
        <div>
            <button onclick={onclick}>{"Connect"}</button>
            <div>
            <span>{"Chain Id: "}</span>{ (*greeting).to_string() }
            </div>
        </div>
    }
}
