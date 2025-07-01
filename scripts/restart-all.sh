#!/usr/bin/env bash
set -x
set -eo pipefail

heroku restart -a block-mesh-manager-api &
heroku restart -a feature-flags-server &
heroku restart -a block-mesh-manager-worker &
heroku restart -a block-mesh-manager-ws &
heroku restart -a emails &
heroku restart -a data-sink &
heroku restart -a block-mesh-manager &
heroku restart -a ids-blockmesh &
heroku restart -a blockmesh-pghero &
