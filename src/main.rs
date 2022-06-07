use yew::prelude::*;

mod components;
use components::{ EthereumContext, EthereumProvider, test_component::TestComponent };

#[function_component(Main)]
pub fn main() -> Html {

    html! {
        
        <EthereumProvider>
            <TestComponent />
        </EthereumProvider>
    }
}

fn main() {
    yew::start_app::<Main>();
}
