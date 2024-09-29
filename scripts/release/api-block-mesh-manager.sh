#!/usr/bin/env bash
set -x
set -eo pipefail

heroku container:login
docker pull blockmesh/block-mesh-manager-api:latest-amd64
docker tag blockmesh/block-mesh-manager-api:latest-amd64 registry.heroku.com/block-mesh-manager-api/web
docker push registry.heroku.com/block-mesh-manager-api/web
#heroku container:push web -a feature-flags-server
heroku container:release web -a block-mesh-manager-api
heroku restart -a block-mesh-manager-api