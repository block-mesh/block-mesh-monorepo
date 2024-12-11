#!/usr/bin/env bash
set -x
pkill -9 clickhouse
set -eo pipefail
export _PWD="$(pwd)"
export ROOT="$(git rev-parse --show-toplevel)"
source "${ROOT}/scripts/setup.sh"
clickhouse server &
export CLICKHOUSE_URL="http://127.0.0.1:8123"
