#!/usr/bin/env bash
set -eo pipefail
export _PWD="$(pwd)"
export ROOT="$(git rev-parse --show-toplevel)"
export RPC="https://api.devnet.solana.com"
export PROGRAM_ID="CfaL9sdaEK49r4WLAtVh2vVgAZuv2eKbb6jSB5jDCMSF"
source "${ROOT}/scripts/setup.sh"

ensure anchor build
ensure anchor deploy --provider.cluster "${RPC}"
ensure anchor idl upgrade --provider.cluster "${RPC}" "${PROGRAM_ID}" --filepath target/idl/blockmesh_program.json