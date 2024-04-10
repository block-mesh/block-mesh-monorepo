# Provider Node

This is the executable for `provider nodes`. They run it on their local machine.

The `provider node` registers on `solana` and waits for incoming requests from clients.

```shell
cargo run -p proxy-master -- --help
```

```shell
Usage: proxy-master [OPTIONS] --keypair-path <KEYPAIR_PATH>

Options:
      --keypair-path <KEYPAIR_PATH>  
      --program-id <PROGRAM_ID>      [default: FRkQxATWhWqkj3SPZmbBCtkVM4fChd6VYLbEGhgCuHHJ]
      --port <PORT>                  [default: 3000]
  -h, --help                         Print help
```