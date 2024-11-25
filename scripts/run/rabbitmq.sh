#!/usr/bin/env bash
set -x
export APP_ENVIRONMENT="local"
export _PWD="$(pwd)"
export ROOT="$(git rev-parse --show-toplevel)"
export CARGO_TARGET_DIR="${ROOT}/target/WEBSERVER"
source "${ROOT}/scripts/setup.sh"
set +x