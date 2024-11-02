#!/usr/bin/env bash
set -x
set -eo pipefail

heroku container:login
docker pull blockmesh/feature-flags-server:latest
docker tag blockmesh/feature-flags-server:latest registry.heroku.com/feature-flags-server/web
docker push registry.heroku.com/feature-flags-server/web
#heroku container:push web -a feature-flags-server
heroku container:release web -a feature-flags-server
heroku restart -a feature-flags-server