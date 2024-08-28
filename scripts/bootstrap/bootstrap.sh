#! /usr/bin/env bash
cargo install cargo-leptos --version=0.2.20
cargo install sqlx-cli --version=0.7.3
cargo install wasm-pack --version=0.12.1
cargo install bunyan
rustup target add wasm32-unknown-unknown

export ROOT="$(git rev-parse --show-toplevel)"

cp ${ROOT}/scripts/bootstrap/pre-commit ${ROOT}/.git/hooks
cp ${ROOT}/scripts/bootstrap/pre-push ${ROOT}/.git/hooks

chmod u+x ${ROOT}/.git/hooks/pre-commit
chmod u+x ${ROOT}/.git/hooks/pre-push
