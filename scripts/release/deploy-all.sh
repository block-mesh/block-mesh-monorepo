#!/usr/bin/env bash
set -x
set -eo pipefail
export _PWD="$(pwd)"
export ROOT="$(git rev-parse --show-toplevel)"

"${ROOT}/scripts/release/feature-flags-server.sh"
"${ROOT}/scripts/release/api-block-mesh-manager.sh"
"${ROOT}/scripts/release/worker-block-mesh-manager.sh"
"${ROOT}/scripts/release/ws-block-mesh-manager.sh"
"${ROOT}/scripts/release/block-mesh-manager.sh"
"${ROOT}/scripts/release/tg-privacy-bot.sh"
"${ROOT}/scripts/release/data-sink.sh"
"${ROOT}/scripts/release/emails.sh"
"${ROOT}/scripts/release/ids.sh"
