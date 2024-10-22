#!/usr/bin/env bash
set -x
set -eo pipefail
export DOCKER_BUILDKIT=1
export ROOT="$(git rev-parse --show-toplevel)"
docker login
docker buildx create --name buildx --use || echo 1
#####################################################################################################################################################
docker buildx build --platform linux/amd64,linux/arm64 -t blockmesh/blockmesh-ubuntu-base -f "${ROOT}/docker/base/blockmesh-ubuntu-base.Dockerfile" --push .
#####################################################################################################################################################
docker buildx build --platform linux/amd64,linux/arm64 -t blockmesh-extra-rust -f "${ROOT}/docker/base/blockmesh-extra-rust.Dockerfile" --push .
#####################################################################################################################################################
docker buildx build --platform linux/amd64,linux/arm64 -t blockmesh-repo -f "${ROOT}/docker/base/blockmesh-repo.Dockerfile" --push .
#####################################################################################################################################################
#docker buildx build --platform linux/amd64 -t blockmesh-ubuntu-base -f "${ROOT}/docker/blockmesh-ubuntu-base.Dockerfile" --load .
#docker tag blockmesh-ubuntu-base blockmesh/blockmesh-ubuntu-base:latest-amd64
#docker push blockmesh/blockmesh-ubuntu-base:latest-amd64
#####################################################################################################################################################
#docker buildx build --platform linux/arm64 -t blockmesh-ubuntu-base -f "${ROOT}/docker/blockmesh-ubuntu-base.Dockerfile" --load .
#docker tag blockmesh-ubuntu-base blockmesh/blockmesh-ubuntu-base:latest-arm64
#docker push blockmesh/blockmesh-ubuntu-base:latest-arm64