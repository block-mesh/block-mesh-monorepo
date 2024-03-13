# BlockMesh

## Repository Structure

* [programs/blockmesh-program](programs/blockmesh-program/) - [Anchor](https://www.anchor-lang.com/)
  [Solana](https://solana.com/) program , manages the state of the BlockMesh network.
* [libs/block-mesh-manager](libs/block-mesh-manager/) - The BlockMesh network server.
* [libs/block-mesh-solana-client](libs/block-mesh-solana-client/) - [RPC](https://solana.com/docs/rpc)
  client and `Instruction` to interact with the BlockMesh program.
* [libs/client-node](libs/client-node/) - The BlockMesh network client users run,
  it creates a local `proxy` that manages the interaction with `solana` and forward requests to BlockMesh network.
* [libs/cloudflare-worker-ip-data](libs/cloudflare-worker-ip-data/) - A cloudflare worker that pings `IP` data providers
  and responds with the data back to the caller.
* [lib/cloudflare-worker-solana-relay](libs/cloudflare-worker-solana-relay/) - A cloudflare worker that relays requests
  to `solana`.
* [libs/ipapi-is-rust](libs/ipapi-is-rust/) - A rust library for [ipapi.is](https://ipapi.is/) types.
* [libs/landing-site](libs/landing-site/) - The BlockMesh network landing site.
* [libs/provider-node](libs/provider-node/) - The `provider-node` client,
  manages interactions with `solana` and incoming requests from `client-node`.
* [libs/solana-tiny-client](libs/solana-tiny-client/) - A tiny `solana` client with minimal dependencies
  hence making it support [WASM](https://webassembly.org/).
* [example-keys](example-keys/) - `solana` keys for `testing`/`demo`.

## Links

* [BlockMesh GitBoook](https://gitbook.blockmesh.xyz/)
* [BlockMesh Twitter](https://twitter.com/blockmesh_xyz)