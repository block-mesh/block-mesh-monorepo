#!/usr/bin/env bash
docker build --platform linux/arm64 -t base -f Dockerfile --load .
docker run   --platform linux/arm64 -v ./docker-target:/code/target -t base
