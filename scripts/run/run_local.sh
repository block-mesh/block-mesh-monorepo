#!/usr/bin/env bash
set -x
# tokio-console:
#export RUSTFLAGS="--cfg tokio_unstable"
export APP_ENVIRONMENT="local"
export SENTRY_LAYER="true"
export SENTRY_SAMPLE_RATE="1.0"
export _PWD="$(pwd)"
export ROOT="$(git rev-parse --show-toplevel)"
export CARGO_TARGET_DIR="${ROOT}/target/WEBSERVER"
source "${ROOT}/scripts/setup.sh"
#cd "${ROOT}/libs/block-mesh-manager" || exit 1
set +x
export AGG_SIZE=1
source "${ROOT}/scripts/setup.sh"
export DATABASE_URL="postgres://postgres:password@localhost:5559/block-mesh"
export REDIS_URL="redis://127.0.0.1:6379"
export MAILGUN_SEND_KEY=""
export TWITTER_CLIENT_ID=""
export TWITTER_CLIENT_SECRET=""
export TWITTER_CALLBACK_URL="http://localhost:3000"
export BLOCKMESH_SERVER_UUID="ff28257b-4ac8-47c2-b26f-d567626a411e"
if [ -f "${ROOT}/.env" ] ; then
  source "${ROOT}/.env"
fi
ensure "${ROOT}/scripts/init_db.sh"
#ensure "${ROOT}/scripts/build.sh"
#"${ROOT}/target/debug/block-mesh-manager" &
export LEPTOS_HASH_FILES=false
#export RUST_LOG=sqlx=trace
export AGGREGATE_AGG_SIZE=1
cargo leptos watch --project block-mesh-manager | bunyan &
export backend=$!
function cleanup()
{
  echo "Killing ${backend}"
  kill "${backend}"
}
trap cleanup SIGINT EXIT
wait "${backend}"
