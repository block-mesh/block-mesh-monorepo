#!/usr/bin/env bash
set -x
export ROOT="$(git rev-parse --show-toplevel)"
git branch -D release
git branch -d release
set -eo pipefail
export ROOT="$(git rev-parse --show-toplevel)"
export CARGO_TARGET_DIR="${ROOT}/target/PRE-PUSH"
git checkout master
git pull
git checkout -b release
#git branch --set-upstream-to=origin/release release
git merge master
git rebase master -Xtheirs
export VERSION=$(grep -m 1 '^version' Cargo.toml | sed -e 's/^version\s*=\s*//' | sed -e 's/"//g')
export MINOR=$(echo $VERSION | cut -d '.' -f 3)
export NEWMINOR=$(expr $MINOR + 1)
export NEWVERSION=$(echo $VERSION | sed -e "s/$MINOR/$NEWMINOR/")
sed -i -e "s/$VERSION/$NEWVERSION/" Cargo.toml
git add Cargo.toml Cargo.lock
git commit -m "New release ${NEWVERSION}"
git push