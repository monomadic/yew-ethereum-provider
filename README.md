# Yew-Ethereum Template

This is a yew frontend template useful both for demonstrating and bootstrapping
a project utilising 100% rust, talking to web3 via compatible EIP1993 clients
such as metamask.

## Installation

### Cargo

```bash
rustup default nightly

# First, install cargo dependencies (if you don't have them).
cargo install trunk
cargo install cargo-generate

# Then generate a new project with this template.
cargo generate monomadic/yew-ethereum-template

# Then start the trunk server.
trunk serve
```

### Nix

```bash
nix develop
trunk serve
```

## Recreating This Template

If you don't want to use cargo generate, here are the basic steps to reproduce this
project.

## Resources
- https://yew.rs/
- https://trunkrs.dev/
