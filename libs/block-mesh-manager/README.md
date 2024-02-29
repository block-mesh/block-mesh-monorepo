# BlockMesh Manager

This is the naive implementation of the `BlockMesh` Manager.

## Usage

1. Provider Node registers with the Manager. A DB row is created for the provider node.
2. The manager will periodically poll for the provider node status.
3. Client will request a proxy from the manager.
4. Client will receive a `uuid` to send requests through.
5. The manager will map the `uuid` to the provider node and forward the request to the proxy.