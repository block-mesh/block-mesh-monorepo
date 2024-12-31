#!/usr/bin/env bash
set -x
export APP_ENVIRONMENT="local"
export SENTRY_LAYER="true"
export SENTRY_SAMPLE_RATE="1.0"
export _PWD="$(pwd)"
export ROOT="$(git rev-parse --show-toplevel)"
export BULK_UPTIME_BONUS=100
source "${ROOT}/scripts/setup.sh"
set +x
export AGG_SIZE=1
export ADD_TO_AGG_SIZE=1
export USERS_IP_AGG_SIZE=1
export AGG_AGG_SIZE=1
export SET_TO_AGG_SIZE=1
export ANALYTICS_AGG_SIZE=1
export DAILY_STATS_AGG_SIZE=1
export CREATE_DAILY_STATS_AGG_SIZE=1
export REF_BONUS_CRON_ENABLE="true"
source "${ROOT}/scripts/setup.sh"
export DATABASE_URL="postgres://postgres:password@localhost:5559/block-mesh"
export WRITE_DATABASE_URL="postgres://postgres:password@localhost:5559/block-mesh"
export FOLLOWER_DATABASE_URL="postgres://postgres:password@localhost:5559/block-mesh"
export CHANNEL_DATABASE_URL="postgres://postgres:password@localhost:5559/block-mesh"
export UNLIMITED_DATABASE_URL="postgres://postgres:password@localhost:5559/block-mesh"
export REF_BONUS_BG_CRON_ENABLE=true
export REDIS_URL="redis://127.0.0.1:6379"
if [ -f "${ROOT}/.env" ] ; then
  source "${ROOT}/.env"
fi
#ensure "${ROOT}/scripts/init_db.sh"
cargo watch --watch libs --shell "cargo run -p block-mesh-manager-worker | bunyan"
#cargo run -p block-mesh-manager-worker | bunyan