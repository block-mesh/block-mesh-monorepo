#!/usr/bin/env bash
set -x
docker network create blockmesh_network
set -eo pipefail
export _PWD="$(pwd)"
export ROOT="$(git rev-parse --show-toplevel)"
source "${ROOT}/scripts/setup.sh"
cd "${ROOT}/libs/block-mesh-manager" || exit
if ! command -v psql &> /dev/null
then
    >&2 echo "Error: `psql` is not installed."
    cd "${_PWD}" || exit
    exit 1
fi

if ! command -v docker &> /dev/null
then
    >&2 echo "Error: `docker` is not installed."
    cd "${_PWD}" || exit
    exit 1
fi

if ! command -v sqlx &> /dev/null
then
    >&2 echo "Error: `sqlx` is not installed."
    >&2 echo "Use the following command to install it:"
    >&2 echo "    cargo install sqlx-cli --no-default-features --features postgres"
    cd "${_PWD}" || exit
    exit 1
fi

DB_USER="${POSTGRES_USER:=postgres}"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
#DB_NAME="${POSTGRES_DB:=block-mesh}"
#DB_PORT="${POSTGRES_PORT:=5559}"

function remove_pg() {
   DOCKERS="$(docker ps -a -q --filter ancestor=postgres:15.3-alpine3.18 --format="{{.ID}}")"
    if [ -n "$DOCKERS" ]
    then
      ensure docker rm --force --volumes $DOCKERS
    fi
}

function start_db() {
  DB_NAME=$1
  DB_PORT=$2
    ensure docker run \
     --network=blockmesh_network \
      -e POSTGRES_USER=postgres \
      -e POSTGRES_PASSWORD=password \
      -e POSTGRES_DB=${DB_NAME} \
      -e POSTGRES_PORT=${DB_PORT} \
      -p "${DB_PORT}:${DB_PORT}" \
      -d postgres:15.3-alpine3.18 \
      postgres -p ${DB_PORT} -N 1000
      export PGPASSWORD=password
      until psql -h "localhost" -p "${DB_PORT}" -U "${DB_USER}" -d postgres -c '\q'
      do
        >&2 echo "Postgres is still unavailable - sleeping"
        sleep 1
      done
}

if [ "${SKIP_DOCKER}" != "yes" ]
then
  remove_pg
  start_db block-mesh 5559
  start_db tg-bot 5551
  start_db data-sink 5552
  start_db emails 5553

  DOCKERS="$(docker ps -a -q --filter ancestor=redis:alpine3.20 --format="{{.ID}}")"
  if [ -n "$DOCKERS" ]
  then
    ensure docker rm --force --volumes $DOCKERS
  fi
  ensure docker run \
    --network=blockmesh_network \
    --name redis -p 6379:6379 -d redis:alpine3.20
  export REDIS_URL="redis://127.0.0.1:6379"
fi

function migrate() {
  sqlx migrate run --source migrations --ignore-missing
  cargo sqlx prepare -- --all-targets --all-features
}

>&2 echo "Postgres is up and running on port ${DB_PORT}!"
set -x

export DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@localhost:5559/block-mesh"
echo "create DB"
ensure sqlx database create
echo "migrate DB block-mesh-manager"
ensure migrate
cd "${ROOT}/libs/block-mesh-manager-worker" || exit
echo "migrate DB block-mesh-manager-worker"
ensure migrate
cd "${ROOT}/libs/block-mesh-manager-api" || exit
echo "migrate DB block-mesh-manager-api"
ensure migrate
cd "${ROOT}/libs/block-mesh-manager-ws" || exit
echo "migrate DB block-mesh-manager-ws"
ensure migrate
cd "${ROOT}/libs/feature-flags-server" || exit
echo "migrate DB feature-flags-server"
ensure migrate
cd "${ROOT}/libs/block-mesh-manager-database-domain" || exit
echo "migrate DB block-mesh-manager-database-domain"
ensure migrate
cd "${ROOT}/libs/database-utils" || exit
echo "migrate DB database-utils"
ensure migrate
export DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@localhost:5551/tg-bot"
echo "create DB"
ensure sqlx database create
cd "${ROOT}/libs/tg-privacy-bot" || exit
echo "migrate DB tg-privacy-bot"
ensure migrate
export DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@localhost:5552/data-sink"
echo "create DB"
ensure sqlx database create
cd "${ROOT}/libs/data-sink" || exit
echo "migrate DB data-sink"
ensure migrate
export DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@localhost:5553/emails"
echo "create DB"
ensure sqlx database create
cd "${ROOT}/libs/emails" || exit
echo "migrate DB emails"
ensure migrate
>&2 echo "Postgres has been migrated, ready to go!"
cd "${_PWD}"
