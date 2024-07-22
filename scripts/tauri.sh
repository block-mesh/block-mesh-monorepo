#!/usr/bin/env bash
export APP_ENVIRONMENT="local"
export _PWD="$(pwd)"
export ROOT="$(git rev-parse --show-toplevel)"
source "${ROOT}/scripts/setup.sh"
export CARGO_TARGET_DIR="${ROOT}/target/TAURI"
#cd "${ROOT}/libs/block-mesh-manager" || exit 1
set +x
source "${ROOT}/scripts/setup.sh"
export DATABASE_URL="postgres://postgres:password@localhost:5555/block-mesh"
if [ -f "${ROOT}/.env" ] ; then
  source "${ROOT}/.env"
fi
export LEPTOS_HASH_FILES=false
cargo tauri dev