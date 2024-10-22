#!/usr/bin/env bash
set -x
export DOCKER_BUILDKIT=1
export ROOT="$(git rev-parse --show-toplevel)"
#####################################################################################################################################################
docker login
docker buildx create --name buildx --use
set -eo pipefail
#####################################################################################################################################################
docker buildx build --platform linux/amd64,linux/arm64 -t blockmesh/blockmesh-ubuntu-base -f "${ROOT}/docker/blockmesh-ubuntu-base.Dockerfile" --push .
#####################################################################################################################################################
docker buildx build --platform linux/amd64 -t blockmesh-ubuntu-base -f "${ROOT}/docker/blockmesh-ubuntu-base.Dockerfile" --load .
docker tag blockmesh-ubuntu-base blockmesh/blockmesh-ubuntu-base:latest-amd64
docker push blockmesh/blockmesh-ubuntu-base:latest-amd64
#####################################################################################################################################################
docker buildx build --platform linux/arm64 -t blockmesh-ubuntu-base -f "${ROOT}/docker/blockmesh-ubuntu-base.Dockerfile" --load .
docker tag blockmesh-ubuntu-base blockmesh/blockmesh-ubuntu-base:latest-arm64
docker push blockmesh/blockmesh-ubuntu-base:latest-arm64