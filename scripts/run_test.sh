#!/usr/bin/env bash
set -x
export _PWD="$(pwd)"
export ROOT="$(git rev-parse --show-toplevel)"
source "${ROOT}/scripts/setup.sh"
cd "${ROOT}/libs/block-mesh-manager" || exit
set +x
cargo install cargo-nextest --locked
#export TEST_LOG=1
#export RUST_BACKTRACE=1
source "${ROOT}/scripts/setup.sh"
export DATABASE_URL="postgres://postgres:password@localhost:5559/block-mesh"
export REDIS_URL="redis://127.0.0.1:6379"
if [ -f "${ROOT}/.env" ] ; then
  source "${ROOT}/.env"
fi
#cargo nextest run create_game --features my-test
ensure cargo test -p block-mesh-manager -- --nocapture | bunyan
