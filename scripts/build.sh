#!/usr/bin/env bash
set -x
export _PWD="$(pwd)"
export ROOT="$(git rev-parse --show-toplevel)"
source "${ROOT}/scripts/setup.sh"
cd "${ROOT}/libs/block-mesh-manager" || exit
set +x
if [ -n "${DATABASE_URL+1}" ]; then
  export DATABASE_URL="postgres://postgres:password@localhost:5555/block-mesh"
fi
ensure "${ROOT}/scripts/init_db.sh"
ensure cargo build
# ensure typeshare . --lang=typescript --output-file="${ROOT}/client/brain-war-client/src/helpers/apiTypes.ts"