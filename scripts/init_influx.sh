#!/usr/bin/env bash
set -x
docker network create blockmesh_network
set -eo pipefail
export _PWD="$(pwd)"
export ROOT="$(git rev-parse --show-toplevel)"
source "${ROOT}/scripts/setup.sh"
cd "${ROOT}/libs/block-mesh-manager" || exit

function remove_influx() {
   DOCKERS="$(docker ps -a -q --filter influxdb:2 --format="{{.ID}}")"
    if [ -n "$DOCKERS" ]
    then
      ensure docker rm --force --volumes $DOCKERS
    fi
}

remove_influx

#docker run \
#--network=blockmesh_network \
#-e INFLUXDB_DB=block-mesh \
#-e INFLUXDB_ADMIN_USER=influxdb \
#-e INFLUXDB_ADMIN_PASSWORD=password \
#-e INFLUXDB_USER=blockmesh \
#-e INFLUXDB_USER_PASSWORD=password \
#-p 8086:8086 \
#-v /tmp/testdata/influx:/root/.influxdb2 \
#-d influxdb:2.0


docker run -d -p 8086:8086 \
-v "$PWD/data:/var/lib/influxdb2" \
-v "$PWD/config:/etc/influxdb2" \
-e DOCKER_INFLUXDB_INIT_MODE=setup \
-e DOCKER_INFLUXDB_INIT_USERNAME=block-mesh \
-e DOCKER_INFLUXDB_INIT_PASSWORD=password \
-e DOCKER_INFLUXDB_INIT_ORG=blockmesh \
-e DOCKER_INFLUXDB_INIT_BUCKET=bucket \
-e DOCKER_INFLUXDB_INIT_RETENTION=1w \
-e DOCKER_INFLUXDB_INIT_ADMIN_TOKEN=my-super-secret-auth-token \
influxdb:2
