#!/usr/bin/env bash
set -x
export APP_ENVIRONMENT="local"
export _PWD="$(pwd)"
export ROOT="$(git rev-parse --show-toplevel)"
source "${ROOT}/scripts/setup.sh"
cd "${ROOT}/libs/block-mesh-manager" || exit 1
set +x
source "${ROOT}/scripts/setup.sh"
export DATABASE_URL="postgres://postgres:password@localhost:5555/block-mesh"
ensure "${ROOT}/scripts/init_db.sh"
ensure "${ROOT}/scripts/build.sh"
if [ -f .env ] ; then
  source .env
fi
#"${ROOT}/target/debug/block-mesh-manager" &
cargo leptos watch --project block-mesh-manager | bunyan &
export backend=$!
function cleanup()
{
  echo "Killing ${backend}"
  kill "${backend}"
}
trap cleanup SIGINT EXIT
wait "${backend}"