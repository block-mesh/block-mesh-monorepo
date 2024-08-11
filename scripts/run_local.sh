#!/usr/bin/env bash
set -x
export APP_ENVIRONMENT="local"
export _PWD="$(pwd)"
export ROOT="$(git rev-parse --show-toplevel)"
export CARGO_TARGET_DIR="${ROOT}/target/WEBSERVER"
source "${ROOT}/scripts/setup.sh"
#cd "${ROOT}/libs/block-mesh-manager" || exit 1
set +x
source "${ROOT}/scripts/setup.sh"
export DATABASE_URL="postgres://postgres:password@localhost:5559/block-mesh"
export REDIS_URL="redis://127.0.0.1:6379"
if [ -f "${ROOT}/.env" ] ; then
  source "${ROOT}/.env"
fi
ensure "${ROOT}/scripts/init_db.sh"
#ensure "${ROOT}/scripts/build.sh"
#"${ROOT}/target/debug/block-mesh-manager" &
export LEPTOS_HASH_FILES=false
cargo leptos watch --project block-mesh-manager | bunyan &
export backend=$!
function cleanup()
{
  echo "Killing ${backend}"
  kill "${backend}"
}
trap cleanup SIGINT EXIT
wait "${backend}"
