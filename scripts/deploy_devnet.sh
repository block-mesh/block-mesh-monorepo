#!/usr/bin/env bash
set -eo pipefail
export _PWD="$(pwd)"
export ROOT="$(git rev-parse --show-toplevel)"
export RPC="https://api.devnet.solana.com"
export PROGRAM_ID="FRkQxATWhWqkj3SPZmbBCtkVM4fChd6VYLbEGhgCuHHJ"
source "${ROOT}/scripts/setup.sh"

ensure anchor build
ensure anchor deploy --provider.cluster "${RPC}"
# doesnt use ensure since this is a one time op
anchor idl init --provider.cluster "${RPC}" "${PROGRAM_ID}" --filepath target/idl/blockmesh_program.json
ensure anchor idl upgrade --provider.cluster "${RPC}" "${PROGRAM_ID}" --filepath target/idl/blockmesh_program.json