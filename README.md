# BlockMesh

<p align="center" width="100%">
    <img width="33%" src="https://imagedelivery.net/3RKw_J_fJQ_4KpJP3_YgXA/e4f3cdc0-c2ba-442d-3e48-e2f31c0dc100/public"> 
</p>

BlockMesh, is an innovative, open and secure network that allows you to easily monetize your excess bandwidth.
Giving you a great opportunity to passively profit and participate in the frontline of AI data layer, online privacy,
open source and blockchain industries.

## Repository Structure

* [libs/renaissance-hackathon](libs/renaissance-hackathon) -
  BlockMesh [Colosseum Renaissance hackathon entry](https://www.colosseum.org/renaissance)
* [programs/blockmesh-program](programs/blockmesh-program/) - [Anchor](https://www.anchor-lang.com/)
  [Solana](https://solana.com/) program , manages the state of the BlockMesh network.
* [libs/block-mesh-manager](libs/block-mesh-manager/) - The BlockMesh network server.
* [libs/block-mesh-solana-client](libs/block-mesh-solana-client/) - [RPC](https://solana.com/docs/rpc)
  client and `Instruction` to interact with the BlockMesh program.
* [libs/client-node](libs/client-node/) - The BlockMesh network client users run,
  it creates a local `proxy` that manages the interaction with `solana` and forward requests to BlockMesh network.
* [libs/cloudflare-worker-ip-data](libs/cloudflare-worker-ip-data/) - A cloudflare worker that pings `IP` data providers
  and responds with the data back to the caller - `curl https://cloudflare-worker-ip-data.blockmesh.xyz/`.
* [lib/cloudflare-worker-solana-relay](libs/cloudflare-worker-solana-relay/) - A cloudflare worker that relays requests
  to `solana`.
* [libs/cloudflare-landing-page](libs/cloudflare-landing-page) - The BlockMesh network landing site.
* [libs/proxy-master](libs/proxy-master/) - The `proxy-master` node,
  manages interactions with `solana` and incoming requests from `client-node`.
* [libs/solana-tiny-client](libs/solana-tiny-client/) - A tiny `solana` client with minimal dependencies
  hence making it support [WASM](https://webassembly.org/).
* [example-keys](example-keys/) - `solana` keys for `testing`/`demo`.

## Links

* [BlockMesh GitBoook](https://gitbook.blockmesh.xyz/)
* [BlockMesh Twitter](https://twitter.com/blockmesh_xyz)

## Setup

Add `.env` at the root of the repo:

```
export LINODE_ACCESS_TOKEN=""
export MAILGUN_SEND_KEY=""
export BLOCKMESH_LOG_ENV="dev"
export BLOCKMESH_SERVER_UUID="11111111-1111-4111-8111-111111111111"
export SENTRY=""
export MAPBOX=""
export AWS_ACCESS_KEY_ID=""
export AWS_SECRET_ACCESS_KEY=""
export TWITTER_API_KEY=""
export TWITTER_API_SECRET_KEY=""
export TWITTER_BEARER_TOKEN=""
export TWITTER_ACCESS_TOKEN=""
export TWITTER_ACCESS_TOKEN_SECRET=""
export TWITTER_CALLBACK_URL=""
export TWITTER_API_TOKEN=""
export TWITTER_API_URL=""
export TWITTER_API_HOST=""
export TWITTER_API_TOKEN_TOKEN=""
export TWITTER_CLIENT_ID=""
export TWITTER_CLIENT_SECRET=""
```

Install the following:

* `cargo install cargo-leptos --version=0.2.17`
* `cargo install sqlx-cli --verison=0.7.3`
* `cargo install wasm-pack --version=0.12.1`
* `rustup target add wasm32-unknown-unknown`
* `cargo install bunyan`
* [Install psql](https://www.timescale.com/blog/how-to-install-psql-on-mac-ubuntu-debian-windows/)
* [Install Docker](https://docs.docker.com/engine/install/)

Run `./scripts/run_local.sh`