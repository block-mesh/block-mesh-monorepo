#!/usr/bin/env bash
set -x
set -eo pipefail

heroku container:login
docker pull blockmesh/block-mesh-manager:latest
docker tag blockmesh/block-mesh-manager:latest registry.heroku.com/block-mesh-manager/web
docker push registry.heroku.com/block-mesh-manager/web
heroku container:push web -a block-mesh-manager
heroku container:release web -a block-mesh-manager
heroku restart -a block-mesh-manager