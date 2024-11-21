#!/usr/bin/env bash

cleanup() {
    echo "Cleaning up..."
    # Kill all background jobs
    kill $(jobs -p) 2>/dev/null
    exit
}

trap cleanup EXIT INT

for i in {1..5000}; do
  websocat -Un "ws://localhost:8002/ws?email=123@blockmesh.xyz&api_token=321d6243-d111-444a-8bf6-329b573d0ade" &
#  websocat --insecure -Un "wss://localhost:3000" &
#  read -t 0.05
done

wait