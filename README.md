# BlockMesh

<p align="center" width="100%">
    <img width="33%" src="https://imagedelivery.net/3RKw_J_fJQ_4KpJP3_YgXA/e4f3cdc0-c2ba-442d-3e48-e2f31c0dc100/public"> 
</p>

BlockMesh (http://blockmesh.notion.site/): Decentralized AI Monitoring 
BlockMesh challenges the centralized control of AI monitoring by promoting a transparent, community-driven approach. As AI becomes more embedded in our lives, the risks of centralized oversight and ethical misalignment grow. Our mission is to democratize AI oversight, making it accessible, accountable, and secure through blockchain and geo-diverse networks. BlockMesh empowers individuals globally to participate in ensuring that AI systems align with human values, fostering a balanced and open digital ecosystem. Join us in reshaping AI accountability.

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

* `cargo install cargo-leptos --version=0.2.20`
* `cargo install sqlx-cli --verison=0.7.3`
* `cargo install wasm-pack --version=0.12.1`
* `rustup target add wasm32-unknown-unknown`
* `cargo install bunyan`
* [Install psql](https://www.timescale.com/blog/how-to-install-psql-on-mac-ubuntu-debian-windows/)
* [Install Docker](https://docs.docker.com/engine/install/)

Run `./scripts/run_local.sh`

## Git Hooks

Add `.git/hooks/pre-commit`:

```shell
#!/bin/sh
set -e
export _PWD="$(pwd)"
export ROOT="$(git rev-parse --show-toplevel)"
source "${ROOT}/scripts/setup.sh"
export CARGO_TARGET_DIR="${ROOT}/target/PRE-COMMIT"
current_branch=$(git branch --show-current)
if [ $current_branch == "master" ] ; then
        echo "Cannot commit to master"
        exit 1
fi
echo '+cargo fmt --all -- --check'
cargo fmt --all -- --check
```

Add `.git/hooks/pre-push`:

```shell
#!/bin/sh
set -e
export _PWD="$(pwd)"
export ROOT="$(git rev-parse --show-toplevel)"
source "${ROOT}/scripts/setup.sh"
export CARGO_TARGET_DIR="${ROOT}/target/PRE-PUSH"
current_branch=$(git branch --show-current)
if [ $current_branch == "master" ] ; then
        echo "Cannot commit to master"
        exit 1
fi

echo '+cargo test --all'
cargo test --all
echo '+cargo clippy --all -- -D warnings'
cargo clippy --all -- -D warnings
echo '+cargo fmt --all -- --check'
cargo fmt --all -- --check
```

