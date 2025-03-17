#!/usr/bin/env bash
# https://github.com/rimutaka/spotify-playlist-builder/blob/master/build.sh
set -x
set -e
export BUILD_TYPE=$1
if [ -z "${BUILD_TYPE}" ] ; then
  export BUILD_TYPE="--dev"
  export BLOCKMESH_LOG_ENV="dev"
elif [ "${BUILD_TYPE}" == "--release" ]; then
  export BUILD_TYPE="--release"
  export BLOCKMESH_LOG_ENV="prod"
elif [ "${BUILD_TYPE}" == "--clean" ]; then
  export BUILD_TYPE="clean"
else
  echo "Invalid argument: ${BUILD_TYPE}"
  exit 1
fi

export _PWD="$(pwd)"
export ROOT="$(git rev-parse --show-toplevel)"
export VERSION=$(grep -m 1 '^version' Cargo.toml | sed -e 's/^version\s*=\s*//' | sed -e 's/"//g')
source "${ROOT}/scripts/setup.sh"
if [ -f "${ROOT}/.env" ] ; then
  source "${ROOT}/.env"
fi
cd "${ROOT}/libs/extension" || exit 1

npm install

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
rm -fr extension_js/js/*.wasm
rm -fr dist/*
rm -f chrome.zip
rm -f firefox.zip
rm -fr "${ROOT}"/tmp_ext/chrome/*
rm -fr "${ROOT}"/tmp_ext/firefox/*

if [ "${BUILD_TYPE}" = "clean" ] ; then
  echo "Finished cleaning"
  exit 0
fi

sed -i -e "s/\"version\":.*/\"version\": \"${VERSION}\",/" extension_js/manifests/manifest_cr.json || exit 1
sed -i -e "s/\"version\":.*/\"version\": \"${VERSION}\",/" extension_js/manifests/manifest_ff.json || exit 1

echo Building wasm module...
export RUSTFLAGS=--cfg=web_sys_unstable_apis
wasm-pack build . ${BUILD_TYPE} --no-typescript --out-dir "./extension_js/js/wasm" --out-name "blockmesh_ext" --target web || exit 1
npm run build
## wasm-pack creates bunch of useless files:
echo Removing trash files...
rm -f extension_js/js/wasm/.gitignore
rm -f extension_js/js/wasm/package.json

## create chrome package and exclude manifest for firefox
## see ReadMe for more info on manifest config
## subshell call with cd is required to avoid placing /extension/ folder as the root
rm -f chrome.zip && \
(cd extension_js && cp manifests/manifest_cr.json manifest.json)&& \
(cd extension_js && zip -rq ../chrome.zip . -x "*.ts" -x "*LICENSE.txt") && \
(cd extension_js && rm -f manifest.json) && \
(cp -f chrome.zip "${ROOT}/tmp_ext/chrome/") && \
(cd "${ROOT}/tmp_ext/chrome/" && unzip -o chrome.zip) && \
echo Chrome package: chrome.zip || exit 1

## create firefox package, exclude chrome manifest and rename FF manifest to its default file name
rm -f firefox.zip && \
(cd extension_js && cp manifests/manifest_ff.json manifest.json)&& \
(cd extension_js && zip -rq ../firefox.zip . -x "*.ts" -x "*LICENSE.txt") && \
(cd extension_js && rm -f manifest.json) && \
(cp -f firefox.zip "${ROOT}/tmp_ext/firefox/") && \
(cd "${ROOT}/tmp_ext/firefox/" && unzip -o firefox.zip) && \
echo Firefox package: firefox.zip || exit 1

