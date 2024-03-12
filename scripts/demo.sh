#!/usr/bin/env bash
set -x
export _PWD="$(pwd)"
export ROOT="$(git rev-parse --show-toplevel)"
source "${ROOT}/scripts/setup.sh"
ensure cargo build
mkdir -p demo_logs

function cleanup()
{
  echo "Killing $PROVIDER_NODE_PID and $CLIENT_NODE_PID"
  kill "${PROVIDER_NODE_PID}"
  kill "${CLIENT_NODE_PID}"
}

#curl \
#  --proxy-header 'api_token: 123' \
#  -x \
#  "127.0.0.1:3000" \
#  "https://example.com"

PROVIDER_NODE_KEYPAIR=example-keys/provider-node.json cargo run -p provider-node &> demo_logs/provider-node.log &
export PROVIDER_NODE_PID=$!
CLIENT_KEYPAIR=example-keys/client.json cargo run -p client-node &> demo_logs/client-node.log &
export CLIENT_NODE_PID=$!

trap cleanup SIGINT EXIT
wait $CLIENT_NODE_PID
echo "Sleeping for 3"
sleep 3
cleanup