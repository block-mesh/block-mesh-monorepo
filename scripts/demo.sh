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

cargo run -p provider-node -- --keypair-path example-keys/provider-node.json --proxy-port 4999 &> demo_logs/provider-node.log &
export PROVIDER_NODE_PID=$!
sleep 2

cargo run -p endpoint-node -- --port 4999 --keypair-path example-keys/client.json --provider-node-owner CERqu7FToQX6c1VGhDojaaFTcMX2H8vBPBbwmPnKfQdY &> demo_logs/endpoint-node.log &
export ENDPOINT_NODE_PID=$!
sleep 2

cargo run -p client-node -- --proxy-override 127.0.0.1:4000 --keypair-path example-keys/client.json --provider-node-owner CERqu7FToQX6c1VGhDojaaFTcMX2H8vBPBbwmPnKfQdY &> demo_logs/client-node.log &
export CLIENT_NODE_PID=$!

trap cleanup SIGINT EXIT
wait $CLIENT_NODE_PID
echo "Sleeping for 3"
sleep 3
cleanup