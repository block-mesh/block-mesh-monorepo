#!/usr/bin/env bash
docker build --platform linux/arm64  -t base -f Dockerfile --load .
docker run --platform linux/arm64 -v ./:/code -v ./docker-target:/code/target -t base


docker run -it --platform linux/arm64 -v ./:/code -t base /bin/bash