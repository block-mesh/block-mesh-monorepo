#!/usr/bin/env bash
set -x
set -eo pipefail

heroku restart -a block-mesh-manager-api
heroku restart -a feature-flags-server
heroku restart -a block-mesh-manager-worker
heroku restart -a block-mesh-manager-ws
heroku restart -a block-mesh-manager