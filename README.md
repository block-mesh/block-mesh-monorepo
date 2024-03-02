# BlockMesh

## V1 - Naive Implementation

1. Provider nodes register themselves to the network.
2. Clients request a proxy from the network.
3. Clients send traffic through the `block-mesh`,
   which forwards it to the provider node, to enforce proper limits/payments/etc.