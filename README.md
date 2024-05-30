# BlockMesh

<p align="center" width="100%">
    <img width="33%" src="https://github.com/block-mesh/.github/assets/20769037/2eea7195-1d85-469e-b8ec-d0a0f90febeb"> 
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
