#!/usr/bin/env bash
set -x
export _PWD="$(pwd)"
export ROOT="$(git rev-parse --show-toplevel)"
source "${ROOT}/scripts/setup.sh"

cd "${ROOT}" || exit 1

ensure ./scripts/mobile/bump-ios-build.sh
ensure ./scripts/mobile/build-native.sh --both

ensure cd "${ROOT}/libs/react-native-app" && npx eas build --platform ios --local
ensure cd "${ROOT}/libs/react-native-app" && npx eas build --platform android --local
ensure cd "${ROOT}/libs/react-native-app" && npx eas build --platform android --local --profile preview