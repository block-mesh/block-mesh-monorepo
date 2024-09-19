#!/usr/bin/env bash
set -x
set -eo pipefail

docker pull blockmesh/block-mesh-manager:latest-amd64
docker tag blockmesh/block-mesh-manager:latest-amd64 767398023645.dkr.ecr.us-east-1.amazonaws.com/block-mesh-manager
aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin 767398023645.dkr.ecr.us-east-1.amazonaws.com
docker push 767398023645.dkr.ecr.us-east-1.amazonaws.com/block-mesh-manager:latest

#docker tag blockmesh/block-mesh-manager:latest-amd64 registry.heroku.com/block-mesh-manager/web
#docker push registry.heroku.com/block-mesh-manager/web
#heroku container:push web -a block-mesh-manager
#docker pull blockmesh/block-mesh-manager-worker:latest-amd64
#docker tag blockmesh/block-mesh-manager-worker:latest-amd64 registry.heroku.com/block-mesh-manager/worker
#docker push registry.heroku.com/block-mesh-manager/worker
#heroku container:release web worker -a block-mesh-manager
#heroku restart -a block-mesh-manager