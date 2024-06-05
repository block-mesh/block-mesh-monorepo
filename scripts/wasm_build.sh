#!/usr/bin/env bash
# https://github.com/rimutaka/spotify-playlist-builder/blob/master/build.sh
set -x
BUILD_TYPE=$1
if [ -z "${BUILD_TYPE}" ] ; then
  BUILD_TYPE="--dev"
elif [ "${BUILD_TYPE}" != "--release" ]; then
  echo "Invalid argument: ${BUILD_TYPE}"
  exit 1
fi

export _PWD="$(pwd)"
export ROOT="$(git rev-parse --show-toplevel)"
export VERSION=$(grep -m 1 '^version' Cargo.toml | sed -e 's/^version\s*=\s*//' | sed -e 's/"//g')
source "${ROOT}/scripts/setup.sh"
cd "${ROOT}/libs/extension" || exit 1

## --release or --dev - exclude/include debug info
## --no-typescript - disable .d.ts files output
## --out-dir - where to write the compiled files
## --out-name - force output file names
## --target - always use "web"!
## See https://rustwasm.github.io/wasm-pack/book/commands/build.html

echo Cleaning up...
mkdir -p "${ROOT}"/tmp_ext/chrome/
mkdir -p "${ROOT}"/tmp_ext/firefox/
rm -fr extension_js/js/wasm/*
rm -f chrome.zip
rm -f firefox.zip
rm -fr "${ROOT}"/tmp_ext/chrome/*
rm -fr "${ROOT}"/tmp_ext/firefox/*

sed -i -e "s/\"version\":.*/\"version\": \"${VERSION}\",/" extension_js/manifests/manifest_cr.json || exit 1
sed -i -e "s/\"version\":.*/\"version\": \"${VERSION}\",/" extension_js/manifests/manifest_ff.json || exit 1

echo Building wasm module...
export RUSTFLAGS=--cfg=web_sys_unstable_apis
wasm-pack build . ${BUILD_TYPE} --no-typescript --out-dir "./extension_js/js/wasm" --out-name "blockmesh_ext" --target web || exit 1

## wasm-pack creates bunch of useless files:
echo Removing trash files...
rm -f extension_js/js/wasm/.gitignore
rm -f extension_js/js/wasm/package.json

## create chrome package and exclude manifest for firefox
## see ReadMe for more info on manifest config
## subshell call with cd is required to avoid placing /extension/ folder as the root
rm -f chrome.zip && \
(cd extension_js && cp manifests/manifest_cr.json manifest.json)&& \
(cd extension_js && zip -rq ../chrome.zip .) && \
(cd extension_js && rm -f manifest.json) && \
(cp -f chrome.zip "${ROOT}/tmp_ext/chrome/") && \
(cd "${ROOT}/tmp_ext/chrome/" && unzip chrome.zip) && \
echo Chrome package: chrome.zip || exit 1

## create firefox package, exclude chrome manifest and rename FF manifest to its default file name
rm -f firefox.zip && \
(cd extension_js && cp manifests/manifest_ff.json manifest.json)&& \
(cd extension_js && zip -rq ../firefox.zip .) && \
(cd extension_js && rm -f manifest.json) && \
(cp -f firefox.zip "${ROOT}/tmp_ext/firefox/") && \
(cd "${ROOT}/tmp_ext/firefox/" && unzip firefox.zip) && \
echo Firefox package: firefox.zip || exit 1

