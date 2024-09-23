#!/usr/bin/env bash
docker login

docker buildx build --platform linux/amd64 -t blockmesh-ubuntu-base -f blockmesh-ubuntu-base.Dockerfile --load .
docker tag blockmesh-ubuntu-base blockmesh/blockmesh-ubuntu-base:latest-amd64
docker push blockmesh/blockmesh-ubuntu-base:latest-amd64

docker buildx build --platform linux/arm64 -t blockmesh-ubuntu-base -f blockmesh-ubuntu-base.Dockerfile --load .
docker tag blockmesh-ubuntu-base blockmesh/blockmesh-ubuntu-base:latest-arm64
docker push blockmesh/blockmesh-ubuntu-base:latest-arm64