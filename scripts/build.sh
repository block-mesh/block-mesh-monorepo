#!/usr/bin/env bash
set -x
export _PWD="$(pwd)"
export ROOT="$(git rev-parse --show-toplevel)"
source "${ROOT}/scripts/setup.sh"
ensure cd "${ROOT}" && typeshare . --lang=typescript --output-file="${ROOT}/libs/react-native-app/utils/apiTypes.ts"
cd "${ROOT}/libs/block-mesh-manager" || exit
set +x
if [ -n "${DATABASE_URL+1}" ]; then
  export DATABASE_URL="postgres://postgres:password@localhost:5559/block-mesh"
fi
if [ -f "${ROOT}/.env" ] ; then
  source "${ROOT}/.env"
fi
ensure "${ROOT}/scripts/init_db.sh"
#ensure cargo build
export LEPTOS_HASH_FILES=false
ensure cargo leptos build --project block-mesh-manager
# ensure typeshare . --lang=typescript --output-file="${ROOT}/client/brain-war-client/src/helpers/apiTypes.ts"
