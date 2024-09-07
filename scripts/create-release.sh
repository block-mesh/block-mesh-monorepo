#!/usr/bin/env bash
set -x
set -eo pipefail
export ROOT="$(git rev-parse --show-toplevel)"
export CARGO_TARGET_DIR="${ROOT}/target/PRE-PUSH"
git checkout master
git pull
#git branch --set-upstream-to=origin/release release
export VERSION=$(grep -m 1 '^version' Cargo.toml | sed -e 's/^version\s*=\s*//' | sed -e 's/"//g')
export MINOR=$(echo $VERSION | cut -d '.' -f 3)
export NEWMINOR=$(expr $MINOR + 1)
export NEWVERSION=$(echo $VERSION | sed -e "s/$MINOR/$NEWMINOR/")
sed -i -e "s/$VERSION/$NEWVERSION/" Cargo.toml
git checkout -b "release-${NEWVERSION}"
cargo clippy --all --features ssr,hydrate -- -D warnings
#git branch --set-upstream-to=origin/release release
#git pull
git add Cargo.toml Cargo.lock
git commit -m "New release ${NEWVERSION}"
git push