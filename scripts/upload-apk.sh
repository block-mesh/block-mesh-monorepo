#!/usr/bin/env bash
set -x
if [ "$1" == "" ] ; then
  echo "Please provide path to file"
  exit 1
fi
wrangler r2 object put blockmesh-network-apk/blockmesh-network.apk --file $1
