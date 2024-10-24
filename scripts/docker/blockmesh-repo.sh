#!/usr/bin/env bash
set -x
export DOCKER_BUILDKIT=1
export ROOT="$(git rev-parse --show-toplevel)"
#####################################################################################################################################################
docker login
docker buildx create --name buildx --use
set -eo pipefail
#####################################################################################################################################################
docker pull blockmesh/blockmesh-extra-rust
#####################################################################################################################################################
docker buildx build --platform linux/amd64,linux/arm64 -t blockmesh/blockmesh-repo -f "${ROOT}/docker/base/blockmesh-repo.Dockerfile" --push .
docker pull blockmesh/blockmesh-repo