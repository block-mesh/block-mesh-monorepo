#!/usr/bin/env bash

export TARGET="blockmesh-bin"
chmod 755 "${TARGET}"

ensure() {
    if ! "$@"; then
      if [ ! -z "${_PWD}" ] ;
      then
          cd "${_PWD}" || err "command failed: $*"
      fi
      err "command failed: $*"
    fi
}

err() {
    printf 'err: %s\n' "$1" >&2
    exit 1
}

export PLIST_CONTENT=""
IFS='' read -r -d '' PLIST_CONTENT <<"EOF"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>xyz.blockmesh</string>
    <key>Program</key>
    <string>/usr/local/bin/blockmesh-bin</string>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
</dict>
</plist>
EOF

ensure echo "$PLIST_CONTENT" > xyz.blockmesh.plist
ensure sudo cp xyz.blockmesh.plist /Library/LaunchDaemons/xyz.blockmesh.plist
ensure sudo launchctl load /Library/LaunchDaemons/xyz.blockmesh.plist
ensure sudo launchctl start xyz.blockmesh
