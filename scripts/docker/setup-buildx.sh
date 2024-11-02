#!/usr/bin/env bash

# https://docs.docker.com/build/building/multi-platform/#qemu
set -x
export DOCKER_BUILDKIT=1
export ROOT="$(git rev-parse --show-toplevel)"
#####################################################################################################################################################
docker login
docker buildx create --name mybuilder --use
docker buildx create --append --name mybuilder node-amd64
docker buildx create --append --name mybuilder node-arm64
docker run --rm --privileged multiarch/qemu-user-static --reset -p yes
docker buildx inspect --bootstrap
docker buildx ls
