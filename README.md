# EthereumProvider

## Running the example

NOTE: if you are targeting `wasm32-unknown-unknown` on `aarch64` (Macbook M1/M2), remember
to ensure rust isn't installed via homebrew, use rustup. I've had so many errors via
the homebrew install for wasm32 targets.

```bash
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown
cargo install --locked wasm-bindgen-cli

# First, install cargo dependencies (if you don't have them).
cargo install trunk

# Then start the trunk server for the example file
cd examples/simple
trunk serve
```

## Usage

Add to your cargo.toml
```yaml
yew-ethereum-provider = { git = "https://github.com/monomadic/yew-ethereum-provider" }
```

Use the provider as a component like so:
```rust
use yew::prelude::*;
use yew_ethereum_provider::{
    chain, AccountLabel, ConnectButton, EthereumContextProvider, SwitchNetworkButton,
};

#[function_component]
pub fn App() -> Html {
    html! {
        <div>
            <EthereumContextProvider>
                <ConnectButton />
                <SwitchNetworkButton chain={chain::ethereum()}/>
                <SwitchNetworkButton chain={chain::avalanche_testnet()}/>
                <AccountLabel />
            </EthereumContextProvider>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
```
