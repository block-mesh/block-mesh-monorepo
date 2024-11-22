#!/usr/bin/env bash
set -x
set -eo pipefail
export ROOT="$(git rev-parse --show-toplevel)"
export CARGO_TARGET_DIR="${ROOT}/target/PRE-PUSH"
git checkout master
git pull
export VERSION=$(grep -m 1 '^version' Cargo.toml | sed -e 's/^version\s*=\s*//' | sed -e 's/"//g')
export MINOR=$(echo $VERSION | cut -d '.' -f 3)
export NEWMINOR=$(expr $MINOR + 1)
export NEWVERSION=$(echo $VERSION | sed -e "s/$MINOR/$NEWMINOR/")
git checkout -B "release-${NEWVERSION}"
sed -i -e "s/$VERSION/$NEWVERSION/" Cargo.toml

function tag() {
  export TOML=$(git status -s Cargo.toml | wc -l | sed -e 's/ //g')
  if [ "$TOML" != "1" ]; then
#    echo "Cargo.toml didnt change"
#    exit 1
    git status
    cargo fmt
  fi
  export LOCK=$(git status -s Cargo.lock | wc -l | sed -e 's/ //g')
  if [ "$LOCK" != "1" ]; then
#    echo "Cargo.lock didnt change"
#    exit 1
    git status
    cargo fmt
  fi
}

tag
tag
git add Cargo.toml Cargo.lock
git commit -m "New release ${NEWVERSION}"
git push