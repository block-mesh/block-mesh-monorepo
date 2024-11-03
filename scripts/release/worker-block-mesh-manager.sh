#!/usr/bin/env bash
set -x
set -eo pipefail

heroku container:login
#heroku container:push web -a block-mesh-manager
docker pull --platform linux/amd64 blockmesh/block-mesh-manager-worker:latest
docker tag blockmesh/block-mesh-manager-worker:latest registry.heroku.com/block-mesh-manager-worker/web
docker push registry.heroku.com/block-mesh-manager-worker/web
heroku container:release web -a block-mesh-manager-worker
heroku restart -a block-mesh-manager-worker