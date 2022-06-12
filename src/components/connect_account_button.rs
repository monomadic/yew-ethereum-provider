use super::ethereum_provider::AccountState;
use std::rc::Rc;
use yew::prelude::*;
use yew_hooks::use_async;

#[function_component(ConnectButtonComponent)]
pub fn create() -> Html {
    let account = use_context::<Rc<AccountState>>().expect("No context found.");
    let connect = use_async(async move { account.connect().await });
    let onclick = Callback::from(move |_| connect.run());

    html! {
        <div>
            <button onclick={onclick}>{"Connect"}</button>
            // { format!("{:?}", account) }
        </div>
    }
}
