#!/usr/bin/env bash
set -x
export _PWD="$(pwd)"
export ROOT="$(git rev-parse --show-toplevel)"

cd "${ROOT}" && cargo b -p blockmesh-cli
cd "${ROOT}/libs/blockmesh-cli/dlopen" && gcc main.c -o test-lib
cd "${ROOT}/libs/blockmesh-cli/dlopen" && ./test-lib "${ROOT}/target/debug/libblockmesh_cli.dylib" $1 $2 $3

