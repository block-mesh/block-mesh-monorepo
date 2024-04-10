# Colosseum Renaissance Hackathon Demo

To fully unlock the demo, you'll need 2 different remote hosts.
But it's not mandatory, you see the basics working on a single host.

## Clone and build the repository

* `git clone https://github.com/block-mesh/block-mesh-monorepo.git`
* `cargo build`

## Launch the Proxy Master node

This will try to bind by default to port `0.0.0.0:5000` and `0.0.0.0:4000`.
Run `cargo run -p blockmesh-bin -- proxy-master --help` to see the options.

`cargo run -p blockmesh-bin -- proxy-master --keypair-path example-keys/proxy-master.json`

## Launch the Proxy-Endpoint Node

This will run the `proxy-endpoint` and try to connect to the first `proxy-master` node it finds published on-chain.
(Obviously this is simplified for demo purposes)

Can override the `IP` or `owner` of the `proxy-master` via the `CLI` , see `--help` for more info.

`cargo run -p blockmesh-bin -- proxy-endpoint --keypair-path example-keys/client.json`

If you want to test with a local `proxy-master` you can use the `--proxy-override` option to point to it.
For example `--proxy-override 127.0.0.1:5000`, which points to the default `proxy-master` `proxy_port`.

## Run the client and perform a request through the network

This client is similar to a basic `curl` but handles the protocol details for you.
Later we'll extend it to be a local proxy, you can point your browser or other applications to go through,
enabling a smooth and seamless interaction with `BlockMesh`.

`cargo run -p blockmesh-bin -- client-node --keypair-path example-keys/client.json`

If you want to test with a local `proxy-master` you can use the `--proxy-override` option to point to it.
For example `--proxy-override 127.0.0.1:4000`, which points to the default `proxy-master` `client_port`.

The default `--target` will just return your `IP` details, which should reflect the `proxy-endpoint` since it actually
terminates the request.
You can change it via the `--target` option.

The `client-node` also supports `proxy` mode, which will launch it as a local `proxy` server.
You can run `curl -x` pointing to it, or point to it in your browser settings.

## Caveats

* The `proxy-master` `--proxy-port` and `--client-port` should be publicly available and open in your firewall, unless
  you're running all locally.
* All the nodes have overrides options for easier testing and demo purposes.
* Currently, the `proxy-master` publish their `IP:PORT` on-chain and the `client-node` creates an on-chain token to
  connect to the `proxy-master`.
  The protocol isn't fully implemented
  yet, [discussion](https://github.com/block-mesh/block-mesh-monorepo/discussions/64) on details and future plan is
  tracked here.
* Currently `BlockMesh` is deployed only to `devnet` since it's still under heavy development.
