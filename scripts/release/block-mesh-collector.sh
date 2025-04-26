#!/usr/bin/env bash
set -x
set -eo pipefail

heroku container:login
docker pull --platform linux/amd64 blockmesh/block-mesh-collector:latest
docker tag blockmesh/block-mesh-collector:latest registry.heroku.com/block-mesh-collector/web
docker push registry.heroku.com/block-mesh-collector/web
#heroku container:push web -a feature-flags-server
heroku container:release web -a block-mesh-collector
heroku restart -a block-mesh-collector