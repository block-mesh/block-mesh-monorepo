#!/usr/bin/env bash
set -x
export APP_ENVIRONMENT="local"
export SENTRY_LAYER="true"
export SENTRY_SAMPLE_RATE="1.0"
export _PWD="$(pwd)"
export ROOT="$(git rev-parse --show-toplevel)"
source "${ROOT}/scripts/setup.sh"
set +x
export AGG_SIZE=1
source "${ROOT}/scripts/setup.sh"
export DATABASE_URL="postgres://postgres:password@localhost:5559/block-mesh"
export REDIS_URL="redis://127.0.0.1:6379"
if [ -f "${ROOT}/.env" ] ; then
  source "${ROOT}/.env"
fi
#ensure "${ROOT}/scripts/init_db.sh"
cargo run -p block-mesh-manager-worker | bunyan