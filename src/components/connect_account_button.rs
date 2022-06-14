use super::ethereum_provider::AccountState;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::use_async;

#[function_component(ConnectButtonComponent)]
pub fn create() -> Html {
    let address = use_state(|| "Not Connected!".to_string());
    let chain_id = use_state(|| "Not Connected".to_string());
    let connect_state = use_state(|| "Not Connected".to_string());
    let disconnect_state = use_state(|| "Not Connected".to_string());

    let account = use_context::<Rc<AccountState>>().expect("No context found.");

    let address_clone = (address).clone();
    let chain_id_clone = (chain_id).clone();
    let connect_state_clone = (connect_state).clone();
    let disconnect_state_clone = (disconnect_state).clone();

    let connect = use_async(async move {
        let connect = account.connect().await;

        let provider_for_chain = account.clone();
        spawn_local(async move {
            provider_for_chain
                .on_chain_changed(move |chain| {
                    log::info!("chainChanged");
                    chain_id_clone.set(chain.to_string());
                })
                .await;
        });

        let provider_for_accounts = account.clone();
        spawn_local(async move {
            provider_for_accounts
                .on_account_changed(move |account_address| {
                    log::info!("accountsChanged");
                    address_clone.set(format!("{:?}", account_address));
                })
                .await;
        });

        let provider_for_connect = account.clone();
        spawn_local(async move {
            provider_for_connect
                .on_connect(move |account_address| {
                    connect_state_clone.set(format!("{:?}", account_address));
                })
                .await;
        });

        let provider_for_disconnect = account.clone();
        spawn_local(async move {
            provider_for_disconnect
                .on_disconnect(move |account_address| {
                    disconnect_state_clone.set(account_address.to_string());
                })
                .await;
        });

        // let chain_id = chain_id.clone();

        return connect;
    });
    let onclick = Callback::from(move |_| connect.run());

    html! {
        <div>
            <button onclick={onclick}>{"Connect"}</button>
            <div>
            <span>{"Chain Id: "}</span>{ (*chain_id).to_string() }
            <br />
            <span>{"Address : "}</span>{ (*address).to_string() }
            <br />
            <span>{"connect : "}</span>{ (*connect_state).to_string() }
            <br />
            <span>{"disconnect : "}</span>{ (*disconnect_state).to_string() }
            </div>
        </div>
    }
}
