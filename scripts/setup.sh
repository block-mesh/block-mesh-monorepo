#!/usr/bin/env bash
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