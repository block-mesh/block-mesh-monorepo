#!/usr/bin/env bash
docker buildx build --platform linux/amd64 -t blockmesh-ubuntu-base -f blockmesh-ubuntu-base.Dockerfile --load .
docker login
docker tag blockmesh-ubuntu-base blockmesh/blockmesh-ubuntu-base:latest-amd64
docker push blockmesh/blockmesh-ubuntu-base:latest-amd64