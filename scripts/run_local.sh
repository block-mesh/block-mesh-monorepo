#!/usr/bin/env bash
set -x
_PWD="$(pwd)"
ROOT="$(git rev-parse --show-toplevel)"
cd "${ROOT}/libs/block-mesh-manager" || exit
set +x
export DATABASE_URL="postgres://postgres:password@localhost:5555/block-mesh"
"${ROOT}/scripts/init_db.sh"
if [ "$?" -ne 0 ]
then
    printf "\n init_db.sh failed\n"
    exit 1
fi
"${ROOT}/scripts/build.sh"
if [ "$?" -ne 0 ]
then
    printf "\n build.sh failed\n"
    exit 1
fi

#"${ROOT}/target/debug/backend" &
#export backend=$!

function cleanup()
{
  echo "Killing $backend and $frontend"
  kill $backend
  kill $frontend
}

trap cleanup SIGINT EXIT

wait $backend
wait $frontend