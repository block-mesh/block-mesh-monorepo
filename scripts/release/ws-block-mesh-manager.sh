#!/usr/bin/env bash
set -x
set -eo pipefail

heroku container:login
docker pull  --platform linux/amd64 blockmesh/block-mesh-manager-ws:latest
docker tag blockmesh/block-mesh-manager-ws:latest registry.heroku.com/block-mesh-manager-ws/web
docker push registry.heroku.com/block-mesh-manager-ws/web
#heroku container:push web -a feature-flags-server
heroku container:release web -a block-mesh-manager-ws
heroku restart -a block-mesh-manager-ws