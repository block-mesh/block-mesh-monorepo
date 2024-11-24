#!/usr/bin/env bash
set -x
export APP_ENVIRONMENT="local"
export _PWD="$(pwd)"
export ROOT="$(git rev-parse --show-toplevel)"
export CARGO_TARGET_DIR="${ROOT}/target/WEBSERVER"
source "${ROOT}/scripts/setup.sh"
set +x
export AGG_SIZE=1
source "${ROOT}/scripts/setup.sh"
export DATABASE_URL="postgres://postgres:password@localhost:5559/block-mesh"
export CHANNEL_DATABASE_URL="postgres://postgres:password@localhost:5559/block-mesh"
export FOLLOWER_DATABASE_URL="postgres://postgres:password@localhost:5559/block-mesh"
export INSTRUMENT_WRAPPER=2500
export REDIS_URL="redis://127.0.0.1:6379"
if [ -f "${ROOT}/.env" ] ; then
  source "${ROOT}/.env"
fi
#ensure "${ROOT}/scripts/build.sh"
#"${ROOT}/target/debug/block-mesh-manager" &
export LEPTOS_HASH_FILES=false
#export RUST_LOG=sqlx=trace
export AGGREGATE_AGG_SIZE=1
cd "${ROOT}/libs/block-mesh-manager-api" || exit 1
ensure sqlx database create
sqlx migrate run --source migrations
ensure cargo sqlx prepare
>&2 echo "Postgres has been migrated, ready to go!"
cd "${_PWD}"
#cargo run -p block-mesh-manager-api
cargo watch --watch libs --shell "cargo run -p block-mesh-manager-api"
export backend=$!
function cleanup()
{
  echo "Killing ${backend}"
  kill "${backend}"
}
trap cleanup SIGINT EXIT
wait "${backend}"
