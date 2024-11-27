#!/usr/bin/env bash
set -x
export APP_ENVIRONMENT="local"
export SENTRY_LAYER="true"
export SENTRY_SAMPLE_RATE="1.0"
export _PWD="$(pwd)"
export ROOT="$(git rev-parse --show-toplevel)"
export ADMIN_PARAM="test"
source "${ROOT}/scripts/setup.sh"
set +x
source "${ROOT}/scripts/setup.sh"
if [ -f "${ROOT}/.env" ] ; then
  source "${ROOT}/.env"
fi
#export POSTGRES_DB="data-sink"
#ensure "${ROOT}/scripts/init_db.sh"
export DATABASE_URL="postgres://postgres:password@localhost:5553/emails"
#cargo watch --watch libs --shell "cargo run -p tg-privacy-bot | bunyan &"
cargo watch --watch libs --shell "cargo run -p emails"
export backend=$!
function cleanup()
{
  echo "Killing ${backend}"
  kill "${backend}"
}
trap cleanup SIGINT EXIT
wait "${backend}"
