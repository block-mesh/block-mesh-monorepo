#!/usr/bin/env bash
set -x
set -eo pipefail

docker build --platform linux/amd64 -t blockmesh-pghero -f blockmesh-pghero.Dockerfile --load .
docker image tag blockmesh-pghero:latest blockmesh/blockmesh-pghero:latest
docker image push blockmesh/blockmesh-pghero:latest

heroku container:login
docker pull blockmesh/blockmesh-pghero:latest
docker tag blockmesh/blockmesh-pghero:latest registry.heroku.com/blockmesh-pghero/web
docker push registry.heroku.com/blockmesh-pghero/web
heroku container:release web -a blockmesh-pghero
heroku restart -a blockmesh-pghero