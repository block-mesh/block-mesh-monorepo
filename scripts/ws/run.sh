#!/usr/bin/env bash

cleanup() {
    echo "Cleaning up..."
    # Kill all background jobs
    kill $(jobs -p) 2>/dev/null
    exit
}

trap cleanup EXIT INT

for i in {1..5000}; do
  websocat -Un "ws://localhost:8002/ws?email=123@blockmesh.xyz&api_token=5fdea056-1128-4659-a606-698acacc4cba" &
#  websocat --insecure -Un "wss://localhost:3000" &
#  read -t 0.05
done

wait