#!/usr/bin/env bash
set -x
set -eo pipefail

heroku container:login
docker pull --platform linux/amd64 blockmesh/ids:latest
docker tag blockmesh/ids:latest registry.heroku.com/ids/web
docker push registry.heroku.com/ids/web
#heroku container:push web -a feature-flags-server
heroku container:release web -a ids
heroku restart -a ids