#!/usr/bin/env bash
set -x
set -eo pipefail
export _PWD="$(pwd)"
export ROOT="$(git rev-parse --show-toplevel)"
source "${ROOT}/scripts/setup.sh"

DOCKERS="$(docker ps -a -q --filter ancestor=clickhouse/clickhouse-server --format="{{.ID}}")"
if [ -n "$DOCKERS" ]
then
  ensure docker rm --force --volumes $DOCKERS
fi
docker run -d --name clickhouse-server  -p 8123:8123 -p 9000:9000 -p 9009:9009 --ulimit nofile=262144:262144 clickhouse/clickhouse-server
export CLICKHOUSE_URL="http://127.0.0.1:8123"