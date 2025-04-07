#!/usr/bin/env bash
set -x
export APP_ENVIRONMENT="local"
export SENTRY_LAYER="true"
export SENTRY_SAMPLE_RATE="1.0"
export _PWD="$(pwd)"
export ROOT="$(git rev-parse --show-toplevel)"
export ENFORCE_KEYPAIR="true"
source "${ROOT}/scripts/setup.sh"
set +x
source "${ROOT}/scripts/setup.sh"
if [ -f "${ROOT}/.env" ] ; then
  source "${ROOT}/.env"
fi
export RAMA_TLS_CRT="${ROOT}/certs/cert.pem"
export RAMA_TLS_KEY="${ROOT}/certs/key.pem"
export DATABASE_URL="postgres://postgres:password@localhost:6999/ids"
export WRITE_DATABASE_URL="${DATABASE_URL}"
export PORT=8080
export AGG_SIZE=1
export ENFORCE_SIGNATURE=true
cargo watch --watch libs --shell "cargo run -p rama-cli -- --secure --port ${PORT}"
export backend=$!
function cleanup()
{
  echo "Killing ${backend}"
  kill "${backend}"
}
trap cleanup SIGINT EXIT
wait "${backend}"
