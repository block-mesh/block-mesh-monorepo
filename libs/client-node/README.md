# Client Node

This is a `CLI` for `clients` to interact with `provider nodes`.

It will manage authentication and payment through `solana`.

```shell
cargo run -p client-node -- --help
```

```shell
Usage: client-node [OPTIONS] --keypair-path <KEYPAIR_PATH> --proxy-master-owner <PROVIDER_NODE_OWNER> [RESPONSE_TYPE]

Arguments:
  [RESPONSE_TYPE]  [default: json] [possible values: json, text]

Options:
      --keypair-path <KEYPAIR_PATH>
          
      --proxy-master-owner <PROVIDER_NODE_OWNER>
          
      --program-id <PROGRAM_ID>
          [default: FRkQxATWhWqkj3SPZmbBCtkVM4fChd6VYLbEGhgCuHHJ]
      --target <TARGET>
          [default: https://api.ipify.org?format=json]
      --proxy-override <PROXY_OVERRIDE>
          
  -h, --help
          Print help
```