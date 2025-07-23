#!/usr/bin/env bash
set -x
export _PWD="$(pwd)"
export ROOT="$(git rev-parse --show-toplevel)"
source "${ROOT}/scripts/setup.sh"
cd "${ROOT}" || exit 1

export RELEASE="${ROOT}/tmp_ext/chrome/chrome.zip"
if [ ! -f "${RELEASE}" ] ; then
  echo "${RELEASE} is missing"
  exit 1
fi

export VERSION=$(grep '"version"' tmp_ext/chrome/manifest.json | sed -e 's/\s*"version":\s*//' | sed -e 's/"\|,//g')
ensure npx wrangler r2 object put "extension-releases/pcn-${VERSION}.zip" --file "${RELEASE}" --remote
ensure npx wrangler r2 object put "extension-releases/pcn-latest.zip" --file "${RELEASE}" --remote
