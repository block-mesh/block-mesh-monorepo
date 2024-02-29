#!/usr/bin/env bash
set -x
_PWD="$(pwd)"
ROOT="$(git rev-parse --show-toplevel)"
cd "${ROOT}/libs/block-mesh-manager" || exit
set +x
cargo install cargo-nextest --locked
#export TEST_LOG=1
#export RUST_BACKTRACE=1
export DATABASE_URL="postgres://postgres:password@localhost:5555/block-mesh"
docker rm "$(docker stop "$(docker ps -a -q --filter ancestor=postgres:15.3-alpine3.18 --format="{{.ID}}")")"
docker volume prune --force
"${ROOT}/scripts/init_db.sh"
if [ "$?" -ne 0 ]
then
    printf "\nbuild.sh failed\n"
    exit 1
fi

#cargo nextest run create_game --features my-test
cargo test -- --nocapture | bunyan
if [ "$?" -ne 0 ]
then
    printf "\cargo test failed\n"
    cd "${_PWD}" || exit
    exit 1
fi