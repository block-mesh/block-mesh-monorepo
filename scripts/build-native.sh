#!/usr/bin/env bash

# https://www.reactnativepro.com/tutorials/integrating-rust-in-a-react-native-project/

set -x
export _PWD="$(pwd)"
export ROOT="$(git rev-parse --show-toplevel)"
source "${ROOT}/scripts/setup.sh"
export RUST_MODULE_DIR="${ROOT}/libs/blockmesh-cli"
export REACT_NATIVE_ANDROID_DIR="${ROOT}/libs/react-native-app/modules/my-rust-module/android/src/main/"
export LIB_NAME="libblockmesh_cli"
cd "${ROOT}/libs/react-native-app" || exit 1
mkdir -p headers

rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
cargo install cargo-ndk

export BUILD_TYPE=$1
export BUILD_IOS="false"
export BUILD_ANDROID="false"

if [ "${BUILD_TYPE}" == "--ios" ] ; then
  export BUILD_IOS="true"
elif [ "${BUILD_TYPE}" == "--android" ]; then
  export BUILD_ANDROID="true"
elif [ "${BUILD_TYPE}" == "--both" ]; then
  export BUILD_ANDROID="true"
  export BUILD_IOS="true"
elif [ "${BUILD_TYPE}" == "" ]; then
  echo "Missing build flag: --ios --android or --both"
  exit 1
else
  echo "Invalid build flag: ${BUILD_TYPE} use --ios --android or --both"
  exit 1
fi


if [ "${BUILD_IOS}" == "true" ]; then
  ensure rm -f "${ROOT}/target/aarch64-apple-ios/release/${LIB_NAME}.a"
  ensure rm -fr "${ROOT}/libs/react-native-app/headers/blockmesh-cli.h"

  ensure cargo build -p blockmesh-cli --release --target aarch64-apple-ios
  ensure cargo build -p blockmesh-cli --release --target aarch64-apple-ios-sim

  ensure cd "${ROOT}/libs/blockmesh-cli" \
  && cbindgen --only-target-dependencies --lang c --crate blockmesh-cli --output "${ROOT}/libs/react-native-app/headers/blockmesh-cli.h" \
  && cd "${_PWD}" || exit 1


  ensure cp "${ROOT}/target/aarch64-apple-ios/release/${LIB_NAME}.a" "${ROOT}/libs/react-native-app/modules/my-rust-module/ios/rust"
  ensure cp "${ROOT}/libs/react-native-app/headers/blockmesh-cli.h" "${ROOT}/libs/react-native-app/modules/my-rust-module/ios/rust"
  ensure mkdir -p "${ROOT}/libs/react-native-app/blockmesh-cli.xcframework"
  ensure rm -fr "${ROOT}/libs/react-native-app/blockmesh-cli.xcframework/ios-arm64"
  ensure rm -fr "${ROOT}/libs/react-native-app/blockmesh-cli.xcframework/ios-arm64-simulator"

  ensure cd "${ROOT}/libs/react-native-app" \
  && ensure xcodebuild -create-xcframework \
  -library "${ROOT}/target/aarch64-apple-ios/release/${LIB_NAME}.a" \
  -headers ./headers \
  -library "${ROOT}/target/aarch64-apple-ios-sim/release/${LIB_NAME}.a" \
  -headers ./headers \
  -output blockmesh-cli.xcframework \
  && cd "${_PWD}" || exit 1
fi

if [ "${BUILD_ANDROID}" == "true" ] ; then
  ensure cd "${ROOT}/libs/blockmesh-cli" && cargo ndk --target aarch64-linux-android    --platform 31 -- build --release && cd "${_PWD}" || exit 1
  ensure cd "${ROOT}/libs/blockmesh-cli" && cargo ndk --target armv7-linux-androideabi  --platform 31 -- build --release && cd "${_PWD}" || exit 1
  ensure cd "${ROOT}/libs/blockmesh-cli" && cargo ndk --target i686-linux-android       --platform 31 -- build --release && cd "${_PWD}" || exit 1
  ensure cd "${ROOT}/libs/blockmesh-cli" && cargo ndk --target x86_64-linux-android     --platform 31 -- build --release && cd "${_PWD}" || exit 1
  mkdir -p "${REACT_NATIVE_ANDROID_DIR}/jniLibs/arm64-v8a"
  mkdir -p "${REACT_NATIVE_ANDROID_DIR}/jniLibs/armeabi-v7a"
  mkdir -p "${REACT_NATIVE_ANDROID_DIR}/jniLibs/x86"
  mkdir -p "${REACT_NATIVE_ANDROID_DIR}/jniLibs/x86_64"
  cp "${ROOT}/target/aarch64-linux-android/release/${LIB_NAME}.so" "$REACT_NATIVE_ANDROID_DIR/jniLibs/arm64-v8a/"
  cp "${ROOT}/target/armv7-linux-androideabi/release/${LIB_NAME}.so" "$REACT_NATIVE_ANDROID_DIR/jniLibs/armeabi-v7a/"
  cp "${ROOT}/target/i686-linux-android/release/${LIB_NAME}.so" "$REACT_NATIVE_ANDROID_DIR/jniLibs/x86/"
  cp "${ROOT}/target/x86_64-linux-android/release/${LIB_NAME}.so" "$REACT_NATIVE_ANDROID_DIR/jniLibs/x86_64/"
fi
