#!/usr/bin/env bash
set -x
export APP_ENVIRONMENT="local"
export _PWD="$(pwd)"
export ROOT="$(git rev-parse --show-toplevel)"
source "${ROOT}/scripts/setup.sh"
cd "${ROOT}/libs/block-mesh-manager" || exit 1
set +x
source "${ROOT}/scripts/setup.sh"
export DATABASE_URL="postgres://postgres:password@localhost:5559/block-mesh"
if [ -f "${ROOT}/.env" ] ; then
  source "${ROOT}/.env"
fi
ensure "${ROOT}/scripts/init_db.sh"
ensure "${ROOT}/scripts/build.sh"
cargo watch -x run -w templates -w src
