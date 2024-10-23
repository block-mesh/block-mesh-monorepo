#!/usr/bin/env bash
set -x
export DOCKER_BUILDKIT=1
export ROOT="$(git rev-parse --show-toplevel)"
#####################################################################################################################################################
docker login
docker buildx create --name buildx --use
set -eo pipefail
#####################################################################################################################################################
#docker buildx build --platform linux/amd64 -t blockmesh-alpine-base -f "${ROOT}/docker/base/blockmesh-alpine-base.Dockerfile" --load .
#docker tag blockmesh-alpine-base blockmesh/blockmesh-alpine-base:latest-amd64
#docker push blockmesh/blockmesh-alpine-base:latest-amd64
#####################################################################################################################################################
#docker buildx build --platform linux/arm64 -t blockmesh-alpine-base -f "${ROOT}/docker/base/blockmesh-alpine-base.Dockerfile" --load .
#docker tag blockmesh-alpine-base blockmesh/blockmesh-alpine-base:latest-arm64
#docker push blockmesh/blockmesh-alpine-base:latest-arm64
#####################################################################################################################################################
docker buildx build --platform linux/amd64,linux/arm64 -t blockmesh/blockmesh-alpine-base -f "${ROOT}/docker/base/blockmesh-alpine-base.Dockerfile" --push .
#####################################################################################################################################################
docker pull blockmesh/blockmesh-alpine-base
#docker pull blockmesh/blockmesh-alpine-base:latest-amd64
#docker pull blockmesh/blockmesh-alpine-base:latest-arm64
