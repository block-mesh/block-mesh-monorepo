#!/usr/bin/env bash
set -x
set -eo pipefail

heroku container:login
docker pull --platform linux/amd64 blockmesh/data-sink:latest
docker tag blockmesh/data-sink:latest registry.heroku.com/data-sink/web
docker push registry.heroku.com/data-sink/web
#heroku container:push web -a feature-flags-server
heroku container:release web -a data-sink
heroku restart -a data-sink