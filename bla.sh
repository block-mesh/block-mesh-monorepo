#!/usr/bin/env bash
docker build --platform linux/amd64 -t base -f Dockerfile --load .
docker run   --platform linux/amd64 -v ./docker-target:/code/target -t base
