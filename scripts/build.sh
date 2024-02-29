#!/usr/bin/env bash
set -x
_PWD="$(pwd)"
ROOT="$(git rev-parse --show-toplevel)"
cd "${ROOT}/libs/block-mesh-manager" || exit
set +x

if [ -n "${DATABASE_URL+1}" ]; then
  export DATABASE_URL="postgres://postgres:password@localhost:5555/block-mesh"
fi
"${ROOT}/scripts/init_db.sh"

if [ "$?" -ne 0 ]
then
    printf "\n init_db.sh failed\n"
    cd "${_PWD}" || exit
    exit 1
fi

cargo build

if [ "$?" -ne 0 ]
then
    printf "\n cargo build failed\n"
    cd "${_PWD}" || exit
    exit 1
fi
#typeshare . --lang=typescript --output-file="${ROOT}/client/brain-war-client/src/helpers/apiTypes.ts"
#if [ "$?" -ne 0 ]
#then
#    printf "\n typeshare failed\n"
#    cd "${_PWD}" || exit
#    exit 1
#fi

