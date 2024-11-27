#!/usr/bin/env bash
set -x
set -eo pipefail

heroku container:login
docker pull --platform linux/amd64 blockmesh/emails:latest
docker tag blockmesh/emails:latest registry.heroku.com/emails/web
docker push registry.heroku.com/emails/web
#heroku container:push web -a feature-flags-server
heroku container:release web -a emails
heroku restart -a emails