#!/usr/bin/env bash
set -x
set -eo pipefail

heroku container:login
docker pull --platform linux/amd64 blockmesh/tg-privacy-bot:latest
docker tag blockmesh/tg-privacy-bot:latest registry.heroku.com/tg-privacy-llm-bot/web
docker push registry.heroku.com/tg-privacy-llm-bot/web
#heroku container:push web -a tg-privacy-llm-bot
heroku container:release web -a tg-privacy-llm-bot
heroku restart -a tg-privacy-llm-bot