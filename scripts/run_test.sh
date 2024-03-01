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
export DATABASE_URL="postgres://postgres:password@localhost:5555/block-mesh"
ensure docker rm "$(docker stop "$(docker ps -a -q --filter ancestor=postgres:15.3-alpine3.18 --format="{{.ID}}")")"
ensure docker volume prune --force
ensure "${ROOT}/scripts/init_db.sh"
#cargo nextest run create_game --features my-test
ensure cargo test -- --nocapture | bunyan