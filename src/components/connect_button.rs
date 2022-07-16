use crate::hooks::UseEthereumHandle;
use yew::prelude::*;
use yew_hooks::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    pub disconnected: Option<Html>,
}

#[function_component]
pub fn ConnectButton(props: &Props) -> Html {
    let ethereum = use_context::<UseEthereumHandle>().expect(
        "no ethereum ethereum found. you must wrap your components in an <Ethereumethereum/>",
    );

    let connect = {
        let ethereum = ethereum.clone();
        use_async(async move { ethereum.connect().await })
    };

    let disconnect = {
        let ethereum = ethereum.clone();
        Callback::from(move |_| ethereum.disconnect())
    };

    let disconnected_html = props.disconnected.clone().unwrap_or_else(|| {
        html! {
            <button>{"Disconnect"}</button>
        }
    });

    html! {
        <div>
            if ethereum.connected() {
                <div onclick={disconnect}>
                    {disconnected_html}
                </div>
            } else {
                <div onclick={ Callback::from(move |_| connect.run()) }>
                    { for props.children.iter() }
                </div>
            }
        </div>
    }
}
