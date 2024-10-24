#!/usr/bin/env bash
set -x
export DOCKER_BUILDKIT=1
export ROOT="$(git rev-parse --show-toplevel)"
#####################################################################################################################################################
docker login
docker buildx create --name buildx --use
set -eo pipefail
#####################################################################################################################################################
docker pull blockmesh/blockmesh-ubuntu-base
#####################################################################################################################################################
#docker buildx build --platform linux/amd64 -t blockmesh-extra-rust -f "${ROOT}/docker/base/blockmesh-extra-rust.Dockerfile" --load .
#docker tag blockmesh-extra-rust blockmesh/blockmesh-extra-rust:latest-amd64
#docker push blockmesh/blockmesh-extra-rust:latest-amd64
#####################################################################################################################################################
#docker buildx build --platform linux/arm64 -t blockmesh-extra-rust -f "${ROOT}/docker/base/blockmesh-extra-rust.Dockerfile" --load .
#docker tag blockmesh-extra-rust blockmesh/blockmesh-extra-rust:latest-arm64
#docker push blockmesh/blockmesh-extra-rust:latest-arm64
#####################################################################################################################################################
docker buildx build --platform linux/amd64,linux/arm64 -t blockmesh/blockmesh-extra-rust -f "${ROOT}/docker/base/blockmesh-extra-rust.Dockerfile" --push .
