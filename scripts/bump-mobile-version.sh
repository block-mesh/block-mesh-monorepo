#!/usr/bin/env bash
set -x
export ROOT="$(git rev-parse --show-toplevel)"

cd "${ROOT}/libs/react-native-app/ios"
xcrun agvtool next-version -all

#export VERSION=$( grep -m 1 'CURRENT_PROJECT_VERSION' "${ROOT}/libs/react-native-app/ios/reactnativeapp.xcodeproj/project.pbxproj" | sed -e 's/.*CURRENT_PROJECT_VERSION = //' | sed -e 's/;//')
#export NEWVERSION=$(expr $VERSION + 1)
#sed -e "s/CURRENT_PROJECT_VERSION.*/CURRENT_PROJECT_VERSION = ${NEWVERSION} ;/" -i "${ROOT}/libs/react-native-app/ios/reactnativeapp.xcodeproj/project.pbxproj"