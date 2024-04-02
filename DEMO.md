# Colosseum Renaissance Hackathon Demo

To fully unlock the demo, you'll need 2 different remote hosts.
But it's not mandatory, you see the basics working on a single host.

## Clone and build the repository

* `git clone https://github.com/block-mesh/block-mesh-monorepo.git`
* `cargo build`

## Launch the Proxy Master node

This will try to bind by default to port `0.0.0.0:5000` and `0.0.0.0:4000`.
Run `cargo run -p provider-node -- --help` to see the options.

`cargo run -p provider-node -- --keypair-path example-keys/provider-node.json`

## Launch the Proxy-Endpoint Node

