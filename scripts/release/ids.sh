#!/usr/bin/env bash
set -x
set -eo pipefail

heroku container:login
docker pull --platform linux/amd64 blockmesh/ids:latest
docker tag blockmesh/ids:latest registry.heroku.com/ids-blockmesh/web
docker push registry.heroku.com/ids-blockmesh/web
#heroku container:push web -a feature-flags-server
heroku container:release web -a ids-blockmesh
heroku restart -a ids-blockmesh