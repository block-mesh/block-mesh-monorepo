#!/usr/bin/env bash
set -x
set -eo pipefail

heroku container:login
docker pull --platform linux/amd64 blockmesh/logs-drain:latest
docker tag blockmesh/logs-drain:latest registry.heroku.com/logs-drain/web
docker push registry.heroku.com/logs-drain/web
#heroku container:push web -a feature-flags-server
heroku container:release web -a logs-drain
heroku restart -a logs-drain