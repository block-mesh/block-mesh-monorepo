#!/usr/bin/env bash
set -x
set -eo pipefail
export ROOT="$(git rev-parse --show-toplevel)"
cd "${ROOT}/libs/react-native-app/ios"
xcrun agvtool next-version -all