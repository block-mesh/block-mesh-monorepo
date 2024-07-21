#!/usr/bin/env bash
pkill yarn
pkill trunk
yarn dev &
export YARN=$!
trunk serve &
export TRUNK=$!


function cleanup()
{
  echo "Killing ${YARN} ${TRUNK}"
  kill "${YARN}"
  kill "${TRUNK}"
}
trap cleanup SIGINT EXIT
wait "${TRUNK}"
wait "${YARN}"
